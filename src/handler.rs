use hyper::{Body, Request, Response};
use std::path::Path;

use crate::{compression, control_headers, static_files};
use crate::{error::Result, error_page};

/// Main server request handler.
pub async fn handle_request(
    base: &Path,
    compression: bool,
    dir_listing: bool,
    req: &Request<Body>,
) -> Result<Response<Body>> {
    let headers = req.headers();
    let method = req.method();

    match static_files::handle_request(method, headers, base, req.uri().path(), dir_listing).await {
        Ok(mut resp) => {
            // Auto compression based on the `Accept-Encoding` header
            if compression {
                resp = compression::auto(method, headers, resp)?;
            }

            // Append `Cache-Control` headers for web assets
            let ext = req.uri().path().to_lowercase();
            control_headers::with_cache_control(&ext, &mut resp);

            Ok(resp)
        }
        Err(status) => error_page::get_error_response(method, &status),
    }
}
