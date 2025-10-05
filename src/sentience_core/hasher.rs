use crate::sentience_core::ast::SentienceTokenAst;
use serde_json;
use sha2::{Digest, Sha256};

/// Generate deterministic token ID from canonical AST
pub fn token_hash(canon_ast: &SentienceTokenAst) -> String {
    // Convert to canonical JSON
    let json = serde_json::to_string(canon_ast).expect("Failed to serialize AST to JSON");

    // Add schema version for future compatibility
    let versioned_json = format!("{{\"schema\":\"sentience/0.2\",\"ast\":{}}}", json);

    // Generate SHA-256 hash
    let mut hasher = Sha256::new();
    hasher.update(versioned_json.as_bytes());
    let hash_bytes = hasher.finalize();

    // Return shortened hex ID with prefix
    format!("mem_{}", hex::encode(&hash_bytes[..8]))
}

/// Generate edge ID from source, relation, and target
pub fn edge_hash(source_id: &str, edge_type: &str, target_id: &str) -> String {
    let input = format!("{}|{}|{}", source_id, edge_type, target_id);
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let hash_bytes = hasher.finalize();
    format!("edge_{}", hex::encode(&hash_bytes[..8]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sentience_core::ast::*;

    #[test]
    fn test_deterministic_hashing() {
        let span = Span::new(1, 1, 1, 10);
        let ast1 = SentienceTokenAst::new(ThoughtType::Percept, span.clone())
            .with_field("modality".to_string(), Value::Str("text".to_string()))
            .with_field("content".to_string(), Value::Str("hello".to_string()));

        let ast2 = SentienceTokenAst::new(ThoughtType::Percept, span)
            .with_field("modality".to_string(), Value::Str("text".to_string()))
            .with_field("content".to_string(), Value::Str("hello".to_string()));

        let hash1 = token_hash(&ast1);
        let hash2 = token_hash(&ast2);

        assert_eq!(hash1, hash2);
        assert!(hash1.starts_with("mem_"));
        assert_eq!(hash1.len(), 12); // "mem_" + 8 hex chars
    }

    #[test]
    fn test_different_ast_different_hash() {
        let span = Span::new(1, 1, 1, 10);
        let ast1 = SentienceTokenAst::new(ThoughtType::Percept, span.clone())
            .with_field("modality".to_string(), Value::Str("text".to_string()));

        let ast2 = SentienceTokenAst::new(ThoughtType::Percept, span)
            .with_field("modality".to_string(), Value::Str("audio".to_string()));

        let hash1 = token_hash(&ast1);
        let hash2 = token_hash(&ast2);

        assert_ne!(hash1, hash2);
    }
}
