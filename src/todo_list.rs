use crate::todo::Todo;
use crate::todo_item::todo_item;
use dioxus::prelude::*;
use gloo_storage::LocalStorage;
use gloo_storage::Storage;
use std::collections::HashMap;
use uuid::Uuid;

pub fn todo_list(cx: Scope) -> Element {
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
  })
}
