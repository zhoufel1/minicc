use crate::parser::Program;

pub fn compile(ast: &Program) -> String {
    let mut res = String::new();
    let main = &ast.main;
    let name = main.name.to_string();
    res.push_str(&format!("    .globl {}\n", name)[..]);
    res.push_str("main:\n");
    let val = main.body.return_value.val;
    res.push_str(&format!("    mov ${}, %eax\n", val)[..]);
    res.push_str("    ret\n");
    res
}

// fn traverse_main(ast: &Program) -> String {

// }