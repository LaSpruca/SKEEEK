use instant::Duration;
use instant::Instant;
use leptos::{
    leptos_dom::helpers::{location_pathname, IntervalHandle},
    *,
};
use std::fmt::Debug;
use std::{cell::Cell, rc::Rc};

pub fn get_tick_length() -> Duration {
    Duration::from_millis(
        (|| match location_pathname()?.as_str() {
            "/sped" => Some(100),
            "/ahhh" => Some(50),
            _ => None,
        })()
        .unwrap_or(150),
    )
}

pub fn set_interval_with_cancel(cb: impl Fn(Box<dyn Fn()>) + 'static, duration: Duration) {
    let handle: Rc<Cell<Option<IntervalHandle>>> = Rc::default();
    let intermediate = handle.clone();
    handle.set(
        set_interval_with_handle(
            move || {
                let intermediate = intermediate.clone();

                let cancel = move || {
                    if let Some(handle) = intermediate.get() {
                        handle.clear()
                    }
                };

                cb(Box::new(cancel))
            },
            duration,
        )
        .ok(),
    );
}

pub fn tweened<F>(cx: Scope, value: F, duration: Duration) -> ReadSignal<f64>
where
    F: Fn() -> f64 + 'static,
{
    let (f, set_f) = create_signal(cx, value());

    create_effect(cx, move |_| {
        let value = value();
        let f = f.get_untracked();
        let diff = value - f;

        if diff == 0.0 {
            return;
        }

        let start = Instant::now();
        let end = start + duration;

        set_interval_with_cancel(
            move |cancel| {
                let now: Instant = Instant::now();

                if now > end {
                    set_f(value);
                    cancel();
                } else {
                    let calculated =
                        f + diff * (now - start).as_millis() as f64 / duration.as_millis() as f64;

                    if diff > 0.0 {
                        set_f(std::cmp::min_by(calculated, value, |a, b| {
                            a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
                        }));
                    } else {
                        set_f(std::cmp::max_by(calculated, value, |a, b| {
                            a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
                        }));
                    }
                }
            },
            Duration::from_millis(17),
        );
    });

    f
}

#[allow(dead_code)]
pub fn create_debug<T, F>(cx: Scope, signal: F)
where
    T: Debug,
    F: Fn() -> T + 'static,
{
    create_effect(cx, move |_| {
        log::debug!("{:?}", signal());
    });
}
