use sqlx::{Error, MySqlPool};

use crate::active_records::User;


pub fn create(username: String) -> User {
    User {
        id: 0,
        username,
    }
}

impl User {
    pub async fn execute(&mut self, db: &MySqlPool) -> Result<u64, Error> {
        let result = sqlx::query("insert into `user`(`username`) values(?)")
            .bind(&self.username).execute(db).await;


        match result {
            Ok(row) => {
                self.id = row.last_insert_id();
                Ok(self.id)
            },
            Err(err) => Err(err),
        }
    }
}

