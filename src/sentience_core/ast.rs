use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ThoughtType {
    Percept,
    Reflection,
    Action,
    Plan,
    Goal,
    SelfModel,
    Concept,
    Contradiction,
    Relation,
    Experience,
}

impl fmt::Display for ThoughtType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThoughtType::Percept => write!(f, "Percept"),
            ThoughtType::Reflection => write!(f, "Reflection"),
            ThoughtType::Action => write!(f, "Action"),
            ThoughtType::Plan => write!(f, "Plan"),
            ThoughtType::Goal => write!(f, "Goal"),
            ThoughtType::SelfModel => write!(f, "Self"),
            ThoughtType::Concept => write!(f, "Concept"),
            ThoughtType::Contradiction => write!(f, "Contradiction"),
            ThoughtType::Relation => write!(f, "Relation"),
            ThoughtType::Experience => write!(f, "Experience"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SentienceTokenAst {
    pub ttype: ThoughtType,
    pub fields: Vec<Field>,
    pub children: Vec<SentienceTokenAst>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub key: String,
    pub value: Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Str(String),
    Num(f64),
    Bool(bool),
    Path(Vec<String>), // e.g., ["percept", "text"]
    List(Vec<Value>),
    Map(Vec<(String, Value)>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Span {
    pub line: usize,
    pub col: usize,
    pub end_line: usize,
    pub end_col: usize,
}

impl SentienceTokenAst {
    pub fn new(ttype: ThoughtType, span: Span) -> Self {
        Self {
            ttype,
            fields: Vec::new(),
            children: Vec::new(),
            span,
        }
    }

    pub fn with_field(mut self, key: String, value: Value) -> Self {
        self.fields.push(Field { key, value });
        self
    }

    pub fn with_child(mut self, child: SentienceTokenAst) -> Self {
        self.children.push(child);
        self
    }

    pub fn get_field(&self, key: &str) -> Option<&Value> {
        self.fields.iter().find(|f| f.key == key).map(|f| &f.value)
    }

    pub fn get_field_str(&self, key: &str) -> Option<&str> {
        self.get_field(key).and_then(|v| match v {
            Value::Str(s) => Some(s.as_str()),
            _ => None,
        })
    }

    pub fn get_field_num(&self, key: &str) -> Option<f64> {
        self.get_field(key).and_then(|v| match v {
            Value::Num(n) => Some(*n),
            _ => None,
        })
    }

    pub fn get_field_bool(&self, key: &str) -> Option<bool> {
        self.get_field(key).and_then(|v| match v {
            Value::Bool(b) => Some(*b),
            _ => None,
        })
    }
}

impl Field {
    pub fn new(key: String, value: Value) -> Self {
        Self { key, value }
    }
}

impl Span {
    pub fn new(line: usize, col: usize, end_line: usize, end_col: usize) -> Self {
        Self {
            line,
            col,
            end_line,
            end_col,
        }
    }

    pub fn single_char(line: usize, col: usize) -> Self {
        Self {
            line,
            col,
            end_line: line,
            end_col: col + 1,
        }
    }
}

// Runtime token with provenance and metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SentienceToken {
    pub id: String,
    pub ast: SentienceTokenAst,
    pub embedding: Vec<f32>,
    pub provenance: Provenance,
    pub meta: TokenMeta,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Provenance {
    pub stm_ids: Vec<String>,
    pub refnet_id: String,
    pub rules_applied: Vec<String>,
    pub agent_id: String,
    pub step_id: u64,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenMeta {
    pub version: String,
    pub strength: f32,
    pub belief: f32,
    pub tags: Vec<String>,
}

impl SentienceToken {
    pub fn new(
        id: String,
        ast: SentienceTokenAst,
        embedding: Vec<f32>,
        provenance: Provenance,
        meta: TokenMeta,
    ) -> Self {
        Self {
            id,
            ast,
            embedding,
            provenance,
            meta,
        }
    }
}

// Edge between tokens
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edge {
    pub id: String,
    pub source_id: String,
    pub target_id: String,
    pub edge_type: EdgeType,
    pub weight: f32,
    pub timestamp: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EdgeType {
    About,
    Causes,
    Supports,
    Contradicts,
    DerivedFrom,
    AboutSelf,
    Temporal,
    Semantic,
    Structural,
}

impl fmt::Display for EdgeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EdgeType::About => write!(f, "ABOUT"),
            EdgeType::Causes => write!(f, "CAUSES"),
            EdgeType::Supports => write!(f, "SUPPORTS"),
            EdgeType::Contradicts => write!(f, "CONTRADICTS"),
            EdgeType::DerivedFrom => write!(f, "DERIVED_FROM"),
            EdgeType::AboutSelf => write!(f, "ABOUT_SELF"),
            EdgeType::Temporal => write!(f, "TEMPORAL"),
            EdgeType::Semantic => write!(f, "SEMANTIC"),
            EdgeType::Structural => write!(f, "STRUCTURAL"),
        }
    }
}

impl Edge {
    pub fn new(
        source_id: String,
        target_id: String,
        edge_type: EdgeType,
        weight: f32,
        timestamp: u64,
    ) -> Self {
        let id = format!(
            "edge_{}",
            hash_string(&format!(
                "{}|{}|{}",
                source_id,
                edge_type.to_string(),
                target_id
            ))
        );
        Self {
            id,
            source_id,
            target_id,
            edge_type,
            weight,
            timestamp,
        }
    }
}

fn hash_string(s: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    format!("{:x}", hasher.finish())[..16].to_string()
}
