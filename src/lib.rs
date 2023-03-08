use http_server_tiny::{HttpServer, Method, Res};
use serde::Deserialize;
use serde::Serialize;
use std::{fs::OpenOptions, io::Write};

#[derive(Serialize, Deserialize)]
struct TodoList {
    todos: Vec<String>,
}

pub fn deploy() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = HttpServer::new("0.0.0.0:8000", "./error.html");
    server.add_route(
        &Method::Get,
        "/",
        Box::new(|_| Res::File {
            name: "./index.html",
            ct: "text/html; charset=utf-8",
            sc: 200,
        }),
    );
    server.add_route(
        &Method::Get,
        "/index.js",
        Box::new(|_| Res::File {
            name: "./index.js",
            ct: "text/html; charset=utf-8",
            sc: 200,
        }),
    );
    server.add_route(
        &Method::Post,
        "/api/todo",
        Box::new(|req| {
            add_todo(req.content).unwrap();
            Res::Empty
        }),
    );
    server.add_route(
        &Method::Get,
        "/api/todo",
        Box::new(|_| {
            let todo_list = get_todos().unwrap();
            let todo_list = serde_json::to_string_pretty(&todo_list).unwrap();
            Res::Json(todo_list)
        }),
    );
    server.add_route(
        &Method::Delete,
        "/api/todo",
        Box::new(|_| {
            std::fs::remove_file("index.json").unwrap();
            Res::Empty
        }),
    );
    server.add_route(
        &Method::Delete,
        "/api/todo/remove",
        Box::new(|req| {
            let todos = serde_json::from_str(&req.content).unwrap();
            std::fs::remove_file("index.json").unwrap();
            update_todos(todos).unwrap();

            Res::Empty
        }),
    );

    server.handle_requests()?;

    Ok(())
}

fn add_todo(todo: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("index.json")?;

    let mut todo_list = get_todos()?;

    todo_list.todos.push(todo);

    let text = serde_json::to_string_pretty(&todo_list)?;
    file.write_all(text.as_bytes())?;

    Ok(())
}

fn get_todos() -> Result<TodoList, Box<dyn std::error::Error>> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("index.json")?;
    let file_content = std::fs::read_to_string("index.json")?;

    let mut todo_list: TodoList = TodoList { todos: vec![] };
    if file_content != String::from("") {
        todo_list = serde_json::from_str(&file_content)?;
    }

    Ok(todo_list)
}

fn update_todos(todo_list: TodoList) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("index.json")?;

    let text = serde_json::to_string_pretty(&todo_list)?;

    file.write_all(text.as_bytes())?;

    Ok(())
}
