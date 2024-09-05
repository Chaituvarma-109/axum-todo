use axum::{Router, extract::{Path, State}, routing::{get, put}, Json};
use super::{
    store::{Store, Todo, CreateTodoReq, UpdateTaskReq, CreateTodo},
    app_state::AppState
};

pub fn todo_routes() -> Router<AppState> {
    Router::new()
        .route("/todo", get(list_todos).post(create_todo))
        .route("/todo/:id", put(update_todo).delete(delete_todo))
}

async fn list_todos(State(store): State<Store>) -> Json<Vec<Todo>> {
    Json(store.get_todos().await.unwrap())
}

async fn create_todo(State(store): State<Store>, Json(todo): Json<CreateTodoReq>) -> Json<CreateTodo> {
    let todo = store.create_todos(todo.task).await.unwrap();
    Json(todo)
}

async fn update_todo(State(store): State<Store>, Path(id): Path<i32>, Json(todo): Json<UpdateTaskReq>) -> Json<Vec<Todo>> {
    Json(store.update_todo(id, todo.task).await.unwrap())
}

async fn delete_todo(State(store): State<Store>, Path(id): Path<i32>) -> Json<String> {
    let res = store.delete_todo(id).await.unwrap();
    Json(res)
}
