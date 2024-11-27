pub struct SQLClient {
    connection: String,
    pool: Option<sqlx::Pool<sqlx::Any>>,
}

impl SQLClient {
    pub fn new(connection: &str) -> Self {
        Self {
            connection: connection.to_string(),
            pool: None,
        }
    }

    pub async fn connect(&mut self) -> Result<(), sqlx::Error> {
        self.pool = Some(sqlx::any::AnyPoolOptions::new()
            .max_connections(5)
            .connect(&self.connection)
            .await?);
        Ok(())
    }

    pub async fn query<T>(&self, query: &str) -> Result<Vec<T>, sqlx::Error> 
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::any::AnyRow>
    {
        if let Some(pool) = &self.pool {
            let rows = sqlx::query_as::<_, T>(query)
                .fetch_all(pool)
                .await?;
            Ok(rows)
        } else {
            Err(sqlx::Error::Configuration("Not connected to database".into()))
        }
    }

    pub async fn execute(&self, query: &str) -> Result<sqlx::any::AnyQueryResult, sqlx::Error> {
        if let Some(pool) = &self.pool {
            sqlx::query(query)
                .execute(pool)
                .await
        } else {
            Err(sqlx::Error::Configuration("Not connected to database".into()))
        }
    }

    pub async fn transaction<F, R, E>(&self, f: F) -> Result<R, E>
    where
        F: FnOnce(&mut sqlx::Transaction<'_, sqlx::Any>) -> Result<R, E>,
        E: From<sqlx::Error>,
    {
        if let Some(pool) = &self.pool {
            let mut tx = pool.begin().await?;
            let result = f(&mut tx)?;
            tx.commit().await?;
            Ok(result)
        } else {
            Err(sqlx::Error::Configuration("Not connected to database".into()).into())
        }
    }

    pub async fn close(&mut self) {
        if let Some(pool) = self.pool.take() {
            pool.close().await;
        }
    }
}