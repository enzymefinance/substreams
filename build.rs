use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("Dispatcher", "abi/dispatcher.json")?
        .generate()?
        .write_to_file("src/abi/dispatcher.rs")?;

    Abigen::new("ERC20", "abi/erc20.json")?
        .generate()?
        .write_to_file("src/abi/erc20.rs")?;

    Ok(())
}
