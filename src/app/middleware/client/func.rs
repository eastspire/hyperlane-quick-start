use crate::*;

pub async fn client(ctx: Context) {
    let socket_addr: String = ctx.get_socket_addr_or_default_string().await;
    let mut ctx: RwLockWriteInnerContext = ctx.get_write_lock().await;
    let response: &mut Response = ctx.get_mut_response();
    response.set_header("SocketAddr", socket_addr);
}
