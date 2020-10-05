use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

macro_rules! console_log {
// Note that this is using the `log` function imported above during
// `bare_bones`
($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Update {
    snake: Vec<Point>,
    fruit: Point,
    failed: bool,
    got_fruit: bool
}

impl Update {
    pub fn new(snake: Vec<Point>, fruit: Point, failed: bool, got_fruit: bool) -> Self {
        Update { snake, fruit, failed, got_fruit }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    #[allow(dead_code)]
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn translate(&self, x: i32, y: i32) -> Point {
        Point { x: self.x + x, y: self.y + y }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        self.x != other.x || self.y != other.y
    }
}

#[wasm_bindgen]
pub fn generate_grid(size: u32) -> String {
    (0..size).
        map(
            |x| format!(r#"<div class="grid-row">{}</div>"#,
                        (0..size).
                            map(|y| format!(r#"<div class="grid-element" id={}></div>"#, (x * size) + y))
                            .collect::<Vec<String>>().join("")
            )).collect::<Vec<String>>().join("")
}

#[wasm_bindgen]
pub fn update_wasm(snake: String, fruit: String, size: i32, direcrion: String) -> String {
    update(snake, fruit, size, direcrion)
}

pub fn update(snake: String, fruit: String, size: i32, direcrion: String,) -> String {
    let dir = direcrion.as_str();

    let mut snake: Vec<Point> = serde_json::from_str(snake.as_str()).unwrap();

    let mut fruit: Point = serde_json::from_str(fruit.as_str()).unwrap();

    let new_head =
        if dir == "left" {
            snake[0].translate(0, -1)
        } else if dir == "right" {
            snake[0].translate(0, 1)
        } else if dir == "up" {
            snake[0].translate( -1, 0)
        } else if dir == "down" {
            snake[0].translate( 1, 0)
        } else {
            snake[0]
        };

    let mut failed = if new_head.x > size - 1 || new_head.y > size - 1
        || new_head.x < 0 || new_head.y < 0
        || (snake.contains(&new_head) && dir != "") {
        true
    } else {
        snake.insert(0, new_head);
        false
    };

    let got_fruit = if new_head == fruit {
        while snake.contains(&fruit) {
            fruit = Point { x: ranint(size), y: ranint(size) };
        }
        true
    } else {
        snake.pop();
        false
    };

    let result = Update { snake, fruit, failed, got_fruit };
    serde_json::to_string(&result).unwrap()
}

#[wasm_bindgen]
pub fn ranint(max: i32) -> i32 {
    (random() * max as f64).floor() as i32
}
