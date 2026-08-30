use super::*;

/// Renders the euv docs page and serves as the OpenAPI documentation endpoint for euv-docs routes.
#[utoipa::path(
    get,
    path = "/euv-docs",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_euv_docs_view() {}
