use std::io::Write;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{bail, Context, Result};
use reqwest::blocking::Client;
use rustls::crypto::ring;

use crate::terminal;

pub fn download(url: &String, writer: impl Write, description: &str) -> Result<()> {
    let client = http_client()?;
    let mut response = client
        .get(url)
        .send()
        .with_context(|| format!("download failed: {}", url))?;

    let pb = terminal::io_progress_bar(
        format!("Downloading {}", description),
        response.content_length().unwrap_or(0),
    );
    response.copy_to(&mut pb.wrap_write(writer))?;
    pb.finish_and_clear();

    if response.status().is_success() {
        Ok(())
    } else {
        bail!("download failed: {}, {}", response.status(), url)
    }
}

fn http_client() -> Result<Client> {
    Ok(Client::builder()
        .timeout(Duration::from_secs(30))
        .use_preconfigured_tls(
            rustls_platform_verifier::tls_config_with_provider(Arc::new(ring::default_provider()))
                .with_context(|| "unable to create TLS configuration")?,
        )
        .build()?)
}
