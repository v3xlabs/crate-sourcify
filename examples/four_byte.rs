use sourcify::Sourcify;

#[tokio::main]
async fn main() -> sourcify::Result<()> {
    let selector = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "0xa9059cbb".to_string());

    let client = Sourcify::new();
    let signatures = client.four_byte().lookup_function(selector).await?;

    if signatures.is_empty() {
        println!("no signatures found");
    } else {
        for signature in signatures {
            println!("{}", signature.name);
        }
    }

    Ok(())
}
