use dioxus::prelude::*;
use gloo_storage::LocalStorage;
use gloo_storage::Storage;
use std::collections::HashMap;
use uuid::Uuid;

fn main() {
    dioxus_web::launch(app);
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub content: String,
}

fn app(cx: Scope) -> Element {
    let serialised_todos: String = LocalStorage::get("todos").unwrap();
    let existing_todos: HashMap<Uuid, Todo> = serde_json::from_str(&serialised_todos).unwrap();
    let text_content = use_ref(cx, String::new);
    let todos = use_ref(cx, || existing_todos);

    let handle_form_submit = move |_| {
        let id = Uuid::new_v4();
        todos.write().insert(
            id,
            Todo {
                id: id,
                content: text_content.read().clone(),
            },
        );
        let serialised_todos = serde_json::to_string(&todos.read().to_owned()).unwrap();
        let _ = LocalStorage::set("todos", serialised_todos);
        text_content.set("".to_string());
    };

    cx.render(rsx! {
        head {
            link {
                href: "https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css",
                rel: "stylesheet",
                integrity: "sha384-9ndCyUaIbzAi2FUVXJi0CjmCapSmO7SnpJef0486qhLnuZ2cdeRhO02iuK6FUUVM",
                crossorigin: "anonymous"
            }
            link {
                href: "https://cdn.jsdelivr.net/npm/bootstrap-icons@1.10.5/font/bootstrap-icons.css",
                rel: "stylesheet",
            }
        }
        body {
            div {
                class: "py-5 bg-primary-subtle",
                style: "height: 100vh;",
                div {
                class: "container-fluid d-flex justify-content-center",
                    div {
                        class: "col-12 col-sm-8",
                        div {
                            class: "bg-light shadow rounded mx-2 overflow-hidden",
                            form {
                                prevent_default: "onsubmit",
                                onsubmit: handle_form_submit,
                                input {
                                    oninput: move |evt| text_content.set(evt.value.clone()),
                                    value: "{text_content.read()}",
                                    class: "w-100 p-3 border-0 shadow-none form-control",
                                    placeholder: "What needs to be done?"
                                }
                            }
                            if !todos.read().is_empty(){
                                rsx! {
                                    ul {
                                        class: "list-group",
                                        todos.read().iter().map(|(id, _todo)| render!(todo_item{id: *id, set_todos: todos}))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

    })
}

#[derive(Props)]
pub struct TodoItemProps<'a> {
    set_todos: &'a UseRef<HashMap<Uuid, Todo>>,
    id: Uuid,
}

fn todo_item<'a>(cx: Scope<'a, TodoItemProps<'a>>) -> Element {
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
