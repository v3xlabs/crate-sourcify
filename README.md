# sourcify

A lightweight read-only Rust wrapper for [Sourcify](https://sourcify.dev),
including the Sourcify v2 API and Sourcify 4byte signature API.

## Supported APIs

- `sourcify::v2`: Sourcify v2 contract data API.
  API docs: <https://sourcify.dev/server/api-docs/swagger.json>
- `sourcify::four_byte`: Sourcify 4byte signature API.
  API docs: <https://api.4byte.sourcify.dev/api-docs/swagger.json>

## What It Does

- Fetch verified contract data by `chain_id` and address.
- Fetch source files, ABI, metadata, deployment info, and compiler details when Sourcify has them.
- Check where an address is verified across all chains.
- Resolve 4-byte function selectors and 32-byte event topics.

## Install

```toml
[dependencies]
sourcify = "0.0.2"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Example

```rust
use sourcify::Sourcify;

#[tokio::main]
async fn main() -> sourcify::Result<()> {
    let client = Sourcify::new();

    let contract = client
        .v2()
        .get_contract_with_fields(
            1,
            "0xdAC17F958D2ee523a2206206994597C13D831ec7",
            &[sourcify::v2::field::SOURCES, sourcify::v2::field::ABI],
        )
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

## API Surface

- `Sourcify::v2().get_contract(chain_id, address)`
- `Sourcify::v2().get_contract_with_fields(chain_id, address, fields)`
- `Sourcify::v2().get_contract_all_chains(address)`
- `Sourcify::v2().is_verified(chain_id, address)`
- `Sourcify::four_byte().lookup_function(selector)`
- `Sourcify::four_byte().lookup_event(topic)`
- `Sourcify::four_byte().search(query)`

This crate is currently read-only. It intentionally does not submit contracts for
verification or import signatures.
