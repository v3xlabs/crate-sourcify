<h1 align="center">
    sourcify
</h1>

<p align="center">
  A lightweight read-only Rust wrapper for <a href="https://sourcify.dev" target="_blank">Sourcify</a>
</p>
<p align="center">
    <a href="https://docs.rs/sourcify"><img src="https://img.shields.io/badge/Docs.rs-blue?logo=rust&color=brown&style=flat" alt="Documentation"></a>
    <a href="https://crates.io/crates/sourcify"><img src="https://img.shields.io/badge/Crate.io-yellow?logo=data:image/x-icon;base64%2CiVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAMAAAAoLQ9TAAACylBMVEUAAADqvWfotVLot1n/wi3lt1/grEfls1HnuFzXnCnXnS3luWTfzavgq0PlwXvltFXlrkTnt4KJYzl4XDz/yHwUEAsIBgQXEgyhek+DZ0j///9JOShFNibsxHnls1Llsk7ntVTpu2Lnv3Lntlf///bhrkrkslHnumTouFvpuV3otlbntFDotlbZoDDpuV7ot1fntlbnumLWnjLirEHlsUvcpz/lr0XlqznmsEjsvGDaozbmrDrgq0LpyIbmtlnlslDntVbdqUXmrTzfq0TbqUXcp0DirELgrUjLlS3OlCPiqTjkrULDjCLDiRfbozfrs0WUahe4fw3hqTrhrEVYRCiQaCfDlk2yjV0yJxppUDZ1WzqEYCKMZzqZd1GxjmgAAAALCAYEAwIwJRt2WjmKZi6fbxeifVKKZ0J3VjNlTC8AAAAAAAALCAVCMiI0KB1DMx+NajaIZDp5WDSFZkOEak4AAAAcFQ5xVzyffViAYDxkTjXnu2XnuV/dpzzjtFnlsk/ntFPntlnnu2Tou2Tbozfjrkfntlfou2XoumLntFLntlbnumLnu2Pir0znvWznumPntljotFDnskrkqzzmrkDbojTgrEfgqkHmr0XiqTvgpzbmrT3mrDjnrDjQliLTmCXVmyvhqDjJljHMmDLnrj/prjzorz/kt17nvGnZp0XGjBjKjxvQlSTepTfEkjHTnTXpsEHpsUTXojzbpj7SnTTDjSTGkSrIjhzjqjnepzrjrD7lr0biqz/cpTnHjBjKjx3MlCfNlSjJlC3GjiLhqTzls1Djrkbkqz3UnjTCkC++gxC/hRLFjB65gxm9hhvPliffpzjlrT3lrDrPmzTKljKwgy+4gBO/hhWrdxGlcxLFjR7kqTfnrDnorz7jqjvRnj2ndx6xeg6odAy/hxfkqjjqsUHmrkHUoUSxhUG7gxXgpzjRo1CmgEewh00nmQYaAAAAe3RSTlMAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAMpetXWfCgBT9/9/d2TQg1v88FlCG7ZG2z+1xsOi9cbASl3xvbXGxHA1xwV0NgdFc/YHRrR2R004+ozJaHF1PauIQoaMmrr9vf0sk8MAQgcWnic+fGtSwsQPrm/UQuNPWZaAAABG0lEQVQY0wEQAe/+AAAAAAEdHh8gISIjJAIDBAUAAAAABiUmJ3t8KCkqKywHCAAAAAAJLX1+f4CBgoMuLzAxAAAAAAoyhIWGh4iJiouMMzQAAAsMDTWNjo+QkZI2k5Q3OAAODxA5OpWWl5iZmpucnTs8AD0+P0BBnp+goaKjpKWmQkMAREWnqKmqq6ytrq+wsYpGRwBISbKztLW2t7i5uru8vUpLAExNvr/AwcLDxMXGx8jJTk8AUFHKy8zNzs/Q0dKb09RSUwBUVdXW19jZ2tud3N3e31ZXAFhZWlvg4eLj5OXm5+hcXV4AX2BhYmNkZenq6+xmZ2hpEQAAamtsbW5vcO1xcnN0EhMUABUWABdqdXZ3eHl6GBkaGxxTKXBYeDUm8QAAAABJRU5ErkJggg==" alt="Crates.io"></a>
    <a href="https://github.com/v3xlabs/sourcify"><img src="https://img.shields.io/badge/Repository-v3xlabs/sourcify-blue?style=flat" alt="Repository"></a>
    <a href="#"><img src="https://img.shields.io/badge/License-LGPL--3.0-hotpink?style=flat" alt="License: LGPL-3.0"></a>
</p>

## Supported APIs

- `sourcify::v2`: Sourcify v2 contract data API.
  API docs: <https://sourcify.dev/server/api-docs/swagger.json>
- `sourcify::four_byte`: Sourcify 4byte signature API.
  API docs: <https://api.4byte.sourcify.dev/api-docs/swagger.json>

## Quickstart

### Install

```toml
[dependencies]
sourcify = "0.0.2"
```

### Example

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

## Limitations

This crate is intentionally a read-only wrapper. It does not handle interactive contract verification or signature submission.
