use crate::ast;
use std::fs;
use std::io::Write;

pub fn write_assembly(tree: &ast::Program) {
    let file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("result.s")
        .expect("Failed to open file");

    //Write default boilerplate
    let tree = &tree.main;
    generate_function(&file, &tree);

    let tree = &tree.body;
    generate_return(&file, &tree);
}

fn generate_function(mut file: &fs::File, tree: &ast::Function) {
    let name = &tree.name;

    writeln!(&mut file, "        .globl  {}", name)
        .expect("Failed to write to file");
    writeln!(&mut file, "{}: ", name)
        .expect("Failed to write to file");
}

fn generate_return(mut file: &fs::File, tree: &ast::Return) {
    match &tree.return_value {
        ast::Expr::Const { int } => {
            generate_int(&mut file, &int);
            writeln!(&mut file, "        ret")
                .expect("Failed to write to file");
        },
        ast::Expr::Expression { op, expr } => {
            match &**expr {
                ast::Expr::Const { int } => {
                    generate_unary_op(&mut file, &op, &expr);
                    writeln!(&mut file, "        ret")
                        .expect("Failed to write to file");
                },
                _ => (), //TODO Recurse
            }
        }
    }
}

fn generate_int(mut file: &fs::File, int: &ast::Int) {
    writeln!(&mut file, "        movl    ${}, %eax", int.val)
        .expect("Failed to write to file");
}

fn generate_unary_op(mut file: &fs::File, op: &ast::UnaryOp, expr: &ast::Expr) {
    match expr {
        ast::Expr::Const { int} => {
            println!("Exe");
            generate_int(file, int);
            match &op.val[..] {
                "-" => writeln!(&mut file, "        neg     %eax")
                    .expect("Failed to write to file"),
                "!" => writeln!(&mut file, "        not     %eax")
                    .expect("Failed to write to file"),
                _ => ()
            }
        },
        ast::Expr::Expression { .. } => {
            ()
        }
    }
}
