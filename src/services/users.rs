use crate::databases;
use crate::active_records::User;

pub async fn create(username: String) -> Result<User, String> {
    let db = databases::get_db().await;

    let result = sqlx::query("insert into `user`(`username`) values(?)")
        .bind(&username).execute(&db).await;

    match result {
        Ok(row) => {
            Ok(User {
                id: row.last_insert_id(),
                username,
            })
        }
        Err(err) => {
            Err(err.to_string())
        }
    }
}