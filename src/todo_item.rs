use dioxus::prelude::*;
use gloo_storage::LocalStorage;
use gloo_storage::Storage;
use std::collections::HashMap;
use uuid::Uuid;

use crate::todo::Todo;

#[derive(Props)]
pub struct TodoItemProps<'a> {
    set_todos: &'a UseRef<HashMap<Uuid, Todo>>,
    id: Uuid,
}

pub fn todo_item<'a>(cx: Scope<'a, TodoItemProps<'a>>) -> Element {
    let todo = &cx.props.set_todos.read()[&cx.props.id];

    render!(
        li {
            class: "p-3 border-top rounded-0",
            key: "{todo.id}",
            div {
                class: "d-flex align-items-center",
                span {
                    "{todo.content}"
                }
                button {
                    onclick: move |_| {
                        cx.props.set_todos.write().remove(&cx.props.id);
                        let serialised_todos = serde_json::to_string(&cx.props.set_todos.read().to_owned()).unwrap();
                        let _ = LocalStorage::set("todos", serialised_todos);
                    },
                    class: "ms-auto btn btn-light",
                    i {
                        class: "bi bi-trash-fill",
                    }
                }
            }
        }
    )
}
