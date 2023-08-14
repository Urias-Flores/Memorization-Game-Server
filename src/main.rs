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
    if(difficulty.to_string() == "easy".to_string()){
        let decrement = 0.50;
        
    }
    if(difficulty.to_string() == "medium".to_string()){

    }
    if(difficulty.to_string() == "hard".to_string()){

    }
    let num_images = (stage * 5) as usize;  // Número de imágenes basado en el stage
    let display_time = 5000 - (level * 500);  // Tiempo de visualización reducido por nivel
    let response_time = 3000 - (level * 300);  // Tiempo de respuesta reducido por nivel

    // Construye y devuelve la configuración del juego como JSON
    let game_config = GameConfig {
        num_images,
        display_time,
        response_time,
    };

    Json(game_config)
}

#[get("/")]
fn helloword() -> String {
    "Hello, world!".to_string()
}

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", routes![get_game_config])
        .mount("/", routes![helloword])
}

fn main() {
    rocket().launch();
}
