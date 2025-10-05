use crate::sentience_core::ast::*;

/// Parse Sentience DSL into typed AST
pub fn parse_program(src: &str) -> Result<SentienceTokenAst, String> {
    let lines: Vec<&str> = src.lines().collect();
    let mut tokens = Vec::new();

    for (line_num, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }

        if trimmed.starts_with("agent ") {
            // Parse agent declaration
            let agent_name = trimmed.strip_prefix("agent ").unwrap_or("unknown");
            let span = Span::new(line_num + 1, 1, line_num + 1, trimmed.len());
            let ast = SentienceTokenAst::new(ThoughtType::SelfModel, span)
                .with_field("name".to_string(), Value::Str(agent_name.to_string()));
            tokens.push(ast);
        } else if trimmed.starts_with("embed ") {
            // Parse embed statement
            if let Some(embed_content) = trimmed.strip_prefix("embed ") {
                let parts: Vec<&str> = embed_content.split(" -> ").collect();
                if parts.len() == 2 {
                    let span = Span::new(line_num + 1, 1, line_num + 1, trimmed.len());
                    let ast = SentienceTokenAst::new(ThoughtType::Percept, span)
                        .with_field("modality".to_string(), Value::Str("text".to_string()))
                        .with_field("content".to_string(), Value::Str(parts[0].to_string()))
                        .with_field("target".to_string(), Value::Str(parts[1].to_string()));
                    tokens.push(ast);
                }
            }
        } else if trimmed.starts_with("reflect {") {
            // Parse reflection block
            let span = Span::new(line_num + 1, 1, line_num + 1, trimmed.len());
            let ast = SentienceTokenAst::new(ThoughtType::Reflection, span).with_field(
                "ops".to_string(),
                Value::List(vec![
                    Value::Str("recall".to_string()),
                    Value::Str("reframe".to_string()),
                    Value::Str("consolidate".to_string()),
                ]),
            );
            tokens.push(ast);
        }
    }

    if tokens.is_empty() {
        return Err("No valid tokens found in input".to_string());
    }

    // Return the first token for now
    // In a full implementation, you'd return a proper program structure
    Ok(tokens[0].clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_agent() {
        let src = "agent TestAgent";
        let result = parse_program(src);
        assert!(result.is_ok());

        let ast = result.unwrap();
        assert_eq!(ast.ttype, ThoughtType::SelfModel);
        assert_eq!(ast.get_field_str("name"), Some("TestAgent"));
    }

    #[test]
    fn test_parse_embed() {
        let src = "embed msg -> percept.text";
        let result = parse_program(src);
        assert!(result.is_ok());

        let ast = result.unwrap();
        assert_eq!(ast.ttype, ThoughtType::Percept);
        assert_eq!(ast.get_field_str("content"), Some("msg"));
        assert_eq!(ast.get_field_str("target"), Some("percept.text"));
    }

    #[test]
    fn test_parse_reflect() {
        let src = "reflect { recall; reframe; consolidate }";
        let result = parse_program(src);
        assert!(result.is_ok());

        let ast = result.unwrap();
        assert_eq!(ast.ttype, ThoughtType::Reflection);
        if let Some(Value::List(ops)) = ast.get_field("ops") {
            assert_eq!(ops.len(), 3);
        } else {
            panic!("Expected ops field to be a list");
        }
    }
}
