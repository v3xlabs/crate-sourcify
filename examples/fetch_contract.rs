use sourcify::Sourcify;

#[tokio::main]
async fn main() -> sourcify::Result<()> {
    let chain_id = std::env::args().nth(1).unwrap_or_else(|| "1".to_string());
    let address = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "0xdAC17F958D2ee523a2206206994597C13D831ec7".to_string());

    let client = Sourcify::new();
    let contract = client
        .v2()
        .get_contract_with_fields(chain_id, &address, &["sources", "abi", "metadata"])
        .await?;

    match contract {
        Some(contract) => {
            let source_count = contract.sources.as_ref().map_or(0, |sources| sources.len());
            println!(
                "verified contract: {} on chain {}",
                contract.address, contract.chain_id
            );
            println!("source files: {source_count}");
            println!("has abi: {}", contract.abi.is_some());
            println!("has metadata: {}", contract.metadata.is_some());

            println!("source files: {:?}", contract.sources.as_ref().map(|sources| sources.iter().next().map(|(_, x)|x.content.clone())));
        }
        None => println!("contract is not verified"),
    }

    Ok(())
}
