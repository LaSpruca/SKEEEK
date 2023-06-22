use crate::util::{get_tick_length, set_interval_with_cancel, tweened};
use leptos::*;
use log::*;
use std::{panic, time::Duration};

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    info!("Starting application");
    let (head_position, set_head_position) = create_signal(cx, Position { x: 0, y: 0 });
    let (direction, set_direction) = create_signal(cx, Direction::None);

    window_event_listener(ev::keydown, move |kb_event| {
        match kb_event.key().to_ascii_lowercase().as_str() {
            "w" | "arrowup" => set_direction(Direction::Up),
            "a" | "arrowleft" => set_direction(Direction::Left),
            "s" | "arrowdown" => set_direction(Direction::Down),
            "d" | "arrowright" => set_direction(Direction::Right),
            _ => {
                debug!("Code: {}", kb_event.key().to_ascii_lowercase());
            }
        }
    });

    set_interval_with_cancel(
        move |cancel| match direction() {
            Direction::Up => set_head_position.update(|mut pos| {
                pos.y -= 1;

                if pos.y < 0 {
                    pos.y = 9;
                }
            }),
            Direction::Down => set_head_position.update(|mut pos| {
                pos.y += 1;

                if pos.y > 9 {
                    pos.y = 0;
                }
            }),
            Direction::Left => set_head_position.update(|mut pos| {
                pos.x -= 1;

                if pos.x < 0 {
                    pos.x = 9;
                }
            }),
            Direction::Right => set_head_position.update(|mut pos| {
                pos.x += 1;

                if pos.x > 9 {
                    pos.x = 0;
                }
            }),

            Direction::None => {}
        },
        get_tick_length(),
    );

    view! {
        cx,
        <div class={"game"}>
            <Head head=head_position />
        </div>
    }
}

#[component]
fn Head<T>(cx: Scope, head: T) -> impl IntoView
where
    T: Fn() -> Position + 'static + Copy,
{
    let duration = get_tick_length();

    let x = tweened(cx, move || (head().x * 25) as f64, duration);
    let y = tweened(cx, move || (head().y * 25) as f64, duration.clone());

    view! {
        cx,
        <div class={"head"} style={move || format!("top: {y}px; left: {x}px", x =  x(), y = y()) }/>
    }
}

#[component]
fn Tail<T>(cx: Scope, head: T) -> impl IntoView
where
    T: Fn() -> Position + 'static + Copy,
{
    let duration = get_tick_length();

    let x = tweened(cx, move || (head().x * 25) as f64, duration);
    let y = tweened(cx, move || (head().y * 25) as f64, duration.clone());

    view! {
        cx,
        <div class={"tail"} style={move || format!("top: {y}px; left: {x}px", x =  x(), y = y()) }/>
    }
}
