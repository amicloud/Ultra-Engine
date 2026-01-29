mod settings;

use ultramayor_engine::Engine;
fn main() {
    println!("Welcome to the Game!");
    let mut engine = Engine::new();

    engine.run();
}
