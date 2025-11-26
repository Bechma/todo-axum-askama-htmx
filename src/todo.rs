use askama::Template;

#[derive(askama::Template)]
#[template(path = "components/todo.html")]
pub struct TodoTemplate {
    pub id: i64,
    pub text: String,
    pub done: bool,
}

#[derive(askama::Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub todos: Vec<TodoTemplate>,
}

macro_rules! impl_into_response {
    ($($ty:ty),*) => {
        $(
impl axum::response::IntoResponse for $ty {
    fn into_response(self) -> axum::response::Response {
        axum::response::Html(self.render().map(axum::response::Html).map_err(|err| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                err.to_string(),
            )
        }))
        .into_response()
    }
}
        )*
    };
}

impl_into_response!(TodoTemplate, IndexTemplate);
