use super::lazy::DB;
use crate::*;

pub async fn init_db_connection() -> MySqlPool {
    let database_url: String = config::mysql::url::get_db_url();
    let connection: MySqlPool = MySqlPoolOptions::new()
        .max_connections(6)
        .connect(&database_url)
        .await
        .unwrap();
    connection
}

pub fn get_db_connection() -> &'static MySqlPool {
    &DB
}

pub async fn create_table() {
    let db: &MySqlPool = get_db_connection();
    let create_table_query: &str = r#"
        CREATE TABLE IF NOT EXISTS `visit` (
            `id` bigint(20) UNSIGNED NOT NULL AUTO_INCREMENT,
            `isdel` bigint(20) UNSIGNED DEFAULT 0,
            `request` longtext DEFAULT '',
            `response` longtext DEFAULT '',
            PRIMARY KEY (`id`)
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='访问记录';
    "#;
    query(create_table_query).execute(db).await.unwrap();
}

pub async fn insert_record(request: &str, response: &str) {
    let db: &MySqlPool = get_db_connection();
    let insert_query: &str = r#"
        INSERT INTO `visit` (`isdel`, `request`, `response`)
        VALUES (?, ?, ?)
    "#;
    sqlx::query(insert_query)
        .bind(0)
        .bind(&request)
        .bind(&response)
        .execute(db)
        .await
        .unwrap();
}
