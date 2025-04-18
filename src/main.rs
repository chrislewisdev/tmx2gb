use anyhow::{Context, Error};
use tiled::Loader;

mod codegen;

fn main() {
    if let Err(e) = cli() {
        eprintln!("Error: {:#}", e);
    }
}

fn cli() -> anyhow::Result<()> {
    let mut loader = Loader::new();
    let map = loader.load_tmx_map("samples/village.tmx")?;

    let tile_layers: Vec<_> = map.layers().filter_map(|l| l.as_tile_layer()).collect();
    if tile_layers.len() != 1 {
        return Err(Error::msg("Exactly one tile layer is required"));
    }

    let tiles = tile_layers.get(0).context("Should have 1 element")?;
    let width = tiles.width().context("Map must be finite")? as i32;
    let height = tiles.height().context("Map must be finite")? as i32;
    let mut array_values: Vec<codegen::Value> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let tile = tiles.get_tile(x, y).context("Failed to get tile")?;
            let tile_index = tile.id().to_string();
            array_values.push(codegen::Value::Literal { value: tile_index });
        }
    }

    let ast = vec![codegen::AstNode::Const {
        c_type: "uint8_t".to_string(),
        name: "village_tiles".to_string(),
        value: codegen::Value::Array {
            values: array_values
        }
    }];
    let output = codegen::generate(ast);

    println!("{}", output);

    Ok(())
}
