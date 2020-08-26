use crate::ast;
use std::fs;
use std::io::Write;

pub fn write_assembly(tree: &ast::Program) {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("test.s")
        .expect("Failed to open file");

    //Write default boilerplate
    let tree = &tree.main;
    let name = &tree.name;

    writeln!(&mut file, "        .globl  {}", name)
        .expect("Failed to write to file");
    writeln!(&mut file, "{}: ", name)
        .expect("Failed to write to file");

    let tree = &tree.body;
    let ret_value = match &tree.return_value {
        ast::Expr::Const { int } => {
            int.val
        },
        _ => panic!("Invalid statement")
    };

    writeln!(&mut file, "        movl    ${}, %eax", ret_value)
        .expect("Failed to write to file");
    writeln!(&mut file, "        ret")
        .expect("Failed to write to file");
}