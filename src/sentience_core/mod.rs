pub mod ast;
pub mod canonicalizer;
pub mod executor;
pub mod hasher;
pub mod parser;
pub mod runtime;

use ast::SentienceTokenAst;
use canonicalizer::canonicalize;
use executor::execute;
use hasher::token_hash;
use parser::parse_program;
use runtime::ExecutionResult;
use runtime::Runtime;

/// Main Sentience Core API - matches SRAI specification
pub struct SentienceCore {
    runtime: Box<dyn Runtime>,
}

impl SentienceCore {
    pub fn new(runtime: Box<dyn Runtime>) -> Self {
        Self { runtime }
    }

    /// Parse Sentience DSL into typed AST
    pub fn parse(&self, src: &str) -> Result<SentienceTokenAst, String> {
        parse_program(src)
    }

    /// Canonicalize AST for deterministic processing
    pub fn canonicalize(&self, ast: &SentienceTokenAst) -> SentienceTokenAst {
        canonicalize(ast)
    }

    /// Generate deterministic token ID from canonical AST
    pub fn hash(&self, canon: &SentienceTokenAst) -> String {
        token_hash(canon)
    }

    /// Generate deterministic embedding vector
    pub fn embed(&self, canon: &SentienceTokenAst) -> Vec<f32> {
        // Symbolic encoder - deterministic ℝ^256
        symbolic_encoder::encode(canon)
    }

    /// Execute AST against runtime (Cortex + RefNet + Superego)
    pub fn execute(&mut self, ast: &SentienceTokenAst) -> Result<ExecutionResult, String> {
        execute(ast, &mut *self.runtime)
    }

    /// Complete pipeline: parse → canonicalize → hash → embed → execute
    pub fn process_step(&mut self, src: &str) -> Result<ExecutionResult, String> {
        let ast = self.parse(src)?;
        let canon = self.canonicalize(&ast);
        let token_id = self.hash(&canon);
        let embedding = self.embed(&canon);

        // Execute with runtime
        let mut result = self.execute(&canon)?;
        result.token_id = Some(token_id);
        result.embedding = Some(embedding);

        Ok(result)
    }
}

/// Symbolic encoder for deterministic embeddings
mod symbolic_encoder {
    use super::ast::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    pub fn encode(ast: &SentienceTokenAst) -> Vec<f32> {
        let mut embedding = vec![0.0; 256];

        // Hash token type
        let type_hash = hash_string(&ast.ttype.to_string());
        distribute_hash(type_hash, &mut embedding, 0);

        // Hash fields
        for (i, field) in ast.fields.iter().enumerate() {
            let field_hash =
                hash_string(&format!("{}:{}", field.key, value_to_string(&field.value)));
            distribute_hash(field_hash, &mut embedding, (i + 1) * 10);
        }

        // Hash children
        for (i, child) in ast.children.iter().enumerate() {
            let child_hash = hash_string(&child.ttype.to_string());
            distribute_hash(child_hash, &mut embedding, (i + 1) * 20);
        }

        // Normalize to unit length
        normalize_vector(&mut embedding);
        embedding
    }

    fn hash_string(s: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    }

    fn distribute_hash(hash: u64, embedding: &mut [f32], offset: usize) {
        for i in 0..8 {
            let idx = (offset + i) % embedding.len();
            let val = ((hash >> (i * 8)) & 0xFF) as f32 / 255.0;
            embedding[idx] += val * 0.1;
        }
    }

    fn normalize_vector(vec: &mut [f32]) {
        let norm: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for val in vec.iter_mut() {
                *val /= norm;
            }
        }
    }

    fn value_to_string(value: &Value) -> String {
        match value {
            Value::Str(s) => s.clone(),
            Value::Num(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Path(path) => path.join("."),
            Value::List(list) => format!(
                "[{}]",
                list.iter()
                    .map(value_to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Value::Map(map) => format!(
                "{{{}}}",
                map.iter()
                    .map(|(k, v)| format!("{}:{}", k, value_to_string(v)))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        }
    }
}
