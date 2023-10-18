#[cfg(all(target_os = "vita"))]
use vita_newlib_shims as _;

use oxhttp::{
    model::{Method, Request},
    Client,
};

fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "full");
    // For openssl instead of rustls you must provide cert files.
    // The shared cert files are located on a vs0 partition which is only accessible with unsafe apps.
    // If you want to use openssl, you must either make your app unsafe or provide your own certs.
    std::env::set_var("SSL_CERT_FILE", "vs0:data/external/cert/CA_LIST.cer");

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            println!(">>>");
            println!(">>> Trying oxhttp");
            let client = Client::new();
            let body = client
                .request(Request::builder(Method::GET, "https://example.com".parse()?).build())?
                .into_body()
                .to_string()?;
            println!(">>> oxhttp response: {body}");

            println!(">>>");
            println!(">>> Trying ureq");
            let res = ureq::get("http://example.com").call()?.into_string()?;
            println!(">>> Ureq response: {:?}", res);

            println!(">>>");
            println!(">>> Trying reqwest");
            let body = reqwest::get("https://example.com").await?.text().await?;
            println!(">>> Reqwest response: {:#?}", body);

            Ok(())
        })
}
