use dioxus::prelude::*;
mod todo;
mod todo_item;
mod todo_list;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
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
                    todo_list::todo_list{}
                }
            }
        }

    })
}
