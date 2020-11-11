use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Update {
    snake: Vec<Point>,
    fruit: Point,
    failed: bool,
    got_fruit: bool,
}

impl Update {
    pub fn new(snake: Vec<Point>, fruit: Point, failed: bool, got_fruit: bool) -> Self {
        Update {
            snake,
            fruit,
            failed,
            got_fruit,
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    #[allow(dead_code)]
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn translate(&self, x: i32, y: i32) -> Point {
        Point {
            x: self.x + x,
            y: self.y + y,
        }
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
pub fn update_wasm(snake: String, fruit: String, size: i32, direcrion: String) -> String {
    update(snake, fruit, size, direcrion)
}

pub fn update(snake: String, fruit: String, size: i32, direcrion: String) -> String {
    let dir = direcrion.as_str();

    let mut snake: Vec<Point> = serde_json::from_str(snake.as_str()).unwrap();

    let mut fruit: Point = serde_json::from_str(fruit.as_str()).unwrap();

    let new_head = if dir == "left" {
        snake[0].translate(0, -1)
    } else if dir == "right" {
        snake[0].translate(0, 1)
    } else if dir == "up" {
        snake[0].translate(-1, 0)
    } else if dir == "down" {
        snake[0].translate(1, 0)
    } else {
        snake[0]
    };

    let failed = if new_head.x > size - 1
        || new_head.y > size - 1
        || new_head.x < 0
        || new_head.y < 0
        || (snake.contains(&new_head) && dir != "")
    {
        true
    } else {
        snake.insert(0, new_head);
        false
    };

    let got_fruit = if new_head == fruit {
        while snake.contains(&fruit) {
            fruit = Point {
                x: ranint(size),
                y: ranint(size),
            };
        }
        true
    } else {
        snake.pop();
        false
    };

    let result = Update {
        snake,
        fruit,
        failed,
        got_fruit,
    };
    serde_json::to_string(&result).unwrap()
}

#[wasm_bindgen]
pub fn ranint(max: i32) -> i32 {
    (random() * max as f64).floor() as i32
}
