#[derive(askama::Template)]
#[template(path = "components/todo.html")]
pub struct TodoTemplate {
    pub id: i64,
    pub text: String,
    pub done: bool,
}
