use crate::*;
use hyperlane::once_cell::sync::Lazy;

pub static DB: Lazy<Arc<MySqlPool>> = Lazy::new(|| {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime
        .block_on(async {
            let database_url = "mysql://root:SQS@localhost:4466";
            MySqlPoolOptions::new()
                .max_connections(100)
                .connect(database_url)
                .await
                .unwrap()
        })
        .into()
});
