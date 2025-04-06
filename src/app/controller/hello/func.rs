use crate::*;

pub async fn handle(ctx: Context) {
    let name: String = ctx.get_route_param("name").await.unwrap_or_default();
    let _ = ctx.set_response_body(format!("Hello {}", name)).await;
}
