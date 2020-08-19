use crate::parser::Program;

pub fn compile(ast: &Program) -> String {
    let main = &ast.main;
    let name = main.name.to_string();
    let val = main.body.return_value.val;

    format!("   .globl {n}
{n}:
    movl ${r}, %eax
    ret
    ", n=name, r=val)
}