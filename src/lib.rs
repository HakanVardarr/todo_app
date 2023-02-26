use handler::handle_file;
use serde::Deserialize;
use serde::Serialize;
use std::{fs::OpenOptions, io::Write};
use tiny_http::{Header, Method, Response, Server};
mod handler;

#[derive(Serialize, Deserialize)]
struct TodoList {
    todos: Vec<String>,
}

pub fn deploy() -> Result<(), std::io::Error> {
    let server = Server::http("0.0.0.0:8000").unwrap();
    println!("Server started on: localhost:8000");

    for mut request in server.incoming_requests() {
        println!(
            "INFO: Recieved request!\nMethod: {:?}\nUrl: {:?}\n",
            request.method(),
            request.url(),
        );

        match (request.method(), request.url()) {
            (Method::Get, "/") => {
                handle_file(request, "index.html", Some("text/html; charset=utf-8"))?;
            }
            (Method::Get, "/index.js") => {
                handle_file(request, "index.js", Some("text/javascript; charset=utf-8"))?;
            }
            (Method::Post, "/api/todo") => {
                let mut todo = String::new();
                request.as_reader().read_to_string(&mut todo)?;
                add_todo(todo)?;
                request.respond(Response::empty(200))?;
            }
            (Method::Get, "/api/todo") => {
                let todo_list = get_todos()?;
                let todo_list = serde_json::to_string_pretty(&todo_list)?;
                let header = Header::from_bytes("Content-Type", "application/json").expect("ERROR");
                request.respond(
                    Response::from_string(&todo_list)
                        .with_status_code(200)
                        .with_header(header),
                )?;
            }
            (Method::Delete, "/api/todo") => {
                std::fs::remove_file("index.json")?;
                request.respond(Response::empty(200))?;
            }
            (Method::Delete, "/api/todo/remove") => {
                let mut new_todo_list = String::new();
                request.as_reader().read_to_string(&mut new_todo_list)?;

                let json_todos = serde_json::from_str(&new_todo_list)?;

                std::fs::remove_file("index.json")?;
                update_todos(json_todos)?;

                request.respond(Response::empty(200))?;
            }
            _ => handle_file(request, "error.html", None)?,
        }
    }

    Ok(())
}

fn add_todo(todo: String) -> Result<(), std::io::Error> {
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

fn get_todos() -> Result<TodoList, std::io::Error> {
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

fn update_todos(todo_list: TodoList) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("index.json")?;

    let text = serde_json::to_string_pretty(&todo_list)?;
    println!("{text}");
    file.write_all(text.as_bytes())?;

    Ok(())
}
