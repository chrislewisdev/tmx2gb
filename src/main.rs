use anyhow::{Context, Error};
use tiled::{Loader, Map};

mod codegen;

fn main() {
    if let Err(e) = cli() {
        eprintln!("Error: {:#}", e);
    }
}

fn cli() -> anyhow::Result<()> {
    let mut loader = Loader::new();
    let map = loader.load_tmx_map("samples/village.tmx")?;

    let output = generate_map_array(&map)?;
    println!("{}", output);

    Ok(())
}

fn generate_map_array(map: &Map) -> anyhow::Result<String> {
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
            let tile_index = u8::try_from(tile.id()).context("Tile indices cannot exceed 255")?;
            array_values.push(codegen::Value::Uint8 { value: tile_index });
        }
    }

    let ast = vec![codegen::AstNode::Const {
        c_type: "uint8_t".to_string(),
        name: "village_tiles".to_string(),
        value: codegen::Value::Array {
            values: array_values,
            hint_array_width: Some(width as u32)
        }
    }];
    let output = codegen::generate(ast);

    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_map_array() -> Result<(), anyhow::Error> {
        let mut loader = Loader::new();
        let map = loader.load_tmx_map("samples/village.tmx")?;

        let code = generate_map_array(&map)?;
        let expected = include_str!("../samples/village_map.c");

        assert_eq!(expected, code);

        Ok(())
    }
}