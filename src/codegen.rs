struct PropertyDeclaration {
    name: String,
    c_type: String,
}

struct PropertyValue {
    name: String,
    value: String,
}

enum Value {
    Literal {
        value: String,
    },
    StructValue {
        properties: Vec<PropertyValue>
    }
}

enum Statement {
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

fn generate(ast: Vec<Statement>) -> String {
    let mut output = String::new();

    for statement in ast {
        match statement {
            Statement::Include { filename } => {
                output.push_str(format!("#include \"{}\"\n", filename).as_str());
            }
            Statement::Define { name, value } => {
                output.push_str(format!("#define {} {}\n", name, value).as_str())
            },
            Statement::StructDeclaration { name, properties } => {
                let properties_str = generate_property_declarations(properties);
                output.push_str(format!("typedef struct {name} {{\n{properties_str}}} {name};\n").as_str());
            },
            Statement::Const { name, c_type, value } => todo!(),
        }
    }

    output
}

fn generate_property_declarations(properties: Vec<PropertyDeclaration>) -> String {
    let mut output = String::new();

    for property in properties {
        let PropertyDeclaration { c_type, name } = property;
        output.push_str(format!("    {c_type} {name};\n").as_str());
    }

    output
}

#[cfg(test)]
mod test {
    use super::*;

    fn include(filename: &str) -> Statement {
        Statement::Include { filename: filename.to_string() }
    }

    fn define(name: &str, value: &str) -> Statement {
        Statement::Define { name: name.to_string(), value: value.to_string() }
    }

    fn struct_dec(name: &str, properties: Vec<PropertyDeclaration>) -> Statement {
        return Statement::StructDeclaration { name: name.to_string(), properties }
    }

    fn prop_dec(name: &str, c_type: &str) -> PropertyDeclaration {
        return PropertyDeclaration { name: name.to_string(), c_type: c_type.to_string() }
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
r#"typedef struct Portal {
    uint8_t x;
    uint8_t y;
    uint8_t target_x;
    uint8_t target_y;
} Portal;
"#;

        assert_eq!(output, expected);
    }
}