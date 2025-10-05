"""
SRAI-Sentience Integration Module

This module provides a bridge between the Rust Sentience Core and the Python SRAI system,
enabling seamless integration of the Sentience DSL with RefNet, Cortex, and Superego.
"""

import numpy as np
from typing import Dict, List, Optional, Any, Tuple
from dataclasses import dataclass
import json
import logging

# Import SRAI components
from srai import (
    Cortex, Edge, EdgeType, STMWindow,
    RefNet, ReflectionMetrics, SRAIRefNetAdapter,
    SentienceToken, TokenType, SentienceDSL
)

# Import Sentience Core (will be available after building)
try:
    from sentience_core import (
        PySentienceCore, PyExecutionResult, create_sentience_core,
        THOUGHT_TYPE_PERCEPT, THOUGHT_TYPE_REFLECTION, THOUGHT_TYPE_ACTION,
        THOUGHT_TYPE_CONCEPT, THOUGHT_TYPE_SELF_MODEL
    )
except ImportError:
    # Fallback for development
    PySentienceCore = None
    PyExecutionResult = None
    create_sentience_core = None

logger = logging.getLogger(__name__)

@dataclass
class SentienceRefNetMetrics:
    """RefNet metrics for Sentience tokens."""
    valence: float
    smd: float
    quality: float
    next_action: str
    action_logits: Dict[str, float]

@dataclass
class SentienceExecutionResult:
    """Result of Sentience execution with SRAI integration."""
    token_id: Optional[str]
    embedding: Optional[np.ndarray]
    metrics: Optional[SentienceRefNetMetrics]
    tokens: List[Dict[str, Any]]
    edges: List[Dict[str, Any]]
    srai_tokens: List[SentienceToken]
    srai_edges: List[Edge]

class SRAISentienceRuntime:
    """
    Runtime that integrates Sentience Core with SRAI components.
    
    This runtime provides:
    - Cortex memory operations
    - RefNet evaluation
    - Superego alignment gating
    - Token conversion between formats
    """
    
    def __init__(self, cortex: Cortex, refnet_adapter: SRAIRefNetAdapter):
        self.cortex = cortex
        self.refnet_adapter = refnet_adapter
        self.sentience_core = create_sentience_core() if create_sentience_core else None
        
        # Token conversion cache
        self.token_cache: Dict[str, SentienceToken] = {}
        
    def process_sentience_dsl(self, dsl_code: str) -> SentienceExecutionResult:
        """
        Process Sentience DSL code through the complete SRAI pipeline.
        
        Args:
            dsl_code: Sentience DSL code to process
            
        Returns:
            Execution result with both Sentience and SRAI tokens
        """
        if not self.sentience_core:
            raise RuntimeError("Sentience Core not available")
            
        # Step 1: Parse and execute with Sentience Core
        py_result = self.sentience_core.process_step(dsl_code)
        
        # Step 2: Convert to SRAI format
        srai_tokens = self._convert_to_srai_tokens(py_result.tokens())
        srai_edges = self._convert_to_srai_edges(py_result.edges())
        
        # Step 3: Evaluate with RefNet
        refnet_metrics = self._evaluate_with_refnet(srai_tokens)
        
        # Step 4: Apply Superego gating
        approved_tokens, approved_edges = self._apply_superego_gating(
            srai_tokens, srai_edges, refnet_metrics
        )
        
        # Step 5: Commit to Cortex
        committed_ids = self._commit_to_cortex(approved_tokens, approved_edges)
        
        # Step 6: Generate final result
        return SentienceExecutionResult(
            token_id=py_result.token_id(),
            embedding=np.array(py_result.embedding()) if py_result.embedding() else None,
            metrics=self._convert_refnet_metrics(refnet_metrics),
            tokens=py_result.tokens(),
            edges=py_result.edges(),
            srai_tokens=approved_tokens,
            srai_edges=approved_edges
        )
    
    def _convert_to_srai_tokens(self, py_tokens: List[Dict[str, Any]]) -> List[SentienceToken]:
        """Convert Python tokens to SRAI SentienceToken format."""
        srai_tokens = []
        
        for py_token in py_tokens:
            # Map Sentience token types to SRAI token types
            type_mapping = {
                THOUGHT_TYPE_PERCEPT: TokenType.PERCEPT,
                THOUGHT_TYPE_REFLECTION: TokenType.REFLECTION,
                THOUGHT_TYPE_ACTION: TokenType.ACTION,
                THOUGHT_TYPE_CONCEPT: TokenType.CONCEPT,
                THOUGHT_TYPE_SELF_MODEL: TokenType.SELF,
            }
            
            sentience_type = py_token["type"]
            srai_type = type_mapping.get(sentience_type, TokenType.PERCEPT)
            
            # Create SRAI token
            srai_token = SentienceToken(
                token_type=srai_type,
                ast=py_token["fields"],
                token_id=py_token["id"]
            )
            
            srai_tokens.append(srai_token)
            self.token_cache[py_token["id"]] = srai_token
            
        return srai_tokens
    
    def _convert_to_srai_edges(self, py_edges: List[Dict[str, Any]]) -> List[Edge]:
        """Convert Python edges to SRAI Edge format."""
        srai_edges = []
        
        for py_edge in py_edges:
            # Map edge types
            edge_type_mapping = {
                "ABOUT": EdgeType.ABOUT_SELF,
                "CAUSES": EdgeType.CAUSES,
                "SUPPORTS": EdgeType.SUPPORTS,
                "CONTRADICTS": EdgeType.CONTRADICTS,
                "DERIVED_FROM": EdgeType.STRUCTURAL,
                "TEMPORAL": EdgeType.TEMPORAL,
                "SEMANTIC": EdgeType.SEMANTIC,
                "STRUCTURAL": EdgeType.STRUCTURAL,
            }
            
            edge_type = edge_type_mapping.get(py_edge["edge_type"], EdgeType.SEMANTIC)
            
            srai_edge = Edge(
                source_id=py_edge["source_id"],
                target_id=py_edge["target_id"],
                edge_type=edge_type,
                weight=py_edge["weight"]
            )
            
            srai_edges.append(srai_edge)
            
        return srai_edges
    
    def _evaluate_with_refnet(self, tokens: List[SentienceToken]) -> ReflectionMetrics:
        """Evaluate tokens with RefNet."""
        if not tokens:
            return ReflectionMetrics(valence=0.5, smd=0.3, quality=0.7, next_action="consolidate")
        
        # Get STM window for RefNet evaluation
        stm_window = self.cortex.get_stm_window()
        
        # Convert to embeddings for RefNet
        embeddings = []
        for token in tokens:
            # Generate embedding (simplified)
            embedding = self._generate_token_embedding(token)
            embeddings.append(embedding)
        
        # Evaluate with RefNet
        metrics = self.refnet_adapter.evaluate_stm_window(stm_window, embeddings)
        return metrics
    
    def _apply_superego_gating(self, tokens: List[SentienceToken], edges: List[Edge], 
                              metrics: ReflectionMetrics) -> Tuple[List[SentienceToken], List[Edge]]:
        """Apply Superego alignment gating."""
        approved_tokens = []
        approved_edges = []
        
        for token in tokens:
            # Simple quality threshold (can be extended with more sophisticated rules)
            if metrics.quality >= 0.6:
                approved_tokens.append(token)
            else:
                logger.warning(f"Token {token.token_id} blocked by Superego: quality too low")
        
        # All edges are approved if their tokens are approved
        approved_token_ids = {token.token_id for token in approved_tokens}
        for edge in edges:
            if edge.source_id in approved_token_ids and edge.target_id in approved_token_ids:
                approved_edges.append(edge)
        
        return approved_tokens, approved_edges
    
    def _commit_to_cortex(self, tokens: List[SentienceToken], edges: List[Edge]) -> List[str]:
        """Commit approved tokens and edges to Cortex."""
        committed_ids = []
        
        for token in tokens:
            # Generate embedding
            embedding = self._generate_token_embedding(token)
            
            # Commit to Cortex
            token_id = self.cortex.commit_token(token, embedding)
            committed_ids.append(token_id)
        
        for edge in edges:
            self.cortex.add_relation(edge)
        
        return committed_ids
    
    def _generate_token_embedding(self, token: SentienceToken) -> List[float]:
        """Generate embedding for a token."""
        # Simple hash-based embedding (can be replaced with learned embeddings)
        import hashlib
        
        token_str = f"{token.token_type.value}:{json.dumps(token.ast, sort_keys=True)}"
        hash_val = int(hashlib.sha256(token_str.encode()).hexdigest()[:8], 16)
        
        # Generate 256-dimensional embedding
        embedding = []
        for i in range(256):
            val = np.sin(hash_val * (i + 1)) * 0.1
            embedding.append(val)
        
        return embedding
    
    def _convert_refnet_metrics(self, metrics: ReflectionMetrics) -> SentienceRefNetMetrics:
        """Convert SRAI RefNet metrics to Sentience format."""
        return SentienceRefNetMetrics(
            valence=metrics.valence,
            smd=metrics.smd,
            quality=metrics.quality,
            next_action=metrics.next_action,
            action_logits={}  # Can be extended
        )

class SentienceAgent:
    """
    High-level agent that uses Sentience DSL for cognitive processing.
    
    This agent provides a simple interface for running Sentience code
    with full SRAI integration.
    """
    
    def __init__(self, cortex: Cortex, refnet_adapter: SRAIRefNetAdapter):
        self.runtime = SRAISentienceRuntime(cortex, refnet_adapter)
        self.step_count = 0
    
    def think(self, dsl_code: str) -> SentienceExecutionResult:
        """
        Execute a thinking step using Sentience DSL.
        
        Args:
            dsl_code: Sentience DSL code to execute
            
        Returns:
            Execution result with all generated tokens and metrics
        """
        self.step_count += 1
        logger.info(f"Executing thinking step {self.step_count}")
        
        result = self.runtime.process_sentience_dsl(dsl_code)
        
        logger.info(f"Step {self.step_count} completed: {len(result.srai_tokens)} tokens, {len(result.srai_edges)} edges")
        
        return result
    
    def reflect_on_input(self, input_text: str) -> SentienceExecutionResult:
        """
        Reflect on input text using Sentience DSL.
        
        Args:
            input_text: Text to reflect upon
            
        Returns:
            Reflection result
        """
        dsl_code = f'''
        embed "{input_text}" -> percept.text
        reflect {{
            recall ltm[similar: "{input_text}", k=5]
            reframe "analyze_and_synthesize"
            consolidate
        }}
        '''
        
        return self.think(dsl_code)
    
    def get_memory_stats(self) -> Dict[str, Any]:
        """Get current memory statistics."""
        return self.runtime.cortex.get_stats()

# Example usage and testing
def create_sentience_agent() -> SentienceAgent:
    """Create a Sentience agent with default SRAI components."""
    # Create Cortex
    cortex = Cortex()
    
    # Create RefNet adapter (using the integrated RefNet)
    refnet_adapter = SRAIRefNetAdapter()
    
    return SentienceAgent(cortex, refnet_adapter)

if __name__ == "__main__":
    # Example usage
    agent = create_sentience_agent()
    
    # Test reflection
    result = agent.reflect_on_input("I need to understand this complex problem")
    
    print(f"Reflection completed:")
    print(f"  Generated {len(result.srai_tokens)} tokens")
    print(f"  Quality: {result.metrics.quality:.3f}")
    print(f"  Next action: {result.metrics.next_action}")
    
    # Test memory stats
    stats = agent.get_memory_stats()
    print(f"Memory stats: {stats}")
