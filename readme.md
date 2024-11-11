### Administrador de Tareas en Rust
## Descripción del Proyecto
Este proyecto surge como mi primer intento de aprender un nuevo lenguaje de programación: Rust. Mi objetivo inicial fue crear un administrador de tareas básico como una forma de familiarizarme con las capacidades del lenguaje y explorar su ecosistema.

Al empezar a investigar, descubrí que Rust tiene un enfoque principal en el desarrollo de sistemas, con una fuerte orientación hacia la seguridad de memoria, concurrencia y eficiencia en el manejo de recursos. Sin embargo, también cuenta con bibliotecas creadas por la comunidad para el desarrollo web, lo que despertó mi curiosidad por probarlas y ver cómo se comporta Rust en esta área.

## Tecnologías y Dependencias Utilizadas
Este proyecto utiliza varias bibliotecas para ayudar con el desarrollo web y la interacción con bases de datos en Rust:

***Rocket:*** Framework web para Rust que proporciona una forma rápida y segura de crear APIs REST.
Versión: 0.5.1
Características: Gestión de rutas, respuesta a peticiones, manejo de JSON, etc.
***Rocket CORS:*** Middleware para habilitar CORS (Cross-Origin Resource Sharing) en aplicaciones ***Rocket.***
Versión: 0.6.0
Usado para permitir peticiones desde diferentes orígenes.
***Serde:*** Biblioteca de serialización y deserialización que facilita trabajar con datos en formato JSON.
Versión: 1.0
***Mysql:*** Conector para interactuar con bases de datos MySQL desde Rust.
Versión: 25.0.1
***Dotenvy:*** Permite cargar variables de entorno desde un archivo .env.
Versión: 0.15
## Proceso de Desarrollo
El proyecto comenzó con el diseño de una API básica para manejar tareas. La API ofrece funcionalidades para crear, listar y eliminar tareas en una base de datos MySQL. Durante el desarrollo, me encontré con algunos desafíos relacionados con la integración de MySQL y el manejo de conexiones en Rust. Sin embargo, esto me permitió entender mejor cómo Rust interactúa con otros sistemas y cómo optimizar la gestión de recursos.

## Creación de la Tabla en MySQL
Para que la aplicación funcione correctamente, es necesario que exista una tabla tareas en la base de datos MySQL configurada. Puedes crearla ejecutando el siguiente comando SQL:

sql
Copiar código
CREATE TABLE tareas (
    id INT AUTO_INCREMENT PRIMARY KEY,
    descripcion VARCHAR(255) NOT NULL
);
Configuración del Archivo .env
El archivo .env debe contener la URL de conexión a la base de datos. Asegúrate de configurar esta URL según tu configuración local. Un ejemplo de configuración es el siguiente:

DATABASE_URL=mysql://mauro:admin@localhost:3306/tareas_rust_db
remplaza mauro, admin y tareas_rust_db con tus valores locales de usuario, contraseña y nombre de la base de datos.

Principales Características
Rutas de la API
POST /tareas: Crea una nueva tarea.
GET /tareas: Lista todas las tareas.
DELETE /tareas/<id>: Elimina una tarea específica por su ID.


## Conexión a MySQL
Configurada usando la biblioteca mysql desde ***crates.io*** https://crates.io/.
Manejo de conexiones mediante Pool para optimizar el uso de recursos.


Beneficios de Rust para el Desarrollo
Rust es conocido por su enfoque en la seguridad y eficiencia de la memoria. Al usar Rust, obtuve una experiencia robusta en la gestión de errores, concurrencia y rendimiento, lo que lo convierte en un lenguaje ideal para desarrollar sistemas críticos o de alta carga. Algunos de los beneficios clave que ofrece Rust son:

Seguridad de memoria: El sistema de propiedades de Rust evita errores comunes como segmentation faults y condiciones de carrera.
Alto rendimiento: Compilación directa a binarios, sin necesidad de un runtime.
Concurrencia segura: Permite escribir código concurrente de manera más segura, evitando problemas como deadlocks.


## Próximos Pasos
Continuaré aprendiendo Rust, enfocándome en su capacidad para manejar memoria de manera eficiente y explorando nuevas aplicaciones en el desarrollo de sistemas y servicios web.