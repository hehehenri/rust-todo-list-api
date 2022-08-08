pub mod todos {
    use crate::models::Todo;
    use sqlx::MySqlExecutor;


    pub async fn all<'a, D>(pool: D) -> Result<Vec<Todo>, sqlx::Error>
    where
        D: MySqlExecutor<'a> {
        let result = sqlx::query!("SELECT * FROM todos").fetch_all(pool).await?;

        let mut todos: Vec<Todo> = Vec::new();

        for row in result {
            todos.push(
                Todo::new(
                    row.id as u64,
                    row.name,
                    match row.is_checked.unwrap() {
                        0 => false,
                        _ => true
                    }
                )
            );
        }

        Ok(todos)
    }
}