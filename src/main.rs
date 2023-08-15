#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::Rocket;
use rocket_cors::CorsOptions;
use rocket_cors::AllowedOrigins;
use rocket_contrib::json::Json;
use serde::Serialize;
use rand::prelude::*;


// Define una estructura para representar la configuración del juego
#[derive(Serialize)]
struct GameConfig {
    images: Vec<u32>,
    display_time: u32,
    response_time: u32,
}

fn generate_random_numbers(count: usize) -> Vec<u32> {
    let mut rng = thread_rng(); // Crea un generador de números aleatorios
    let mut numbers: Vec<u32> = (1..=10).collect(); // Crea un arreglo de números del 1 al 10

    numbers.shuffle(&mut rng); // Mezcla los números aleatoriamente
    numbers.truncate(count); // Reduce el tamaño del arreglo al valor deseado

    numbers
}

// Define un estado compartido para la configuración del juego
#[get("/game_config/<difficulty>/<stage>/<level>")]
fn get_game_config(difficulty: String, stage: u32, level: u32) -> Json<GameConfig> {
    // ... (código de configuración del juego)
    let difficulty_str: &str = difficulty.as_str();
    let mut response_time = match difficulty_str {
        "Easy" | "easy" => 12,
        "Medium" | "medium" => 10,
        "Hard" | "hard" => 8,
        _ => panic!("Invalid difficulty"),
    };

    response_time -= stage - 1;

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

    let images = generate_random_numbers(num_images);

    // Construye y devuelve la configuración del juego como JSON
    let game_config = GameConfig {
        images,
        display_time,
        response_time,
    };

    Json(game_config)
}

fn rocket() -> Rocket {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::some_exact(&["http://localhost:5173"]))
        .allow_credentials(true)
        .to_cors()
        .expect("Error while building CORS");

    rocket::ignite()
        .attach(cors)
        .mount("/", routes![get_game_config])
}

fn main() {
    rocket().launch();
}