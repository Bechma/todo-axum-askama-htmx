# todo-axum-askama-htmx
Sample application with:
- Backend: axum, sqlite and askama
- Frontend: HTMX and TailwindCSS

![How the application looks](workflow.gif)

## Run

You need to have [rust](https://www.rust-lang.org/) installed.

```bash
export DATABASE_URL=DATABASE_URL=sqlite://./todosdb.sqlite
cargo install sqlx-cli
sqlx database create
sqlx migrate run
cargo run
```

## Development

You need to have completed the previous steps and also have [node](https://nodejs.org/en/download) installed:

```bash
# For tailwind hot reload
npm i -D tailwindcss
npm run dev
```

```bash
# For the http server hot reload
# Remember have your DATABASE_URL exported
cargo install cargo-watch
cargo watch -x run -w templates -w src -w assets
```
