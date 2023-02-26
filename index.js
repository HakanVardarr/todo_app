let input = document.getElementById("query");
let todos = document.getElementById("todos");
let clear = document.getElementById("clear");

async function post(text){
    await fetch("/api/todo", {
        headers: {'Content-Type': 'text/plain'},
        body: text,
        method: 'POST',
    })
};


async function del(){
    await fetch("/api/todo", {
        method: 'DELETE'
    })
}

async function update(json){
    await fetch("/api/todo/remove", {
        method: 'DELETE',
        headers: {'Content-Type': 'text/plain'},
        body: json
    })
}


async function get(){
    const response = await fetch("/api/todo", {
        method: 'GET',
    })

    const json = await response.json();

    for (todo in json.todos){
        let todo_text = json.todos[todo];
        let item = document.createElement("span");
        let button = document.createElement("button");
        button.innerHTML = "X";
        button.style = "margin-left: 10px";
        button.id = todo;
        button.addEventListener("click", (e) => {
            let id = e.target.id;
            json.todos.splice(id, 1);
            update(JSON.stringify(json));
            location.reload();
        })
        item.appendChild(document.createTextNode(todo_text));
        item.appendChild(button);
        item.appendChild(document.createElement("br"));

        todos.appendChild(item);
    }
}



function main(){
    clear.addEventListener("click", (e) => {
        del();
        location.reload();
    });
    input.addEventListener("keypress", (e) => {
        if (e.key == "Enter"){
            post(input.value);
            location.reload();
        }
    });
    document.addEventListener("DOMContentLoaded", (e) => {
        get();
    });
}

main();