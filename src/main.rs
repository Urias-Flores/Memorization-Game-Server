#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::Rocket;
use rocket_contrib::json::Json;
use serde::Serialize;


// Define una estructura para representar la configuración del juego
#[derive(Serialize)]
struct GameConfig {
    num_images: usize,
    display_time: u32,
    response_time: u32,
}

// Define un estado compartido para la configuración del juego
#[get("/game_config/<difficulty>/<stage>/<level>")]
fn get_game_config(difficulty: String, stage: u32, level: u32) -> Json<GameConfig> {
    // ... (código de configuración del juego)
    let difficulty_str: &str = difficulty.as_str();
    let mut response_time = match difficulty_str {
        "easy" => 12,
        "medium" => 10,
        "hard" => 8,
        _ => panic!("Invalid difficulty"),
    };

    response_time -= (stage - 1);

    let num_images = match stage {
        1 | 2 => 4,
        3 | 4 => 6,
        5 | 6 => 8,
        _ => panic!("Invalid stage"),
    };

    let display_time = match level {
        1 | 2 => 3,
        3 | 4 => 2,
        5 => 1,
        _ => panic!("Invalid level"),
    };  // Tiempo de respuesta reducido por nivel

    // Construye y devuelve la configuración del juego como JSON
    let game_config = GameConfig {
        num_images,
        display_time,
        response_time,
    };

    Json(game_config)
}

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", routes![get_game_config])
}

fn main() {
    rocket().launch();
}
