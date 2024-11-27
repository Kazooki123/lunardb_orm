use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryOptions {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub order_by: Option<String>,
    pub filters: Option<HashMap<String, String>>,
}

#[derive(Debug)]
pub struct LunarDB {
    connection: String,
    query_options: QueryOptions,
}

impl LunarDB {
    pub fn new(connection: &str) -> Self {
        Self {
            connection: connection.to_string(),
            query_options: QueryOptions {
                limit: None,
                offset: None,
                order_by: None,
                filters: None,
            },
        }
    }

    pub fn sql(&self) -> crate::sql::SQLClient {
        crate::sql::SQLClient::new(&self.connection)
    }

    pub fn nosql(&self) -> crate::nosql::NoSQLClient {
        crate::nosql::NoSQLClient::new(&self.connection)
    }

    pub fn with_options(&mut self, options: QueryOptions) -> &mut Self {
        self.query_options = options;
        self
    }

    pub fn limit(&mut self, limit: u32) -> &mut Self {
        self.query_options.limit = Some(limit);
        self
    }

    pub fn offset(&mut self, offset: u32) -> &mut Self {
        self.query_options.offset = Some(offset);
        self
    }

    pub fn order_by(&mut self, field: &str) -> &mut Self {
        self.query_options.order_by = Some(field.to_string());
        self
    }

    pub fn filter(&mut self, key: &str, value: &str) -> &mut Self {
        let filters = self.query_options.filters.get_or_insert_with(HashMap::new);
        filters.insert(key.to_string(), value.to_string());
        self
    }

    pub fn get_query_options(&self) -> &QueryOptions {
        &self.query_options
    }

    pub async fn execute_raw_sql(&self, query: &str) -> Result<Vec<HashMap<String, serde_json::Value>>, sqlx::Error> {
        let client = self.sql();
        client.query(query).await
    }

    pub async fn execute_nosql_command(&mut self, command: &str, args: Vec<&str>) -> Result<Option<String>, String> {
        let mut client = self.nosql();
        match command {
            "SET" => {
                if args.len() != 2 {
                    return Err("SET command requires key and value".to_string());
                }
                client.set(args[0], args[1])
            },
            "GET" => {
                if args.len() != 1 {
                    return Err("GET command requires key".to_string());
                }
                Ok(client.get(args[0]))
            },
            "DEL" => {
                if args.len() != 1 {
                    return Err("DEL command requires key".to_string());
                }
                client.del(args[0])
            },
            _ => Err(format!("Unknown command: {}", command))
        }
    }
}