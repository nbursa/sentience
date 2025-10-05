use crate::sentience_core::ast::*;
use std::collections::BTreeMap;

/// Canonicalize AST for deterministic processing
pub fn canonicalize(ast: &SentienceTokenAst) -> SentienceTokenAst {
    SentienceTokenAst {
        ttype: ast.ttype.clone(),
        fields: canonicalize_fields(&ast.fields),
        children: ast.children.iter().map(canonicalize).collect(),
        span: ast.span.clone(), // Keep span for debugging
    }
}

fn canonicalize_fields(fields: &[Field]) -> Vec<Field> {
    let mut field_map: BTreeMap<String, Value> = BTreeMap::new();

    for field in fields {
        field_map.insert(field.key.clone(), canonicalize_value(&field.value));
    }

    field_map
        .into_iter()
        .map(|(key, value)| Field::new(key, value))
        .collect()
}

fn canonicalize_value(value: &Value) -> Value {
    match value {
        Value::Str(s) => Value::Str(normalize_string(s)),
        Value::Num(n) => Value::Num(normalize_number(*n)),
        Value::Bool(b) => Value::Bool(*b),
        Value::Path(path) => Value::Path(path.iter().map(|s| normalize_string(s)).collect()),
        Value::List(list) => Value::List(list.iter().map(canonicalize_value).collect()),
        Value::Map(map) => {
            let mut canonical_map: BTreeMap<String, Value> = BTreeMap::new();
            for (key, val) in map {
                canonical_map.insert(normalize_string(key), canonicalize_value(val));
            }
            Value::Map(canonical_map.into_iter().collect())
        }
    }
}

fn normalize_string(s: &str) -> String {
    // Normalize Unicode to NFC
    let normalized: String = unicode_normalization::UnicodeNormalization::nfc(s).collect();

    // Normalize whitespace and escape sequences
    normalized
        .replace("\\n", "\n")
        .replace("\\t", "\t")
        .replace("\\\"", "\"")
        .replace("\\\\", "\\")
        .trim()
        .to_string()
}

fn normalize_number(n: f64) -> f64 {
    // Normalize floating point numbers
    if n.fract() == 0.0 {
        n.trunc() // Convert 1.0 to 1
    } else {
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonicalize_fields_order() {
        let fields1 = vec![
            Field::new("b".to_string(), Value::Str("value2".to_string())),
            Field::new("a".to_string(), Value::Str("value1".to_string())),
        ];

        let fields2 = vec![
            Field::new("a".to_string(), Value::Str("value1".to_string())),
            Field::new("b".to_string(), Value::Str("value2".to_string())),
        ];

        let canon1 = canonicalize_fields(&fields1);
        let canon2 = canonicalize_fields(&fields2);

        assert_eq!(canon1, canon2);
        assert_eq!(canon1[0].key, "a");
        assert_eq!(canon1[1].key, "b");
    }

    #[test]
    fn test_normalize_string() {
        assert_eq!(normalize_string("  hello  "), "hello");
        assert_eq!(normalize_string("\\n\\t"), "\n\t");
        assert_eq!(normalize_string("\\\"quoted\\\""), "\"quoted\"");
    }

    #[test]
    fn test_normalize_number() {
        assert_eq!(normalize_number(1.0), 1.0);
        assert_eq!(normalize_number(1.5), 1.5);
        assert_eq!(normalize_number(2.0), 2.0);
    }
}
