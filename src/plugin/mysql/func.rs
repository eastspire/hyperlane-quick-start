use super::lazy::DB;
use crate::*;

pub async fn get_db_connection() -> Pool<MySql> {
    let db: Pool<MySql> = DB.read().await.clone().unwrap();
    db.clone()
}

pub async fn init_db_connection() {
    let database_url: &str = "mysql://root:SQS@localhost:4466";
    let pool: Pool<MySql> = MySqlPoolOptions::new()
        .max_connections(100)
        .connect(database_url)
        .await
        .unwrap();
    if let Some(db) = DB.write().await.as_mut() {
        *db = pool;
    };
}

pub async fn create_table() {
    let pool: Pool<MySql> = get_db_connection().await;
    let create_table_query: &str = r#"
        CREATE TABLE IF NOT EXISTS `visit` (
            `id` bigint(20) UNSIGNED NOT NULL AUTO_INCREMENT,
            `isdel` bigint(20) UNSIGNED DEFAULT 0,
            `request` longblob DEFAULT '',
            `response` longblob DEFAULT '',
            PRIMARY KEY (`id`)
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='访问记录';
    "#;
    query(create_table_query).execute(&pool).await.unwrap();
}

pub async fn insert_record() {
    let pool: Pool<MySql> = get_db_connection().await;
    let insert_query: &str = r#"
        INSERT INTO `visit` (`isdel`, `request`, `response`)
        VALUES (?, ?, ?)
    "#;
    let request_data: Vec<u8> = vec![1, 2, 3, 4];
    let response_data: Vec<u8> = vec![5, 6, 7, 8];
    sqlx::query(insert_query)
        .bind(0) // isdel
        .bind(&request_data) // request
        .bind(&response_data) // response
        .execute(&pool) // 使用 Arc 中的 pool
        .await
        .unwrap();
}
