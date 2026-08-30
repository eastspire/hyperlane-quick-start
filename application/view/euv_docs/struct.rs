use super::*;

/// Route structure for the EUV docs view endpoints.
///
/// `GET /euv-docs` 302-redirects to the GitHub Pages proxy path that
/// serves the rendered `euv-dev/euv-docs` site. The proxy at
/// `/github/pages/euv-dev/euv-docs/[...]` is backed by `GithubPagesService`,
/// which fetches the matching asset from
/// `https://euv-dev.github.io/euv-docs/` on first request and caches it.
#[route("/euv-docs")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct EuvDocsViewRoute;
