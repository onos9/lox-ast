use std::env::args;
use std::fs::File;
use std::io::{self, Write};

#[derive(Debug)]
struct TreeType {
    base_class_name: String,
    class_name: String,
    fields: Vec<String>,
}

pub fn generate_ast(output_dir: &String) -> io::Result<()> {
    let types = vec![
        "Binary   : Expr left, Token operator, Expr right",
        "Grouping : Expr expression",
        "Literal  : Object value",
        "Unary    : Token operator, Expr right",
    ];
    define_ast(output_dir, "Expr", types)?;
    Ok(())
}

fn define_ast(output_dir: &String, base_name: &str, types: Vec<&str>) -> io::Result<()> {
    let path = format!("{output_dir}/{}.rs", base_name.to_lowercase());
    let mut file = File::create(path)?;
    let mut tree_types: Vec<TreeType> = Vec::new();

    writeln!(file, "use crate::error::*;")?;
    writeln!(file, "use crate::token::*;")?;

    for ttype in types {
        let (base_class_name, args) = ttype.split_once(":").unwrap();
        let class_name = format!("{}{}", base_class_name.trim(), base_name);
        let ags_split = args.split(",");
        let mut fields: Vec<String> = Vec::new();

        for arg in ags_split {
            let (tpy, name) = arg.trim().split_once(" ").unwrap();
            let field: String;
            if tpy == "Token" {
                field = format!("{}: {}", name, tpy);
            } else {
                field = format!("{}: Box<{}>", name, tpy);
            }

            fields.push(field);
        }
        tree_types.push(TreeType {
            base_class_name: base_class_name.to_string(),
            class_name,
            fields,
        })
    }

    // Generate enum
    writeln!(file, "\npub enum {base_name} {{")?;
    for tt in &tree_types {
        writeln!(
            file,
            "    {}({}),",
            tt.base_class_name.trim(),
            tt.class_name.trim()
        )?;
    }
    writeln!(file, "}}\n")?;

    // Generate struct
    for tt in &tree_types {
        writeln!(file, "pub struct {} {{", tt.class_name)?;
        for f in &tt.fields {
            writeln!(file, "    {f},")?;
        }
        writeln!(file, "}}")?;
        writeln!(file, "")?;
    }
    writeln!(file, "")?;

    // Generate trait
    writeln!(file, "pub trait ExprVisitor<T> {{")?;
    for tt in &tree_types {
        writeln!(
            file,
            "    fn visitor_{}_{}(&self, expr: &{}) -> Result<T, LoxError>;",
            tt.base_class_name.trim().to_lowercase(),
            base_name.to_lowercase(),
            tt.class_name.trim()
        )?;
    }
    writeln!(file, "}}\n")?;

    // Generate impl
    for tt in &tree_types {
        writeln!(file, "pub impl {} {{", tt.class_name)?;
        writeln!(
            file,
            "    fn eccept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {{"
        )?;
        writeln!(
            file,
            "       visitor.visitor_{}_{}(self)",
            tt.base_class_name.trim().to_lowercase(),
            base_name.to_lowercase(),
        )?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}")?;
        writeln!(file, "")?;
    }
    writeln!(file, "")?;

    Ok(())
}