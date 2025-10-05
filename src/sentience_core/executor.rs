use crate::sentience_core::ast::*;
use crate::sentience_core::runtime::*;
use std::time::{SystemTime, UNIX_EPOCH};

/// Execute Sentience AST against runtime
pub fn execute(
    ast: &SentienceTokenAst,
    runtime: &mut dyn Runtime,
) -> Result<ExecutionResult, String> {
    let mut result = ExecutionResult::new();

    // Get STM window for RefNet evaluation
    let stm = runtime.cortex().stm(10);

    // Evaluate with RefNet
    let metrics = runtime.refnet().evaluate(&stm);
    result.metrics = Some(metrics.clone());

    // Execute based on token type
    match ast.ttype {
        ThoughtType::Percept => execute_percept(ast, &mut result)?,
        ThoughtType::Reflection => execute_reflection(ast, &mut result, &metrics)?,
        ThoughtType::Action => execute_action(ast, &mut result)?,
        ThoughtType::Concept => execute_concept(ast, &mut result)?,
        ThoughtType::SelfModel => execute_self_model(ast, &mut result)?,
        _ => return Err(format!("Unsupported token type: {:?}", ast.ttype)),
    }

    // Apply Superego gating
    for token in &mut result.tokens {
        let verdict = runtime.superego().judge(token, &metrics);
        match verdict {
            Verdict::Allow => {
                // Commit to Cortex
                let commit_id = runtime.cortex().commit(token, &result.edges)?;
                token.id = commit_id;
            }
            Verdict::Modify(modified_token) => {
                *token = modified_token;
                let commit_id = runtime.cortex().commit(token, &result.edges)?;
                token.id = commit_id;
            }
            Verdict::Defer => {
                // Skip this token for now
                continue;
            }
            Verdict::Block(reason) => {
                return Err(format!("Token blocked by Superego: {}", reason));
            }
        }
    }

    Ok(result)
}

fn execute_percept(ast: &SentienceTokenAst, result: &mut ExecutionResult) -> Result<(), String> {
    let modality = ast.get_field_str("modality").unwrap_or("unknown");
    let content = ast.get_field_str("content").unwrap_or("");

    let token = create_percept_token(modality, content, &ast.span)?;
    result.tokens.push(token);

    Ok(())
}

fn execute_reflection(
    ast: &SentienceTokenAst,
    result: &mut ExecutionResult,
    metrics: &RefMetrics,
) -> Result<(), String> {
    let empty_list = Vec::new();
    let ops = ast
        .get_field("ops")
        .and_then(|v| match v {
            Value::List(list) => Some(list),
            _ => None,
        })
        .unwrap_or(&empty_list);

    let mut reflection_ops = Vec::new();
    for op in ops {
        if let Value::Str(op_str) = op {
            reflection_ops.push(op_str.clone());
        }
    }

    let token = create_reflection_token(reflection_ops, metrics, &ast.span)?;
    result.tokens.push(token);

    // Generate edges to STM tokens
    // This would be more sophisticated in a real implementation
    Ok(())
}

fn execute_action(ast: &SentienceTokenAst, result: &mut ExecutionResult) -> Result<(), String> {
    let action_name = ast.get_field_str("name").unwrap_or("unknown");
    let target = ast.get_field_str("target").unwrap_or("");

    let token = create_action_token(action_name, target, &ast.span)?;
    result.tokens.push(token);

    Ok(())
}

fn execute_self_model(ast: &SentienceTokenAst, result: &mut ExecutionResult) -> Result<(), String> {
    let name = ast.get_field_str("name").unwrap_or("unknown");

    let token = create_self_model_token(name, &ast.span)?;
    result.tokens.push(token);

    Ok(())
}

fn execute_concept(ast: &SentienceTokenAst, result: &mut ExecutionResult) -> Result<(), String> {
    let summary = ast.get_field_str("summary").unwrap_or("");
    let empty_list = Vec::new();
    let from_tokens = ast
        .get_field("from")
        .and_then(|v| match v {
            Value::List(list) => Some(list),
            _ => None,
        })
        .unwrap_or(&empty_list);

    let mut source_ids = Vec::new();
    for token_ref in from_tokens {
        if let Value::Str(id) = token_ref {
            source_ids.push(id.clone());
        }
    }

    let token = create_concept_token(summary, source_ids, &ast.span)?;
    result.tokens.push(token.clone());

    // Generate DERIVED_FROM edges
    if let Some(Value::List(from_list)) = token.ast.get_field("from") {
        for source_id in from_list {
            if let Value::Str(src_id) = source_id {
                let edge = Edge::new(
                    src_id.clone(),
                    token.id.clone(),
                    EdgeType::DerivedFrom,
                    1.0,
                    current_timestamp(),
                );
                result.edges.push(edge);
            }
        }
    }

    Ok(())
}

// Token creation helpers
fn create_self_model_token(name: &str, span: &Span) -> Result<SentienceToken, String> {
    let ast = SentienceTokenAst::new(ThoughtType::SelfModel, span.clone())
        .with_field("name".to_string(), Value::Str(name.to_string()));

    let embedding = vec![0.0; 256]; // Placeholder
    let provenance = create_provenance();
    let meta = TokenMeta {
        version: "sentience/0.2".to_string(),
        strength: 1.0,
        belief: 1.0,
        tags: vec!["self_model".to_string()],
    };

    Ok(SentienceToken::new(
        "temp_id".to_string(),
        ast,
        embedding,
        provenance,
        meta,
    ))
}

fn create_percept_token(
    modality: &str,
    content: &str,
    span: &Span,
) -> Result<SentienceToken, String> {
    let ast = SentienceTokenAst::new(ThoughtType::Percept, span.clone())
        .with_field("modality".to_string(), Value::Str(modality.to_string()))
        .with_field("content".to_string(), Value::Str(content.to_string()));

    let embedding = vec![0.0; 256]; // Placeholder
    let provenance = create_provenance();
    let meta = TokenMeta {
        version: "sentience/0.2".to_string(),
        strength: 1.0,
        belief: 1.0,
        tags: vec!["percept".to_string()],
    };

    Ok(SentienceToken::new(
        "temp_id".to_string(), // Will be replaced by Cortex
        ast,
        embedding,
        provenance,
        meta,
    ))
}

fn create_reflection_token(
    ops: Vec<String>,
    metrics: &RefMetrics,
    span: &Span,
) -> Result<SentienceToken, String> {
    let ast = SentienceTokenAst::new(ThoughtType::Reflection, span.clone()).with_field(
        "ops".to_string(),
        Value::List(ops.into_iter().map(Value::Str).collect()),
    );

    let embedding = vec![0.0; 256]; // Placeholder
    let provenance = create_provenance();
    let meta = TokenMeta {
        version: "sentience/0.2".to_string(),
        strength: metrics.quality,
        belief: metrics.quality,
        tags: vec!["reflection".to_string()],
    };

    Ok(SentienceToken::new(
        "temp_id".to_string(),
        ast,
        embedding,
        provenance,
        meta,
    ))
}

fn create_action_token(name: &str, target: &str, span: &Span) -> Result<SentienceToken, String> {
    let ast = SentienceTokenAst::new(ThoughtType::Action, span.clone())
        .with_field("name".to_string(), Value::Str(name.to_string()))
        .with_field("target".to_string(), Value::Str(target.to_string()));

    let embedding = vec![0.0; 256]; // Placeholder
    let provenance = create_provenance();
    let meta = TokenMeta {
        version: "sentience/0.2".to_string(),
        strength: 1.0,
        belief: 1.0,
        tags: vec!["action".to_string()],
    };

    Ok(SentienceToken::new(
        "temp_id".to_string(),
        ast,
        embedding,
        provenance,
        meta,
    ))
}

fn create_concept_token(
    summary: &str,
    from_tokens: Vec<String>,
    span: &Span,
) -> Result<SentienceToken, String> {
    let ast = SentienceTokenAst::new(ThoughtType::Concept, span.clone())
        .with_field("summary".to_string(), Value::Str(summary.to_string()))
        .with_field(
            "from".to_string(),
            Value::List(from_tokens.into_iter().map(Value::Str).collect()),
        );

    let embedding = vec![0.0; 256]; // Placeholder
    let provenance = create_provenance();
    let meta = TokenMeta {
        version: "sentience/0.2".to_string(),
        strength: 0.8,
        belief: 0.8,
        tags: vec!["concept".to_string()],
    };

    Ok(SentienceToken::new(
        "temp_id".to_string(),
        ast,
        embedding,
        provenance,
        meta,
    ))
}

fn create_provenance() -> Provenance {
    Provenance {
        stm_ids: Vec::new(), // Will be populated by runtime
        refnet_id: "stub_refnet_v1".to_string(),
        rules_applied: Vec::new(),
        agent_id: "default_agent".to_string(),
        step_id: current_timestamp(),
        timestamp: current_timestamp(),
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
