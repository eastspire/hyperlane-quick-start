use super::*;

async fn http_buffer_size(server: &Server) {
    server.http_buffer_size(SERVER_HTTP_BUFFER_SIZE).await;
    println_success!(
        "Server http buffer size: ",
        SERVER_HTTP_BUFFER_SIZE,
        SPACE,
        "bytes"
    );
}

async fn ws_buffer_size(server: &Server) {
    server.ws_buffer_size(SERVER_WS_BUFFER_SIZE).await;
    println_success!(
        "Server websocket buffer size: ",
        SERVER_WS_BUFFER_SIZE,
        SPACE,
        "bytes"
    );
}

async fn host(server: &Server) {
    server.host(SERVER_HOST).await;
    println_success!("Server host: ", SERVER_HOST);
}

async fn linger(server: &Server) {
    server.set_linger(SERVER_LINGER).await;
    println_success!("Server linger: ", format!("{:?}", SERVER_LINGER));
}

async fn port(server: &Server) {
    server.port(SERVER_PORT).await;
    println_success!("Server port: ", SERVER_PORT);
}

async fn nodelay(server: &Server) {
    server.set_nodelay(SERVER_NODELAY).await;
    println_success!("Server nodelay: ", SERVER_NODELAY);
}

async fn error_handler(server: &Server) {
    server
        .error_handler(exception::framework::error_handler)
        .await;
}

async fn ttl(server: &Server) {
    server.set_ttl(SERVER_TTI).await;
    println_success!("Server ttl: ", SERVER_TTI);
}

async fn register_request_middleware(server: &Server) {
    server
        .request_middleware(middleware::request::cross::cross)
        .await;
    server
        .request_middleware(middleware::request::response::response_header)
        .await;
    server
        .request_middleware(middleware::request::response::response_status_code)
        .await;
    server
        .request_middleware(middleware::request::response::response_body)
        .await;
    println_success!("Server request middleware initialization completed");
}

async fn register_response_middleware(server: &Server) {
    server
        .response_middleware(middleware::response::send::send)
        .await;
    server
        .response_middleware(middleware::response::log::log)
        .await;
    println_success!("Server response middleware initialization completed");
}

async fn register_route(server: &Server) {
    server.route("/", controller::root::handle).await;
    server
        .route(format!("/hello/{{{NAME_KEY}}}"), controller::hello::handle)
        .await;
    server.route("/websocket", controller::ws::handle).await;
    server
        .route("/favicon.ico", controller::favicon_ico::handle)
        .await;
    println_success!("Server route initialization completed");
}

fn runtime() -> Runtime {
    Builder::new_multi_thread()
        .worker_threads(num_cpus::get_physical())
        .thread_stack_size(1_048_576)
        .max_blocking_threads(2_048)
        .max_io_events_per_tick(1_024)
        .enable_all()
        .build()
        .unwrap()
}

#[hyperlane(server)]
async fn create_server() {
    host(&server).await;
    port(&server).await;
    ttl(&server).await;
    linger(&server).await;
    nodelay(&server).await;
    error_handler(&server).await;
    http_buffer_size(&server).await;
    ws_buffer_size(&server).await;
    register_request_middleware(&server).await;
    register_route(&server).await;
    register_response_middleware(&server).await;
    let host_port: String = format!("{SERVER_HOST}:{SERVER_PORT}");
    println_success!("Server initialization successful");
    let server_result: ServerResult = server.run().await;
    match server_result {
        Ok(_) => println_success!("Server listen in: ", host_port),
        Err(server_error) => println_error!("Server run error: ", server_error),
    }
}

pub fn run() {
    runtime().block_on(hyperlane_plugin::server_manager::create_server_manage(
        create_server,
    ));
}
