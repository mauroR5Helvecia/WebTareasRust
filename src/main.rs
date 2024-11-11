#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

mod schema;
mod models;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenvy::dotenv;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_cors::{CorsOptions, AllowedOrigins};
use std::env;
use rocket::State;
use models::Tarea;

#[derive(Deserialize)]
struct TareaSinId {
    descripcion: String,
}

// Función para establecer la conexión con la base de datos
fn establecer_conexion() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL debe estar definido");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error al conectar con {}", database_url))
}

#[post("/tareas", format = "json", data = "<tarea>")]
fn crear_tarea(conn: &State<MysqlConnection>, tarea: Json<TareaSinId>) -> Json<Tarea> {
    use schema::tareas;

    let mut conn = conn.inner().clone(); // Clonamos la conexión para obtener un mutable
    let nueva_tarea = Tarea {
        id: 0, // Diesel ignorará este valor durante la inserción
        descripcion: tarea.descripcion.clone(),
    };

    diesel::insert_into(tareas::table)
        .values(&nueva_tarea)
        .execute(&mut conn)
        .expect("Error al insertar nueva tarea");

    Json(nueva_tarea)
}

#[delete("/tareas/<id>")]
fn eliminar_tarea(id: i32, conn: &State<MysqlConnection>) -> Json<bool> {
    use schema::tareas::dsl::*;

    let mut conn = conn.inner().clone(); // Clonamos la conexión para obtener un mutable
    let num_eliminadas = diesel::delete(tareas.filter(id.eq(id)))
        .execute(&mut conn)
        .expect("Error al eliminar tarea");

    Json(num_eliminadas > 0)
}


#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let conn = establecer_conexion();

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .to_cors()
        .expect("Error al configurar CORS");

    rocket::build()
        .manage(conn)
        .mount("/", routes![crear_tarea, listar_tareas, eliminar_tarea])
        .attach(cors)
}
