use sentience_core::{SentienceCore, SentienceTokenAst, SimpleRuntime, Span, ThoughtType, Value};

fn main() {
    println!("Sentience Core Demo - SRAI Compliant");
    println!("==========================================");

    // Create Sentience Core with default runtime
    let runtime = Box::new(SimpleRuntime::new());
    let mut core = SentienceCore::new(runtime);

    // Example 1: Parse and execute a Percept token
    println!("\n1. Processing Percept Token");
    let percept_dsl = "embed msg -> percept.text";

    match core.process_step(percept_dsl) {
        Ok(result) => {
            println!("✓ Processed Percept token");
            println!("  Token ID: {:?}", result.token_id);
            println!(
                "  Embedding dimension: {}",
                result.embedding.as_ref().map(|v| v.len()).unwrap_or(0)
            );
            println!("  RefNet metrics: {:?}", result.metrics);
            println!("  Generated tokens: {}", result.tokens.len());
            println!("  Generated edges: {}", result.edges.len());
        }
        Err(e) => println!("✗ Error: {}", e),
    }

    // Example 2: Parse and execute a Reflection token
    println!("\n2. Processing Reflection Token");
    let reflection_dsl = "reflect { recall; reframe; consolidate }";

    match core.process_step(reflection_dsl) {
        Ok(result) => {
            println!("✓ Processed Reflection token");
            println!("  Token ID: {:?}", result.token_id);
            if let Some(metrics) = &result.metrics {
                println!("  Valence: {:.3}", metrics.valence);
                println!("  SMD: {:.3}", metrics.smd);
                println!("  Quality: {:.3}", metrics.quality);
                println!("  Next Action: {}", metrics.next_action);
            }
            println!("  Generated tokens: {}", result.tokens.len());
        }
        Err(e) => println!("✗ Error: {}", e),
    }

    // Example 3: Demonstrate deterministic hashing
    println!("\n3. Testing Deterministic Hashing");
    let span = Span::new(1, 1, 1, 10);
    let ast1 = SentienceTokenAst::new(ThoughtType::Percept, span.clone())
        .with_field("modality".to_string(), Value::Str("text".to_string()))
        .with_field("content".to_string(), Value::Str("hello".to_string()));

    let ast2 = SentienceTokenAst::new(ThoughtType::Percept, span)
        .with_field("modality".to_string(), Value::Str("text".to_string()))
        .with_field("content".to_string(), Value::Str("hello".to_string()));

    let hash1 = core.hash(&core.canonicalize(&ast1));
    let hash2 = core.hash(&core.canonicalize(&ast2));

    println!("✓ Hash 1: {}", hash1);
    println!("✓ Hash 2: {}", hash2);
    println!("✓ Deterministic: {}", hash1 == hash2);

    // Example 4: Demonstrate embedding generation
    println!("\n4. Testing Embedding Generation");
    let embedding1 = core.embed(&core.canonicalize(&ast1));
    let embedding2 = core.embed(&core.canonicalize(&ast2));

    println!("✓ Embedding 1 length: {}", embedding1.len());
    println!("✓ Embedding 2 length: {}", embedding2.len());
    println!("✓ Deterministic embeddings: {}", embedding1 == embedding2);

    // Calculate cosine similarity
    let similarity = cosine_similarity(&embedding1, &embedding2);
    println!("✓ Cosine similarity: {:.6}", similarity);

    // Example 5: Show complete pipeline
    println!("\n5. Complete Pipeline Demo");
    let agent_dsl = r#"
        agent Analyst {
            mem short
            goal: "Understand input and self-correct"
            
            on input(msg) {
                embed msg -> percept.text
                reflect {
                    recall ltm[similar: msg, k=5]
                    reframe "summarize_and_check"
                    consolidate
                }
            }
        }
    "#;

    match core.process_step(agent_dsl) {
        Ok(result) => {
            println!("✓ Complete pipeline executed");
            println!("  Generated {} tokens", result.tokens.len());
            println!("  Generated {} edges", result.edges.len());

            for (i, token) in result.tokens.iter().enumerate() {
                println!("  Token {}: {} (ID: {})", i + 1, token.ast.ttype, token.id);
            }

            for (i, edge) in result.edges.iter().enumerate() {
                println!(
                    "  Edge {}: {} -> {} ({})",
                    i + 1,
                    edge.source_id,
                    edge.target_id,
                    edge.edge_type
                );
            }
        }
        Err(e) => println!("✗ Pipeline error: {}", e),
    }

    println!("\nSentience Core Demo Complete!");
    println!("\nKey Features Demonstrated:");
    println!("  ✓ SRAI-compliant token processing");
    println!("  ✓ Deterministic hashing and canonicalization");
    println!("  ✓ Symbolic embedding generation");
    println!("  ✓ RefNet evaluation integration");
    println!("  ✓ Superego gating");
    println!("  ✓ Cortex memory operations");
    println!("  ✓ Complete parse → execute → commit pipeline");
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
