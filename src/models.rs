use serde::{Deserialize, Serialize};
use super::schema::tareas;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = tareas)]
pub struct Tarea {
    pub id: i32,
    pub descripcion: String,
}
