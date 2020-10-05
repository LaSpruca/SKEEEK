use snake_wasm::update;

fn main() {
    let snake = vec![snake_wasm::Point::new(10, 10), snake_wasm::Point::new(3, 3)];

    println!("{}", serde_json::to_string(&snake_wasm::Update::new(snake.clone(), false)).unwrap());

    update(serde_json::to_string(&snake).unwrap());
}