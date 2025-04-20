pub struct PropertyDeclaration {
    name: String,
    c_type: String,
}

pub struct PropertyValue {
    name: String,
    value: Value,
}

pub enum Value {
    Literal {
        value: String,
    },
    StructValue {
        properties: Vec<PropertyValue>
    },
    Array {
        values: Vec<Value>,
        hint_array_width: Option<u32>
    }
}

pub enum AstNode {
    Include {
        filename: String,
    },
    Define {
        name: String,
        value: String,
    },
    StructDeclaration {
        name: String,
        properties: Vec<PropertyDeclaration>,
    },
    Const {
        name: String,
        c_type: String,
        value: Value
    }
}

pub fn generate(ast: Vec<AstNode>) -> String {
    let mut output = String::new();

    for statement in ast {
        match statement {
            AstNode::Include { filename } => {
                output.push_str(format!("#include \"{}\"\n", filename).as_str());
            }
            AstNode::Define { name, value } => {
                output.push_str(format!("#define {} {}\n", name, value).as_str())
            },
            AstNode::StructDeclaration { name, properties } => {
                let properties_str = generate_property_declarations(&properties);
                output.push_str(format!("typedef struct {name} {{\n{properties_str}}} {name};\n").as_str());
            },
            AstNode::Const { name, c_type, value } => {
                let value_str = generate_value(&value, 1);
                if let Value::Array { values, .. } = value {
                    let size = values.len();
                    output.push_str(format!("const {c_type} {name}[{size}] = {value_str};\n").as_str());
                } else {
                    output.push_str(format!("const {c_type} {name} = {value_str};\n").as_str());
                }
            },
        }
    }

    output
}

fn generate_value(value: &Value, nesting: usize) -> String {
    match value {
        Value::Literal { value } => value.clone(),
        Value::StructValue { properties } => generate_property_values(properties, nesting),
        Value::Array { values, hint_array_width } => generate_array_values(values, hint_array_width),
    }
}

fn generate_property_declarations(properties: &Vec<PropertyDeclaration>) -> String {
    let mut output = String::new();

    for property in properties {
        let PropertyDeclaration { c_type, name } = property;
        output.push_str(format!("\t{c_type} {name};\n").as_str());
    }

    output
}

fn generate_property_values(properties: &Vec<PropertyValue>, nesting: usize) -> String {
    let mut output = String::from("{\n");
    let indentation = "\t".repeat(nesting);
    
    for property in properties {
        let PropertyValue { name, value } = property;
        let value_str = generate_value(value, nesting + 1);
        output.push_str(format!("{indentation}.{name} = {value_str},\n").as_str());
    }
    output.push_str("\t".repeat(nesting - 1).as_str());
    output.push_str("}");

    output
}

fn generate_array_values(values: &Vec<Value>, hint_array_width: &Option<u32>) -> String {
    let mut output = String::from("{\n");

    if let Some(width) = hint_array_width {
        let rows = values.chunks(*width as usize);
        for row in rows {
            let mapped = row.iter().map(|v| generate_value(v, 1)).collect::<Vec<String>>();
            output.push_str("\t");
            output.push_str(mapped.join(",").as_str());
            output.push_str(",\n");
        }
    } else {
        let mapped = values.iter().map(|v| generate_value(v, 1)).collect::<Vec<String>>();
        output.push_str("\t");
        output.push_str(mapped.join(",").as_str());
        output.push_str(",\n");
    }

    output.push_str("}");

    output
}

#[cfg(test)]
mod test {
    use super::*;

    fn include(filename: &str) -> AstNode {
        AstNode::Include { filename: filename.to_string() }
    }

    fn define(name: &str, value: &str) -> AstNode {
        AstNode::Define { name: name.to_string(), value: value.to_string() }
    }

    fn struct_dec(name: &str, properties: Vec<PropertyDeclaration>) -> AstNode {
        AstNode::StructDeclaration { name: name.to_string(), properties }
    }

    fn prop_dec(name: &str, c_type: &str) -> PropertyDeclaration {
        PropertyDeclaration { name: name.to_string(), c_type: c_type.to_string() }
    }

    fn const_dec(name: &str, c_type: &str, value: Value) -> AstNode {
        AstNode::Const { name: name.to_string(), c_type: c_type.to_string(), value }
    }

    fn literal(value: &str) -> Value {
        Value::Literal { value: value.to_string() }
    }

    fn struct_value(properties: Vec<PropertyValue>) -> Value {
        Value::StructValue { properties }
    }

    fn prop_value(name: &str, value: Value) -> PropertyValue {
        PropertyValue { name: name.to_string(), value }
    }

    fn array(values: Vec<Value>) -> Value {
        Value::Array { values, hint_array_width: None }
    }

    #[test]
    fn test_include() {
        let output = generate(vec![include("main.h")]);
        assert_eq!(output, "#include \"main.h\"\n");
    }

    #[test]
    fn test_define() {
        let output = generate(vec![define("ARRAY_SIZE", "5")]);
        assert_eq!(output, "#define ARRAY_SIZE 5\n");
    }

    #[test]
    fn test_struct() {
        let output = generate(vec![struct_dec("Portal", vec![
            prop_dec("x", "uint8_t"),
            prop_dec("y", "uint8_t"),
            prop_dec("target_x", "uint8_t"),
            prop_dec("target_y", "uint8_t"),
        ])]);

        let expected = 
"typedef struct Portal {
\tuint8_t x;
\tuint8_t y;
\tuint8_t target_x;
\tuint8_t target_y;
} Portal;
";

        assert_eq!(output, expected);
    }

    #[test]
    fn test_literals() {
        let output = generate(vec![const_dec("foo", "uint8_t", literal("5"))]);

        let expected = "const uint8_t foo = 5;\n";

        assert_eq!(output, expected);
    }

    #[test]
    fn test_struct_value() {
        let output = generate(vec![const_dec("portal", "Portal", struct_value(vec![
            prop_value("x", literal("1")),
            prop_value("y", literal("2")),
            prop_value("target_x", literal("3")),
            prop_value("target_y", literal("4")),
        ]))]);

        let expected =
"const Portal portal = {
\t.x = 1,
\t.y = 2,
\t.target_x = 3,
\t.target_y = 4,
};
";

        assert_eq!(output, expected);
    }

    #[test]
    fn test_nested_struct() {
        let output = generate(vec![const_dec("portal", "Portal", struct_value(vec![
            prop_value("nested", struct_value(vec![
                prop_value("x", literal("1")),
            ])),
        ]))]);

        let expected =
"const Portal portal = {
\t.nested = {
\t\t.x = 1,
\t},
};
";

        assert_eq!(output, expected);
    }

    #[test]
    fn test_array() {
        let output = generate(vec![const_dec("array", "int", array(vec![
            literal("1"),
            literal("2"),
            literal("3"),
        ]))]);

        let expected =
"const int array[3] = {
\t1,2,3,
};
";
        
        assert_eq!(output, expected);
    }
}