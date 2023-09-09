mod generate;
use generate::*;

use std::io;

fn main() -> io::Result<()> {
    generate_ast(&"src".to_string())
}
