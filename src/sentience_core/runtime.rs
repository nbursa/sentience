use crate::sentience_core::ast::*;
use std::collections::HashMap;

/// Execution result from Sentience Core
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub tokens: Vec<SentienceToken>,
    pub edges: Vec<Edge>,
    pub token_id: Option<String>,
    pub embedding: Option<Vec<f32>>,
    pub metrics: Option<RefMetrics>,
}

impl ExecutionResult {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            edges: Vec::new(),
            token_id: None,
            embedding: None,
            metrics: None,
        }
    }
}

/// RefNet evaluation metrics
#[derive(Debug, Clone)]
pub struct RefMetrics {
    pub valence: f32,
    pub smd: f32,
    pub quality: f32,
    pub next_action: String,
    pub action_logits: HashMap<String, f32>,
}

/// Superego verdict for token gating
#[derive(Debug, Clone)]
pub enum Verdict {
    Allow,
    Modify(SentienceToken),
    Defer,
    Block(String), // reason
}

/// Runtime trait for SRAI integration
pub trait Runtime: Send {
    fn cortex(&mut self) -> &mut dyn Cortex;
    fn refnet(&self) -> &dyn RefNet;
    fn superego(&self) -> &dyn Superego;
}

/// Cortex interface for memory operations
pub trait Cortex {
    fn commit(&mut self, token: &SentienceToken, edges: &[Edge]) -> Result<String, String>;
    fn recall_similar(&self, vec: &[f32], k: usize) -> Vec<TokenRef>;
    fn stm(&self, n: usize) -> Vec<TokenRef>;
    fn get_token(&self, id: &str) -> Option<SentienceToken>;
}

/// RefNet interface for cognitive evaluation
pub trait RefNet {
    fn evaluate(&self, stm: &[TokenRef]) -> RefMetrics;
}

/// Superego interface for alignment gating
pub trait Superego {
    fn judge(&self, token: &SentienceToken, metrics: &RefMetrics) -> Verdict;
}

/// Token reference for efficient memory operations
#[derive(Debug, Clone)]
pub struct TokenRef {
    pub id: String,
    pub ttype: ThoughtType,
    pub embedding: Vec<f32>,
}

impl TokenRef {
    pub fn new(id: String, ttype: ThoughtType, embedding: Vec<f32>) -> Self {
        Self {
            id,
            ttype,
            embedding,
        }
    }
}

/// In-memory Cortex implementation for testing
pub struct InMemoryCortex {
    tokens: HashMap<String, SentienceToken>,
    edges: HashMap<String, Edge>,
    stm_window: Vec<String>,
    max_stm_size: usize,
}

impl InMemoryCortex {
    pub fn new(max_stm_size: usize) -> Self {
        Self {
            tokens: HashMap::new(),
            edges: HashMap::new(),
            stm_window: Vec::new(),
            max_stm_size,
        }
    }
}

impl Cortex for InMemoryCortex {
    fn commit(&mut self, token: &SentienceToken, edges: &[Edge]) -> Result<String, String> {
        // Store token
        self.tokens.insert(token.id.clone(), token.clone());

        // Store edges
        for edge in edges {
            self.edges.insert(edge.id.clone(), edge.clone());
        }

        // Update STM window
        self.stm_window.push(token.id.clone());
        if self.stm_window.len() > self.max_stm_size {
            self.stm_window.remove(0);
        }

        Ok(token.id.clone())
    }

    fn recall_similar(&self, vec: &[f32], k: usize) -> Vec<TokenRef> {
        // Simple cosine similarity for now
        let mut similarities: Vec<(String, f32)> = Vec::new();

        for (id, token) in &self.tokens {
            let similarity = cosine_similarity(vec, &token.embedding);
            similarities.push((id.clone(), similarity));
        }

        // Sort by similarity and take top k
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        similarities
            .into_iter()
            .take(k)
            .map(|(id, _)| {
                let token = &self.tokens[&id];
                TokenRef::new(id, token.ast.ttype.clone(), token.embedding.clone())
            })
            .collect()
    }

    fn stm(&self, n: usize) -> Vec<TokenRef> {
        self.stm_window
            .iter()
            .rev()
            .take(n)
            .filter_map(|id| self.tokens.get(id))
            .map(|token| {
                TokenRef::new(
                    token.id.clone(),
                    token.ast.ttype.clone(),
                    token.embedding.clone(),
                )
            })
            .collect()
    }

    fn get_token(&self, id: &str) -> Option<SentienceToken> {
        self.tokens.get(id).cloned()
    }
}

/// Stub RefNet implementation
pub struct StubRefNet;

impl RefNet for StubRefNet {
    fn evaluate(&self, _stm: &[TokenRef]) -> RefMetrics {
        RefMetrics {
            valence: 0.5,
            smd: 0.3,
            quality: 0.7,
            next_action: "consolidate".to_string(),
            action_logits: HashMap::from([
                ("consolidate".to_string(), 0.8),
                ("recall".to_string(), 0.2),
                ("reframe".to_string(), 0.1),
            ]),
        }
    }
}

/// Stub Superego implementation
pub struct StubSuperego;

impl Superego for StubSuperego {
    fn judge(&self, _token: &SentienceToken, metrics: &RefMetrics) -> Verdict {
        // Simple quality threshold
        if metrics.quality >= 0.6 {
            Verdict::Allow
        } else {
            Verdict::Block("Quality too low".to_string())
        }
    }
}

/// Simple runtime implementation
pub struct SimpleRuntime {
    cortex: InMemoryCortex,
    refnet: StubRefNet,
    superego: StubSuperego,
}

impl SimpleRuntime {
    pub fn new() -> Self {
        Self {
            cortex: InMemoryCortex::new(64),
            refnet: StubRefNet,
            superego: StubSuperego,
        }
    }
}

impl Runtime for SimpleRuntime {
    fn cortex(&mut self) -> &mut dyn Cortex {
        &mut self.cortex
    }

    fn refnet(&self) -> &dyn RefNet {
        &self.refnet
    }

    fn superego(&self) -> &dyn Superego {
        &self.superego
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }

    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot_product / (norm_a * norm_b)
    }
}
