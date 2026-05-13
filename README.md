# sourcify

A lightweight read-only Rust wrapper for the Sourcify v2 API and Sourcify 4byte
signature API.

## What It Does

- Fetch verified contract data by `chain_id` and address.
- Fetch source files, ABI, metadata, deployment info, and compiler details when Sourcify has them.
- Check where an address is verified across all chains.
- Resolve 4-byte function selectors and 32-byte event topics.

## Example

```rust
use sourcify::Sourcify;

#[tokio::main]
async fn main() -> sourcify::Result<()> {
    let client = Sourcify::new();

    let contract = client
        .v2()
        .get_contract(1, "0xdAC17F958D2ee523a2206206994597C13D831ec7")
        .await?;

    if let Some(contract) = contract {
        println!("{} source files", contract.sources.map_or(0, |sources| sources.len()));
    }

    let signatures = client.four_byte().lookup_function("0xa9059cbb").await?;
    for signature in signatures {
        println!("{}", signature.name);
    }

    Ok(())
}
```

## Examples

```sh
cargo run --example fetch_contract -- 1 0xdAC17F958D2ee523a2206206994597C13D831ec7
cargo run --example four_byte -- 0xa9059cbb
```
