use crate::util::{get_tick_length, tweened};
use leptos::*;
use log::*;

const SIZE: i32 = 20;
const BLOCK_SIZE: i32 = 25;

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
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum GameState {
    Play,
    Loose,
    Pause,
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    info!("Starting application");
    let (head_position, set_head_position) = create_signal(cx, Position { x: 0, y: 0 });
    let (direction, set_direction) = create_signal(cx, Direction::Down);
    let (game_state, set_game_state) = create_signal(cx, GameState::Pause);
    let set_direction = move |direction| {
        if game_state() == GameState::Play {
            set_direction(direction);
        } else if game_state() == GameState::Pause {
            set_game_state(GameState::Play);
            set_direction(direction);
        }
    };

    window_event_listener(ev::keydown, move |kb_event| {
        match kb_event.key().to_ascii_lowercase().as_str() {
            "w" | "arrowup" => set_direction(Direction::Up),
            "a" | "arrowleft" => set_direction(Direction::Left),
            "s" | "arrowdown" => set_direction(Direction::Down),
            "d" | "arrowright" => set_direction(Direction::Right),
            "escape" => set_game_state(GameState::Pause),
            _ => {
                debug!("Code: {}", kb_event.key().to_ascii_lowercase());
            }
        }
    });

    let interval_fn = move || match direction() {
        Direction::Up => set_head_position.update(|mut pos| {
            pos.y -= 1;

            if pos.y < 0 {
                pos.y += 1;
                set_game_state(GameState::Loose);
            }
        }),
        Direction::Down => set_head_position.update(|mut pos| {
            pos.y += 1;

            if pos.y > SIZE - 1 {
                pos.y -= 1;
                set_game_state(GameState::Loose);
            }
        }),
        Direction::Left => set_head_position.update(|mut pos| {
            pos.x -= 1;

            if pos.x < 0 {
                pos.x += 1;
                set_game_state(GameState::Loose);
            }
        }),
        Direction::Right => set_head_position.update(|mut pos| {
            pos.x += 1;

            if pos.x > SIZE - 1 {
                pos.x -= 1;
                set_game_state(GameState::Loose);
            }
        }),
    };

    create_effect(cx, move |handle: Option<Option<_>>| match game_state() {
        GameState::Play => {
            if let Some(None) = handle {
                Some(
                    set_interval_with_handle(interval_fn, get_tick_length())
                        .expect("Set interval for game loop failed"),
                )
            } else {
                None
            }
        }
        GameState::Loose | GameState::Pause => {
            if let Some(Some(handle)) = handle {
                handle.clear();
            }

            None
        }
    });

    view! {
        cx,
        <div class={"game"} style={format!("width: {0}px; height: {0}px;", SIZE * BLOCK_SIZE)}>
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

    // let x = move || (head().x * 25);
    // let y = move || (head().y * 25);

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
