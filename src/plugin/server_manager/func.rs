use crate::*;
use server_manager::*;

pub async fn creat_server_manage<F, Fut>(server_func: F)
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    let args: Vec<String> = std::env::args().collect();
    let config: ServerManagerConfig = ServerManagerConfig {
        pid_file: config::process::PID_FILE_PATH.to_owned(),
    };
    let manager: ServerManager<F> = ServerManager::new(config, server_func);

    if args.len() < 2 {
        manager.start().await;
        return;
    }

    let is_daemon: bool = args.len() >= 3 && args[2].to_lowercase() == "-d";
    let command: String = args[1].to_lowercase();

    let start_server = || async {
        if is_daemon {
            match manager.start_daemon() {
                Ok(_) => println_success!("Server started in background successfully"),
                Err(e) => println_danger!(format!("Error starting server in background: {}", e)),
            }
        } else {
            println_success!("Server started successfully");
            manager.start().await;
        }
    };

    let stop_server = || async {
        match manager.stop() {
            Ok(_) => println_success!("Server stopped successfully"),
            Err(e) => println_danger!(format!("Error stopping server: {}", e)),
        }
    };

    let restart_server = || async {
        stop_server().await;
        start_server().await;
    };

    match command.as_str() {
        "start" => start_server().await,
        "stop" => stop_server().await,
        "restart" => restart_server().await,
        _ => {
            println_danger!(format!("Invalid command: {}", command));
        }
    }
}
