use super::lazy::DB;
use crate::*;

pub async fn init_db_connection() {
    let database_url = "mysql://root:SQS@localhost:4466/hyperlane";
    let connection = MySqlPoolOptions::new()
        .max_connections(100)
        .connect(database_url)
        .await
        .unwrap();
    let mut db = DB.write().await;
    *db = Some(connection);
}

pub async fn get_db_connection() -> Option<MySqlPool> {
    DB.read().await.clone()
}

pub async fn create_table() {
    let pool: MySqlPool = get_db_connection().await.unwrap();
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
    let pool: MySqlPool = get_db_connection().await.unwrap();
    let insert_query: &str = r#"
        INSERT INTO `visit` (`isdel`, `request`, `response`)
        VALUES (?, ?, ?)
    "#;
    let request_data: Vec<u8> = vec![1, 2, 3, 4];
    let response_data: Vec<u8> = vec![5, 6, 7, 8];
    sqlx::query(insert_query)
        .bind(0) // isdel
        .bind(&request_data)
        .bind(&response_data)
        .execute(&pool)
        .await
        .unwrap();
}
