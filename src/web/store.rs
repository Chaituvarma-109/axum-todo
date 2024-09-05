use serde::{Deserialize, Serialize};
use sqlx::{query_as, Error, PgPool};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub task: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoReq {
    pub task: String,
}

#[derive(Deserialize)]
pub struct UpdateTaskReq {
    pub task: String,
}

#[derive(Serialize)]
pub struct CreateTodo {
    pub id: i32,
}

#[derive(Clone)]
pub struct Store {
    pool: PgPool
}

impl Store {
    pub fn new(pool: PgPool) -> Self {
        Self{ pool }
    }

    pub async fn get_todos(&self) -> Result<Vec<Todo>, Error> {
        let r = query_as!(Todo, r#"SELECT id, task from task ORDER BY id"#)
            .fetch_all(&self.pool)
            .await.expect("err in get todos");

        Ok(r)
    }

    pub async fn create_todos(&self, task: String) -> Result<CreateTodo, Error> {
        let r = query_as!(CreateTodo, r#"INSERT INTO task (task) VALUES ($1) RETURNING id"#, task)
            .fetch_one(&self.pool)
            .await.expect("err in create todos");

        Ok(r)
    }

    pub async fn update_todo(&self, id: i32, task: String) -> Result<Vec<Todo>, Error> {
        let r = query_as!(Todo, r#"UPDATE task SET task = $1 WHERE id = $2 RETURNING id, task"#, task, id)
            .fetch_all(&self.pool)
            .await
            .expect("err in updating");

        Ok(r)
    }

    pub async fn delete_todo(&self, id: i32) -> Result<String, Error> {
        sqlx::query!(r#"DELETE FROM task WHERE id = $1"#, id).execute(&self.pool).await?;

        Ok("Deleted".to_string())
    }
}
