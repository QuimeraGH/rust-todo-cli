# TodoCLI

`TodoCLI` es una herramienta de línea de comandos escrita en Rust para gestionar tus tareas pendientes (todos). Con esta herramienta, puedes agregar, listar, buscar, marcar y eliminar tareas.

## Uso

```bash
todo-cli <command>
```

## Comandos

- `print`  
  Imprimir todos los todos.

- `print-yes`  
  Imprimir los todos completados.

- `print-no`  
  Imprimir los todos no completados.

- `add`  
  Añadir un todo.

- `find`  
  Encontrar un todo por título.

- `mark`  
  Marcar un todo como completado o incompleto.

- `delete`  
  Borrar un todo por ID.

## Ejemplos

- Para imprimir todos los todos:
  ```bash
  todo print
  ```

- Para imprimir los todos completados:
  ```bash
  todo print-yes
  ```

- Para imprimir los todos no completados:
  ```bash
  todo print-no
  ```

- Para añadir un nuevo todo:
  ```bash
  todo add
  ```

- Para encontrar un todo por título:
  ```bash
  todo find "Título del todo"
  ```

- Para marcar un todo como completado:
  ```bash
  todo mark <ID>
  ```

- Para borrar un todo por ID:
  ```bash
  todo delete <ID>
  ```

## Contribuciones

Este proyecto es extremadamente pequeño, por ende no es relevante contribuir, sin embargo puedes dejar un comentario.

## Licencia

Este proyecto está bajo la Licencia MIT. Consulta el archivo `LICENSE` para más detalles.

---

# TodoCLI

`TodoCLI` is a command-line tool written in Rust for managing your to-do tasks. With this tool, you can add, list, search, mark, and delete tasks.

## Usage

```bash
todo <command>
```

## Commands

- `print`  
  Print all todos.

- `print-yes`  
  Print completed todos.

- `print-no`  
  Print incomplete todos.

- `add`  
  Add a new todo.

- `find`  
  Find a todo by title.

- `mark`  
  Mark a todo as completed or incomplete.

- `delete`  
  Delete a todo by ID.

## Examples

- To print all todos:
  ```bash
  todo print
  ```

- To print completed todos:
  ```bash
  todo print-yes
  ```

- To add a new todo:
  ```bash
  todo add "Todo Title"
  ```

- To find a todo by title:
  ```bash
  todo find "Todo Title"
  ```

- To mark a todo as completed:
  ```bash
  todo mark <ID> yes
  ```

- To delete a todo by ID:
  ```bash
  todo delete <ID>
  ```

## Contributing

As this project is extremely small, contributing is not relevant, although you can leave a comment.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.