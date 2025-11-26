use askama::Template;
use axum::response::{Html, IntoResponse};

#[derive(askama::Template)]
#[template(path = "components/todo.html")]
pub struct TodoTemplate {
    pub id: i64,
    pub text: String,
    pub done: bool,
}

#[derive(askama::Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    todos: Vec<TodoTemplate>,
}

impl IntoResponse for TodoTemplate {
    fn into_response(self) -> axum::response::Response {
        Html(self.render().map(Html).map_err(|err| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                err.to_string(),
            )
        }))
        .into_response()
    }
}
