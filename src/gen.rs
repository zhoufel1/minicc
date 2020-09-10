use crate::ast;
use std::fs;
use std::path::Path;
use std::io::Write;

pub fn generate_assembly(tree: &ast::Program) {
    if Path::new("result.s").exists() {
        fs::remove_file("result.s").expect("Failed to remove file");
    }

    let file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("result.s")
        .expect("Failed to open file");

    //Write default boilerplate
    let tree = &tree.main;
    emit_function(&file, &tree);

    let tree = &tree.body;
    emit_return(&file, &tree);
}

fn emit_function(mut file: &fs::File, tree: &ast::Function) {
    let name = &tree.name;

    writeln!(&mut file, "        .globl  _{}", name)
        .expect("Failed to write to file");
    writeln!(&mut file, "_{}: ", name)
        .expect("Failed to write to file");
}

fn emit_return(mut file: &fs::File, tree: &ast::Return) {
    match &tree.return_value {
        ast::Expr::Const { int } => {
            emit_int(&mut file, &int);
        },
        ast::Expr::Expression { op, expr } => {
            emit_unary_op(&mut file, &op, &expr);
        }
    }
    writeln!(&mut file, "        ret")
        .expect("Failed to write to file");
}

fn emit_int(mut file: &fs::File, int: &ast::Int) {
    writeln!(&mut file, "        movl    ${}, %eax", int.val)
        .expect("Failed to write to file");
}

fn emit_unary_op(mut file: &fs::File, operator: &ast::UnaryOp, expression: &ast::Expr) {
    match expression {
        ast::Expr::Const { int } => {
            emit_int(file, int);
        },
        ast::Expr::Expression { op, expr } => {
            emit_unary_op(&mut file, &op, &expr);
        }
    }
    match &operator.val[..] {
        "-" => writeln!(&mut file, "        neg     %eax")
            .expect("Failed to write to file"),
        "~" => writeln!(&mut file, "        not     %eax")
            .expect("Failed to write to file"),
        "!" => {
            writeln!(&mut file, "        cmp     $0, %eax").
                expect("Failed to write to file");
            writeln!(&mut file, "        movl    $0, %eax").
                expect("Failed to write to file");
            writeln!(&mut file, "        sete    %al").
                expect("Failed to write to file");
        }
        _ => ()
    }
}
