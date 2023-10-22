use sqlx::PgPool;

#[derive(sqlx::FromRow)]
pub struct Todo {
    pub(crate) id: i32,
    pub(crate) description: String,
    pub done: bool
}

pub struct Todos<'a> {
    pool: &'a PgPool
}

impl Todos<'_> {
    pub fn from(pool: &PgPool) -> Todos {
        Todos { pool }
    }

    pub async fn get(&self, id: i32) -> anyhow::Result<Todo> {
        let todo = sqlx::query_as!(Todo, r#"SELECT id,description,done FROM todos WHERE id=$1"#, id)
            .fetch_one(self.pool)
            .await?;

        Ok(todo)
    }

    pub async fn add(&self, description: String) -> anyhow::Result<i32> {
        let rec = sqlx::query!(
            r#"
            INSERT INTO todos ( description )
            VALUES ( $1 )
            RETURNING id
            "#,
            description
        )
            .fetch_one(self.pool)
            .await?;

        Ok(rec.id)
    }

    pub async fn complete(&self, id: i32) -> anyhow::Result<u64> {
        let rows_affected = sqlx::query!(
            r#"UPDATE todos SET done = true WHERE id=$1"#,
            id
        ).execute(self.pool).await?.rows_affected();

        Ok(rows_affected)
    }

    pub async fn list(&self) -> anyhow::Result<Vec<Todo>> {
        let todos = sqlx::query_as!(Todo, r#"SELECT id,description,done FROM todos"#)
            .fetch_all(self.pool)
            .await?;

        Ok(todos)
    }

}