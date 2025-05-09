use anyhow::{Context, Error};
use tiled::Map;

use crate::codegen::{self, AstNode};

pub fn build_ast(map: &Map) -> anyhow::Result<(Vec<AstNode>, Vec<AstNode>)> {
    let (map_defines, map) = build_tile_data(map)?;
    Ok((map_defines, vec![map]))
}

fn build_tile_data(map: &Map) -> anyhow::Result<(Vec<AstNode>, AstNode)> {
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

    let name = map
        .source
        .file_stem()
        .context("Failed to retrieve file stem from map source")?
        .to_string_lossy()
        .to_string();
    let tile_data = codegen::AstNode::Const {
        c_type: "uint8_t".to_string(),
        name: format!("{name}_map"),
        value: codegen::Value::Array {
            values: array_values,
            hint_array_width: Some(width as u32),
        },
    };

    let header_data = vec![
        codegen::AstNode::Define {
            name: format!("{name}_WIDTH"),
            value: width.to_string(),
        },
        codegen::AstNode::Define {
            name: format!("{name}_HEIGHT"),
            value: height.to_string(),
        },
    ];

    Ok((header_data, tile_data))
}

#[cfg(test)]
mod test {
    use super::*;
    use tiled::Loader;

    #[test]
    fn test_map_array() -> Result<(), anyhow::Error> {
        let mut loader = Loader::new();
        let map = loader.load_tmx_map("samples/village.tmx")?;

        let (_header_ast, map) = build_tile_data(&map)?;
        let expected = include_str!("../samples/village_map.c");

        assert_eq!(expected, codegen::generate(vec![map]));

        Ok(())
    }
}
