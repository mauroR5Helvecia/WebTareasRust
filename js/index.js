document.getElementById("formularioTarea").addEventListener("submit", async function(event) {
    event.preventDefault();

    const tarea = document.getElementById("nuevaTarea").value;
    if (!tarea) return;

    const response = await fetch("http://localhost:8000/tareas", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({ descripcion: tarea })
    });

    if (response.ok) {
        cargarTareas();
        document.getElementById("nuevaTarea").value = "";
    }
});

async function cargarTareas() {
    const response = await fetch("http://localhost:8000/tareas");
    const tareas = await response.json();

    const listaTareas = document.getElementById("listaTareas");
    listaTareas.innerHTML = "";

    tareas.forEach(tarea => {
        const li = document.createElement("li");
        li.textContent = tarea.descripcion;

        const botonEliminar = document.createElement("button");
        botonEliminar.textContent = "Eliminar";
        botonEliminar.className = "ml-2 bg-red-500 text-white px-2 py-1 rounded hover:bg-red-700 transition";
        botonEliminar.onclick = async () => {
            await fetch(`http://localhost:8000/tareas/${tarea.id}`, { method: "DELETE" });
            cargarTareas();
        };

        li.appendChild(botonEliminar);
        listaTareas.appendChild(li);
    });
}

cargarTareas();
