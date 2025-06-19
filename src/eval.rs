use crate::context::AgentContext;
use crate::types::Statement;

fn eval_expr(expr: &str, input: &str, _ctx: &AgentContext) -> String {
    if expr == "input" {
        input.to_string()
    } else {
        expr.trim_matches('"').to_string()
    }
}

/// Evaluate a single AST statement in the given context.
pub fn eval(stmt: &Statement, indent: &str, input: &str, ctx: &mut AgentContext) {
    match stmt {
        Statement::AgentDeclaration { name, body } => {
            println!("Agent: {}", name);
            for inner in body.iter() {
                match inner {
                    Statement::MemDeclaration { target } => {
                        println!("  Init mem: {}", target);
                    }
                    Statement::Goal(text) => {
                        println!("  Goal: \"{}\"", text);
                    }
                    _ => {}
                }
            }
            ctx.current_agent = Some(stmt.clone());
            println!("Agent: {} [registered]", name);
        }
        Statement::MemDeclaration { .. } => {}
        Statement::OnInput { .. } => {}
        Statement::Reflect { body } => {
            let nested_indent = format!("{}  ", indent);
            for inner in body.iter() {
                eval(inner, &nested_indent, input, ctx);
            }
        }
        Statement::ReflectAccess { mem_target, key } => {
            let val = match mem_target.as_str() {
                "short" => ctx.get_mem("short", key),
                "long" => ctx.get_mem("long", key),
                _ => String::new(),
            };
            ctx.output = Some(val.clone());
            println!("{}{}", indent, val);
        }
        Statement::Train { .. } => {}
        Statement::Evolve { .. } => {}
        Statement::Goal(_) => {}
        Statement::Embed { .. } => {}
        Statement::IfContextIncludes { values, body } => {
            let current_val = ctx.get_mem("short", "msg");
            for v in values.iter() {
                if current_val.contains(v) {
                    for inner in body.iter() {
                        eval(inner, indent, input, ctx);
                    }
                    break;
                }
            }
        }
        Statement::Print(text) => {
            println!("{}{}", indent, text);
        }
        Statement::Assignment(name, expr) => {
            if name == "output" {
                let val = eval_expr(expr, input, ctx);
                ctx.output = Some(val.clone());
                return;
            }

            let val = eval_expr(expr, input, ctx);
            ctx.set_mem("short", name, &val);
        }
    }
}
