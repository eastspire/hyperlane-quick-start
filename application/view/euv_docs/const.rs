/// Directory path for euv docs view redirect path.
///
/// Resolved by `application/view/github_pages` against the
/// `euv-dev/euv-docs` repository (see
/// `config/application/github_pages/const::SYNC_REPOSITORIES` for the
/// pre-fetch list). The trailing slash is intentional: GitHub Pages
/// serves the repository root via the slash form, and the proxy router
/// in `view/github_pages` redirects directory lookups there with a 301.
pub const EUV_DOCS_VIEW_REDIRECT_PATH: &str = "/github/pages/euv-dev/euv-docs/";
