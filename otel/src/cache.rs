use redis::{Client, Commands, Connection};
use crate::entity::todo;
use uuid::Uuid;

#[derive(Clone)]
pub struct RedisCache {
    client: Client,
}

impl RedisCache {
    pub fn new(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = Client::open(redis_url)?;
        Ok(RedisCache { client })
    }

    fn get_connection(&self) -> Result<Connection, redis::RedisError> {
        self.client.get_connection()
    }

    pub fn get_todo(&self, id: &Uuid) -> Result<Option<todo::Model>, redis::RedisError> {
        let mut conn = self.get_connection()?;
        let key = format!("todo:{}", id);
        let json: Option<String> = conn.get(&key)?;
        
        match json {
            Some(data) => {
                let todo_model: todo::Model = serde_json::from_str(&data)
                    .map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, "JSON parse error", e.to_string())))?;
                Ok(Some(todo_model))
            }
            None => Ok(None),
        }
    }

    pub fn set_todo(&self, todo: &todo::Model) -> Result<(), redis::RedisError> {
        let mut conn = self.get_connection()?;
        let key = format!("todo:{}", todo.id);
        let json = serde_json::to_string(todo)
            .map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, "JSON serialize error", e.to_string())))?;
        
        conn.set_ex(&key, json, 3600) // 1 hour TTL
    }

    pub fn delete_todo(&self, id: &Uuid) -> Result<bool, redis::RedisError> {
        let mut conn = self.get_connection()?;
        let key = format!("todo:{}", id);
        let deleted: i32 = conn.del(&key)?;
        Ok(deleted > 0)
    }
}