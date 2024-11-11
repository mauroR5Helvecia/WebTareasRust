use mysql::{Pool, Opts};
use mysql::prelude::*;
use mysql::params;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_cors::{CorsOptions, AllowedOrigins};
use std::sync::Arc;
use rocket::State;
use rocket::{launch, routes};
use rocket::{post, get, delete};

#[derive(Deserialize, Serialize, Clone)]
struct Tarea {
    id: Option<u32>,
    descripcion: String,
}

#[derive(Deserialize)]
struct TareaSinId {
    descripcion: String,
}

// Configurar la conexión a MySQL
fn establecer_conexion() -> Pool {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL debe estar definido");

    // Construye `Opts` usando `OptsBuilder`
    let opts = Opts::from_url(&url).expect("URL de conexión inválida");
    Pool::new(opts).expect("Error al crear el pool de conexiones")
}

#[post("/tareas", format = "json", data = "<tarea>")]
fn crear_tarea(conn: &State<Arc<Pool>>, tarea: Json<TareaSinId>) -> Json<Tarea> {
    let mut conn = conn.get_conn().expect("Error al obtener la conexión");

    // Insertar la tarea
    let nueva_tarea = Tarea {
        id: None, // Se generará automáticamente
        descripcion: tarea.descripcion.clone(),
    };

    conn.exec_drop(
        r"INSERT INTO tareas (descripcion) VALUES (:descripcion)",
        params! {
            "descripcion" => nueva_tarea.descripcion.clone(),
        },
    ).expect("Error al insertar nueva tarea");

    // Obtener el ID generado
    let id: u32 = conn.last_insert_id().try_into().expect("Error al obtener el último ID");

    Json(Tarea {
        id: Some(id),
        descripcion: nueva_tarea.descripcion,
    })
}

#[get("/tareas")]
fn listar_tareas(conn: &State<Arc<Pool>>) -> Json<Vec<Tarea>> {
    let mut conn = conn.get_conn().expect("Error al obtener la conexión");

    // Recuperar las tareas
    let tareas = conn
        .query_map(
            "SELECT id, descripcion FROM tareas",
            |(id, descripcion)| Tarea {
                id: Some(id),
                descripcion,
            },
        )
        .expect("Error al listar tareas");

    Json(tareas)
}

#[delete("/tareas/<id>")]
fn eliminar_tarea(id: u32, conn: &State<Arc<Pool>>) -> Json<bool> {
    let mut conn = conn.get_conn().expect("Error al obtener la conexión");

    let resultado = conn
        .exec_drop(
            "DELETE FROM tareas WHERE id = :id",
            params! {
                "id" => id,
            },
        );

    // Devuelve `true` si la operación fue exitosa, `false` si falló
    match resultado {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}



#[launch]
fn rocket() -> _ {
    dotenvy::dotenv().ok();
    let pool = Arc::new(establecer_conexion());

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .to_cors()
        .expect("Error al configurar CORS");

    rocket::build()
        .manage(pool)
        .mount("/", routes![crear_tarea, listar_tareas, eliminar_tarea])
        .attach(cors)
}
