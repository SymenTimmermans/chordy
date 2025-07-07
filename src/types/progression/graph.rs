//! Progression graph implementations for major and minor keys
//!
//! This module contains the concrete implementations of progression graphs
//! using the static data generated from Stephen Mugglin's progression maps.

use super::{ProgressionNode, ProgressionEdge, ProgressionOptions, NodeRef, NodeType};
use std::collections::HashMap;

/// Static progression graph for major keys using compile-time generated data
/// 
/// This struct provides zero-allocation progression lookups using static references
/// to nodes and edges generated at compile time from major.progression.
pub struct StaticMajorGraph;

/// Static progression graph for minor keys using compile-time generated data
/// 
/// This struct provides zero-allocation progression lookups using static references
/// to nodes and edges generated at compile time from minor.progression.  
pub struct StaticMinorGraph;

/// Dynamic progression graph that can be built at runtime
/// 
/// This allows for custom progression maps and runtime modifications,
/// though with higher memory overhead than static graphs.
pub struct ProgressionGraph {
    nodes: Vec<ProgressionNode>,
    edges: Vec<ProgressionEdge>,
    node_map: HashMap<String, usize>,
}

/// Common interface for all progression graph types
/// 
/// This trait allows uniform access to progression functionality regardless
/// of whether the graph is static (compile-time) or dynamic (runtime).
pub trait ProgressionGraphLike {
    /// Get all progression options from a given node
    /// 
    /// Returns categorized options by strength:
    /// - Strong: explicit arrows (natural voice leading)
    /// - Moderate: jumps to primary nodes (stable)
    /// - Weak: jumps to secondary nodes (transitional)
    fn progression_options<'a>(&self, from: impl Into<NodeRef<'a>>) -> Option<ProgressionOptions>;
    
    /// Get a node by its string identifier
    fn get_node(&self, id: &str) -> Option<&ProgressionNode>;
    
    /// Get all nodes in the graph
    fn all_nodes(&self) -> &[&ProgressionNode];
    
    /// Get all edges in the graph
    fn all_edges(&self) -> &[&ProgressionEdge];
}

impl StaticMajorGraph {
    /// Create a new static major progression graph
    pub fn new() -> Self {
        Self
    }
}

impl ProgressionGraphLike for StaticMajorGraph {
    fn progression_options<'a>(&self, from: impl Into<NodeRef<'a>>) -> Option<ProgressionOptions> {
        use crate::types::progression::major_data::{ALL_NODES, ALL_EDGES, get_node};
        
        let from_node = match from.into() {
            NodeRef::Static(node) => node,
            NodeRef::Dynamic(id) => get_node(&id)?,
        };
        
        let mut options = ProgressionOptions::new();
        
        // Find strong connections (explicit arrows)
        for edge in ALL_EDGES {
            if std::ptr::eq(edge.from, from_node) {
                options.strong.push(edge.to);
            }
        }
        
        // Find moderate and weak connections (jumps to all other nodes)
        for &node in ALL_NODES {
            if std::ptr::eq(node, from_node) {
                continue; // Skip self
            }
            
            // Skip if already in strong connections
            if options.strong.iter().any(|&strong_node| std::ptr::eq(strong_node, node)) {
                continue;
            }
            
            match node.node_type {
                NodeType::Primary => options.moderate.push(node),
                NodeType::Secondary => options.weak.push(node),
            }
        }
        
        Some(options)
    }
    
    fn get_node(&self, id: &str) -> Option<&ProgressionNode> {
        use crate::types::progression::major_data::get_node;
        get_node(id)
    }
    
    fn all_nodes(&self) -> &[&ProgressionNode] {
        use crate::types::progression::major_data::ALL_NODES;
        ALL_NODES
    }
    
    fn all_edges(&self) -> &[&ProgressionEdge] {
        use crate::types::progression::major_data::ALL_EDGES;
        ALL_EDGES
    }
}

impl StaticMinorGraph {
    /// Create a new static minor progression graph
    pub fn new() -> Self {
        Self
    }
}

impl ProgressionGraphLike for StaticMinorGraph {
    fn progression_options<'a>(&self, from: impl Into<NodeRef<'a>>) -> Option<ProgressionOptions> {
        use crate::types::progression::minor_data::{ALL_NODES, ALL_EDGES, get_node};
        
        let from_node = match from.into() {
            NodeRef::Static(node) => node,
            NodeRef::Dynamic(id) => get_node(&id)?,
        };
        
        let mut options = ProgressionOptions::new();
        
        // Find strong connections (explicit arrows)
        for edge in ALL_EDGES {
            if std::ptr::eq(edge.from, from_node) {
                options.strong.push(edge.to);
            }
        }
        
        // Find moderate and weak connections (jumps to all other nodes)
        for &node in ALL_NODES {
            if std::ptr::eq(node, from_node) {
                continue; // Skip self
            }
            
            // Skip if already in strong connections
            if options.strong.iter().any(|&strong_node| std::ptr::eq(strong_node, node)) {
                continue;
            }
            
            match node.node_type {
                NodeType::Primary => options.moderate.push(node),
                NodeType::Secondary => options.weak.push(node),
            }
        }
        
        Some(options)
    }
    
    fn get_node(&self, id: &str) -> Option<&ProgressionNode> {
        use crate::types::progression::minor_data::get_node;
        get_node(id)
    }
    
    fn all_nodes(&self) -> &[&ProgressionNode] {
        use crate::types::progression::minor_data::ALL_NODES;
        ALL_NODES
    }
    
    fn all_edges(&self) -> &[&ProgressionEdge] {
        use crate::types::progression::minor_data::ALL_EDGES;
        ALL_EDGES
    }
}

impl ProgressionGraph {
    /// Create a new empty dynamic progression graph
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            node_map: HashMap::new(),
        }
    }
    
    /// Add a node to the graph
    pub fn add_node(&mut self, node: ProgressionNode) {
        let index = self.nodes.len();
        self.node_map.insert(node.display_name.to_string(), index);
        self.nodes.push(node);
    }
    
    /// Add an edge to the graph
    /// 
    /// Note: This creates owned ProgressionEdge instances rather than static references
    pub fn add_edge(&mut self, from_id: &str, to_id: &str) -> Result<(), String> {
        let from_idx = self.node_map.get(from_id)
            .ok_or_else(|| format!("Node not found: {}", from_id))?;
        let to_idx = self.node_map.get(to_id)
            .ok_or_else(|| format!("Node not found: {}", to_id))?;
        
        // For dynamic graphs, we need to work differently since we can't create
        // static references. This is a limitation of the current design that
        // prioritizes static efficiency.
        Err("Dynamic edge creation not yet implemented - use static graphs".to_string())
    }
}

impl ProgressionGraphLike for ProgressionGraph {
    fn progression_options<'a>(&self, _from: impl Into<NodeRef<'a>>) -> Option<ProgressionOptions> {
        // TODO: Implement dynamic progression options
        // This requires a different approach since we can't use static references
        None
    }
    
    fn get_node(&self, id: &str) -> Option<&ProgressionNode> {
        let index = self.node_map.get(id)?;
        self.nodes.get(*index)
    }
    
    fn all_nodes(&self) -> &[&ProgressionNode] {
        // This doesn't work with the current trait design that expects static references
        // TODO: Consider redesigning the trait for better dynamic support
        &[]
    }
    
    fn all_edges(&self) -> &[&ProgressionEdge] {
        // Same issue as all_nodes
        &[]
    }
}

impl Default for StaticMajorGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for StaticMinorGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ProgressionGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_static_major_graph_creation() {
        let graph = StaticMajorGraph::new();
        let nodes = graph.all_nodes();
        assert!(!nodes.is_empty(), "Major graph should have nodes");
    }
    
    #[test]
    fn test_static_minor_graph_creation() {
        let graph = StaticMinorGraph::new();
        let nodes = graph.all_nodes();
        assert!(!nodes.is_empty(), "Minor graph should have nodes");
    }
    
    #[test]
    fn test_node_lookup() {
        let graph = StaticMajorGraph::new();
        let node = graph.get_node("I");
        assert!(node.is_some(), "Should find I chord in major graph");
    }
    
    #[test]
    fn test_progression_options() {
        let graph = StaticMajorGraph::new();
        let options = graph.progression_options("I");
        assert!(options.is_some(), "Should get progression options for I");
        
        let options = options.unwrap();
        assert!(!options.strong.is_empty(), "I should have strong progressions");
    }
}