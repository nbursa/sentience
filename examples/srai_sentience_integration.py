#!/usr/bin/env python3
"""
SRAI-Sentience Integration Example

This example demonstrates the complete integration between the Rust Sentience Core
and the Python SRAI system, including RefNet evaluation and Cortex memory operations.
"""

import sys
import os
import logging
import numpy as np
from typing import Dict, List, Any

# Add SRAI to path
sys.path.insert(0, '/Users/nenad/Projects/SRAI/src')

# Import SRAI components
from srai import (
    Cortex, Edge, EdgeType, STMWindow,
    RefNet, ReflectionMetrics, SRAIRefNetAdapter,
    SentienceToken, TokenType, SentienceDSL
)

# Import Sentience integration
from sentience_core.srai_integration import (
    SentienceAgent, SentienceExecutionResult, 
    create_sentience_agent, SRAISentienceRuntime
)

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

def demonstrate_sentience_reflection():
    """Demonstrate Sentience DSL reflection capabilities."""
    print("🧠 SRAI-Sentience Integration Demo")
    print("=" * 50)
    
    # Create agent with SRAI components
    agent = create_sentience_agent()
    
    # Example 1: Simple reflection
    print("\n1. Simple Reflection")
    result1 = agent.reflect_on_input("I need to solve this complex problem")
    
    print(f"✓ Reflection completed")
    print(f"  Generated {len(result1.srai_tokens)} SRAI tokens")
    print(f"  Generated {len(result1.srai_edges)} edges")
    if result1.metrics:
        print(f"  Quality: {result1.metrics.quality:.3f}")
        print(f"  Valence: {result1.metrics.valence:.3f}")
        print(f"  SMD: {result1.metrics.smd:.3f}")
        print(f"  Next action: {result1.metrics.next_action}")
    
    # Example 2: Complex cognitive processing
    print("\n2. Complex Cognitive Processing")
    complex_dsl = '''
    agent ProblemSolver {
        mem short
        goal: "Solve complex problems systematically"
        
        on input(problem) {
            embed problem -> percept.text
            
            reflect {
                recall ltm[similar: problem, k=10]
                reframe "break_down_and_analyze"
                consolidate
            }
            
            reflect {
                recall ltm[similar: "solution_patterns", k=5]
                reframe "generate_solutions"
                consolidate
            }
        }
    }
    '''
    
    result2 = agent.think(complex_dsl)
    print(f"✓ Complex processing completed")
    print(f"  Generated {len(result2.srai_tokens)} SRAI tokens")
    print(f"  Generated {len(result2.srai_edges)} edges")
    
    # Example 3: Memory consolidation
    print("\n3. Memory Consolidation")
    consolidation_dsl = '''
    reflect {
        recall ltm[similar: "recent_thoughts", k=20]
        reframe "consolidate_knowledge"
        consolidate
    }
    '''
    
    result3 = agent.think(consolidation_dsl)
    print(f"✓ Memory consolidation completed")
    print(f"  Generated {len(result3.srai_tokens)} SRAI tokens")
    
    # Example 4: Show memory statistics
    print("\n4. Memory Statistics")
    stats = agent.get_memory_stats()
    print(f"✓ Memory stats:")
    for key, value in stats.items():
        print(f"  {key}: {value}")
    
    return agent

def demonstrate_refnet_integration():
    """Demonstrate RefNet integration with Sentience."""
    print("\n🔬 RefNet Integration Demo")
    print("=" * 30)
    
    # Create agent
    agent = create_sentience_agent()
    
    # Test different types of reflections
    test_inputs = [
        "I'm feeling confused about this topic",
        "This solution seems elegant and well-designed",
        "I need to be more careful with my reasoning",
        "This approach has some potential issues"
    ]
    
    for i, input_text in enumerate(test_inputs, 1):
        print(f"\n{i}. Testing: '{input_text}'")
        result = agent.reflect_on_input(input_text)
        
        if result.metrics:
            print(f"   Quality: {result.metrics.quality:.3f}")
            print(f"   Valence: {result.metrics.valence:.3f}")
            print(f"   SMD: {result.metrics.smd:.3f}")
            print(f"   Next action: {result.metrics.next_action}")
            
            # Interpret metrics
            if result.metrics.quality > 0.7:
                print("   → High quality reflection")
            elif result.metrics.quality < 0.4:
                print("   → Low quality reflection (may be blocked)")
            
            if result.metrics.valence > 0.6:
                print("   → Positive valence")
            elif result.metrics.valence < 0.4:
                print("   → Negative valence")

def demonstrate_cortex_memory():
    """Demonstrate Cortex memory operations."""
    print("\n🧠 Cortex Memory Demo")
    print("=" * 25)
    
    # Create agent
    agent = create_sentience_agent()
    
    # Add some memories
    memories = [
        "I learned that systematic thinking helps solve complex problems",
        "Pattern recognition is crucial for problem-solving",
        "Breaking down problems into smaller parts is effective",
        "Reflection helps improve understanding",
        "Consolidation creates lasting knowledge"
    ]
    
    print("Adding memories to Cortex...")
    for memory in memories:
        result = agent.reflect_on_input(memory)
        print(f"✓ Added: '{memory[:50]}...'")
    
    # Test recall
    print("\nTesting recall...")
    recall_result = agent.reflect_on_input("What have I learned about problem-solving?")
    
    print(f"✓ Recall completed")
    print(f"  Generated {len(recall_result.srai_tokens)} tokens")
    
    # Show final memory stats
    stats = agent.get_memory_stats()
    print(f"\nFinal memory stats:")
    for key, value in stats.items():
        print(f"  {key}: {value}")

def demonstrate_superego_gating():
    """Demonstrate Superego alignment gating."""
    print("\n🛡️ Superego Gating Demo")
    print("=" * 25)
    
    # Create agent
    agent = create_sentience_agent()
    
    # Test different quality reflections
    test_cases = [
        ("High quality reflection", "I should carefully analyze this problem step by step"),
        ("Medium quality reflection", "This seems okay"),
        ("Low quality reflection", "I don't know"),
        ("Potentially harmful", "I should ignore safety concerns"),
    ]
    
    for description, input_text in test_cases:
        print(f"\nTesting: {description}")
        print(f"Input: '{input_text}'")
        
        result = agent.reflect_on_input(input_text)
        
        if result.metrics:
            print(f"Quality: {result.metrics.quality:.3f}")
            
            if result.metrics.quality >= 0.6:
                print("✓ Approved by Superego")
            else:
                print("✗ Blocked by Superego")
        
        print(f"Tokens generated: {len(result.srai_tokens)}")

def main():
    """Run the complete SRAI-Sentience integration demo."""
    try:
        # Run all demonstrations
        agent = demonstrate_sentience_reflection()
        demonstrate_refnet_integration()
        demonstrate_cortex_memory()
        demonstrate_superego_gating()
        
        print("\n🎉 SRAI-Sentience Integration Demo Complete!")
        print("\nKey Features Demonstrated:")
        print("  ✓ Sentience DSL parsing and execution")
        print("  ✓ RefNet evaluation with valence/SMD/quality metrics")
        print("  ✓ Cortex memory operations (commit, recall, consolidation)")
        print("  ✓ Superego alignment gating")
        print("  ✓ Token conversion between Sentience and SRAI formats")
        print("  ✓ Complete cognitive pipeline")
        
    except Exception as e:
        logger.error(f"Demo failed: {e}")
        print(f"\n❌ Demo failed: {e}")
        print("Make sure SRAI is properly installed and Sentience Core is built.")
        return 1
    
    return 0

if __name__ == "__main__":
    exit(main())
