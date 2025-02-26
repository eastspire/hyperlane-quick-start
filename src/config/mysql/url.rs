use super::constant::*;

pub fn get_db_url() -> String {
    format!(
        "mysql://{}:{}@{}:{}/{}",
         USER_NAME, PASSWORD, HOST, PORT, DATABASE
    )
}
