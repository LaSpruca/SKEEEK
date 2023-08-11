use leptos::*;
use log::info;

pub const SIZE: i32 = 20;
pub const BLOCK_SIZE: i32 = 25;

#[derive(Clone, Copy)]
pub struct Tail {
    read: ReadSignal<Vec<(ReadSignal<Position>, WriteSignal<Position>)>>,
    write: WriteSignal<Vec<(ReadSignal<Position>, WriteSignal<Position>)>>,
}

impl Tail {
    pub fn new(cx: Scope) -> Self {
        let (read, write) = create_signal(cx, vec![]);
        Self { read, write }
    }

    pub fn contains(&self, position: Position) -> bool {
        self.read
            .with_untracked(|vec: &Vec<(ReadSignal<Position>, WriteSignal<Position>)>| {
                vec.iter()
                    .map(|(item, _)| item())
                    .any(|item| item == position)
            })
    }

    pub fn is_empty(&self) -> bool {
        self.read.with(|vec| vec.is_empty())
    }

    pub fn clear(&self) {
        self.write.update(|vec| vec.clear());
    }

    pub fn push(&self, cx: Scope, position: Position) {
        self.write.update(|vec| {
            vec.push(create_signal(cx, position));
        })
    }

    pub fn update(&self, position: Position) {
        info!("Are we even logging?");
        self.read.with_untracked(|items| {
            items
                .iter()
                .zip(
                    [position]
                        .into_iter()
                        .chain(items.iter().map(|(x, _)| x()))
                        .collect::<Vec<_>>(),
                )
                .enumerate()
                .for_each(|(i, ((curr, write), value))| {
                    info!("Setting {i} from {:?} to {value:?}", curr.get_untracked());

                    write(value);
                });
        });
    }

    pub fn get(&self) -> Vec<(usize, ReadSignal<Position>)> {
        self.read
            .with(|vec| vec.iter().map(|(item, _)| *item).enumerate().collect())
    }
}

impl FnOnce<()> for Tail {
    type Output = Vec<(usize, ReadSignal<Position>)>;

    extern "rust-call" fn call_once(self, _args: ()) -> Vec<(usize, ReadSignal<Position>)> {
        self.get()
    }
}

impl FnMut<()> for Tail {
    extern "rust-call" fn call_mut(&mut self, _args: ()) -> Vec<(usize, ReadSignal<Position>)> {
        self.get()
    }
}

impl Fn<()> for Tail {
    extern "rust-call" fn call(&self, _args: ()) -> Vec<(usize, ReadSignal<Position>)> {
        self.get()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Play,
    Loose,
    Pause,
    Start,
}

impl GameState {
    pub fn class(&self) -> &str {
        match self {
            GameState::Play => "",
            GameState::Loose => "loose",
            GameState::Pause => "pause",
            GameState::Start => "start",
        }
    }
}
