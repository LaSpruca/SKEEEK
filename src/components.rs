use crate::{
    data::*,
    util::{get_tick_length, rand_range, tweened},
};
use leptos::*;
use log::debug;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Create game state
    let (head_position, set_head_position) = create_signal(cx, Position { x: 0, y: 0 });
    let (direction, set_direction) = create_signal(cx, Direction::Down);
    let (game_state, set_game_state) = create_signal(cx, GameState::Start);
    let (fruit_position, set_fruit_position) = create_signal(cx, Position { x: 0, y: 0 });
    let tail = Tail::new(cx);

    let new_random_fruit = move || {
        let mut new_position;
        while {
            let x = rand_range(0.0, SIZE as f64).floor() as i32;
            let y = rand_range(0.0, SIZE as f64).floor() as i32;

            new_position = Position { x, y };

            log::debug!("New fruit position: {:?}", new_position);

            head_position() == new_position || tail.contains(new_position)
        } {}

        set_fruit_position(new_position);
    };

    new_random_fruit();

    let interval_fn = move || {
        let old_position = head_position();
        match direction() {
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
        }

        if tail.contains(head_position()) {
            set_game_state(GameState::Loose);
        }

        if head_position() == fruit_position() {
            tail.push(cx, old_position);
            new_random_fruit();
        } else if !tail.is_empty() {
            tail.update(old_position);
        }
    };

    // Setup keyboard event listener
    window_event_listener(ev::keydown, move |kb_event| {
        debug!("Keyboard is keyboarding {}", kb_event.key());
        match (game_state(), kb_event.key().to_ascii_lowercase().as_str()) {
            (GameState::Play, "w" | "arrowup") => set_direction(Direction::Up),
            (GameState::Play, "a" | "arrowleft") => set_direction(Direction::Left),
            (GameState::Play, "s" | "arrowdown") => set_direction(Direction::Down),
            (GameState::Play, "d" | "arrowright") => set_direction(Direction::Right),

            (GameState::Start, "w" | "arrowup") => {
                set_game_state(GameState::Play);
                set_direction(Direction::Up)
            }
            (GameState::Start, "a" | "arrowleft") => {
                set_game_state(GameState::Play);
                set_direction(Direction::Left)
            }
            (GameState::Start, "s" | "arrowdown") => {
                set_game_state(GameState::Play);
                set_direction(Direction::Down)
            }
            (GameState::Start, "d" | "arrowright") => {
                set_game_state(GameState::Play);
                set_direction(Direction::Right)
            }

            (GameState::Play, " ") => set_game_state(GameState::Pause),
            (GameState::Loose, " ") => {
                set_game_state(GameState::Start);
                set_head_position(Position { x: 0, y: 0 });
                new_random_fruit();
                tail.clear();
            }
            (GameState::Pause, " ") => set_game_state(GameState::Play),

            _ => {}
        }
    });

    // Set the interval for the game loop
    create_effect(cx, move |handle: Option<Option<_>>| match game_state() {
        GameState::Play => {
            debug!("Creating event loop");
            if let Some(None) = handle {
                return Some(
                    set_interval_with_handle(interval_fn, get_tick_length())
                        .expect("Set interval for game loop failed"),
                );
            }

            None
        }
        GameState::Loose | GameState::Pause | GameState::Start => {
            if let Some(Some(handle)) = handle {
                handle.clear();
            }

            None
        }
    });

    // --------------------
    // RENDERS HERE
    // --------------------
    view! {
        cx,
        <div class={move || format!("game {}", game_state().class())} style={format!("width: {0}px; height: {0}px; --size: {1}px;", SIZE * BLOCK_SIZE, BLOCK_SIZE)}>
            <Screens />
            <Head head=head_position />
            <Fruit fruit=fruit_position />
            <For each=tail key={|(index, _)| *index} view={|cx, (_, tile)| view! {cx, <Tail tile/>}}/>
        </div>
    }
}

#[component]
fn Head<T>(cx: Scope, head: T) -> impl IntoView
where
    T: Fn() -> Position + 'static + Copy,
{
    let duration = get_tick_length();

    let x = tweened(cx, move || (head().x * BLOCK_SIZE) as f64, duration);
    let y = tweened(cx, move || (head().y * BLOCK_SIZE) as f64, duration);

    view! {
        cx,
        <div class="tile head" style={move || format!("top: {y}px; left: {x}px", x =  x(), y = y()) }/>
    }
}

#[component]
fn Tail<T>(cx: Scope, tile: T) -> impl IntoView
where
    T: Fn() -> Position + 'static + Copy,
{
    let duration = get_tick_length();

    let x = tweened(cx, move || (tile().x * BLOCK_SIZE) as f64, duration);
    let y: ReadSignal<f64> = tweened(cx, move || (tile().y * BLOCK_SIZE) as f64, duration);

    view! {
        cx,
        <div class="tile tail" style={move || format!("top: {y}px; left: {x}px", x =  x(), y = y()) }/>
    }
}

#[component]
fn Fruit<T>(cx: Scope, fruit: T) -> impl IntoView
where
    T: Fn() -> Position + 'static + Copy,
{
    let x = move || fruit().x * BLOCK_SIZE;
    let y = move || fruit().y * BLOCK_SIZE;

    view! {
        cx,
        <div class="tile fruit" style={move || format!("top: {y}px; left: {x}px", x =  x(), y = y()) }/>
    }
}

#[component]
fn Screens(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <>
            <div class="screen start">
                <h1>"Snake"</h1>
                <p>"Press W A S D or the arrow keys to play"</p>
            </div>

            <div class="screen pause">
                <h1>"Game Paused"</h1>
                <p>"Press space to continue"</p>
            </div>

            <div class="screen loose">
                <h1>"You lost"</h1>
                <p>"Press space to reset"</p>
            </div>
        </>
    }
}
