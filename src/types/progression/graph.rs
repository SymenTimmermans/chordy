//! Progression graph implementations for major and minor keys
//!
//! This module contains the concrete implementations of progression graphs
//! using the static data generated from Stephen Mugglin's progression maps.

use super::{ProgressionNode, ProgressionOptions, NodeRef, NodeType, DynamicProgressionEdge};
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
    edges: Vec<DynamicProgressionEdge>,
    node_map: HashMap<String, usize>,
}

/// Common interface for all progression graph types
/// 
/// This trait allows uniform access to progression functionality regardless
/// of whether the graph is static (compile-time) or dynamic (runtime).
pub trait ProgressionGraphLike {
    /// Iterator type for nodes in this graph
    type NodeIter<'a>: Iterator<Item = &'a ProgressionNode> + 'a where Self: 'a;
    
    /// Get all progression options from a given node
    /// 
    /// Returns categorized options by strength:
    /// - Strong: explicit arrows (natural voice leading)
    /// - Moderate: jumps to primary nodes (stable)
    /// - Weak: jumps to secondary nodes (transitional)
    fn progression_options<'a>(&self, from: impl Into<NodeRef<'a>>) -> Option<ProgressionOptions>;
    
    /// Get a node by its string identifier
    fn get_node(&self, id: &str) -> Option<&ProgressionNode>;
    
    /// Get an iterator over all nodes in the graph
    fn nodes(&self) -> Self::NodeIter<'_>;
    
    /// Get the total number of nodes in the graph
    fn node_count(&self) -> usize;
    
    /// Get the total number of edges in the graph  
    fn edge_count(&self) -> usize;
    
    /// Check if the graph contains any nodes
    fn is_empty(&self) -> bool {
        self.node_count() == 0
    }
}

impl StaticMajorGraph {
    /// Create a new static major progression graph
    pub fn new() -> Self {
        Self
    }
}

impl ProgressionGraphLike for StaticMajorGraph {
    type NodeIter<'a> = std::iter::Map<std::slice::Iter<'a, &'static ProgressionNode>, fn(&&'static ProgressionNode) -> &'a ProgressionNode>;
    
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
    
    fn nodes(&self) -> Self::NodeIter<'_> {
        use crate::types::progression::major_data::ALL_NODES;
        ALL_NODES.iter().map(|node_ref| -> &'_ ProgressionNode { *node_ref })
    }
    
    fn node_count(&self) -> usize {
        use crate::types::progression::major_data::ALL_NODES;
        ALL_NODES.len()
    }
    
    fn edge_count(&self) -> usize {
        use crate::types::progression::major_data::ALL_EDGES;
        ALL_EDGES.len()
    }
}

impl StaticMinorGraph {
    /// Create a new static minor progression graph
    pub fn new() -> Self {
        Self
    }
}

impl ProgressionGraphLike for StaticMinorGraph {
    type NodeIter<'a> = std::iter::Map<std::slice::Iter<'a, &'static ProgressionNode>, fn(&&'static ProgressionNode) -> &'a ProgressionNode>;
    
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
    
    fn nodes(&self) -> Self::NodeIter<'_> {
        use crate::types::progression::minor_data::ALL_NODES;
        ALL_NODES.iter().map(|node_ref| -> &'_ ProgressionNode { *node_ref })
    }
    
    fn node_count(&self) -> usize {
        use crate::types::progression::minor_data::ALL_NODES;
        ALL_NODES.len()
    }
    
    fn edge_count(&self) -> usize {
        use crate::types::progression::minor_data::ALL_EDGES;
        ALL_EDGES.len()
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
    /// Creates a strong connection between two nodes identified by their display names.
    /// Both nodes must already exist in the graph.
    pub fn add_edge(&mut self, from_id: &str, to_id: &str) -> Result<(), String> {
        let from_idx = *self.node_map.get(from_id)
            .ok_or_else(|| format!("Node not found: {}", from_id))?;
        let to_idx = *self.node_map.get(to_id)
            .ok_or_else(|| format!("Node not found: {}", to_id))?;
        
        // Check if edge already exists to avoid duplicates
        if self.edges.iter().any(|edge| edge.from_index == from_idx && edge.to_index == to_idx) {
            return Err(format!("Edge already exists: {} -> {}", from_id, to_id));
        }
        
        self.edges.push(DynamicProgressionEdge {
            from_index: from_idx,
            to_index: to_idx,
        });
        
        Ok(())
    }
    
    /// Get all edges in the graph
    pub fn edges(&self) -> &[DynamicProgressionEdge] {
        &self.edges
    }
    
    /// Check if an edge exists between two nodes
    pub fn has_edge(&self, from_id: &str, to_id: &str) -> bool {
        if let (Some(&from_idx), Some(&to_idx)) = (self.node_map.get(from_id), self.node_map.get(to_id)) {
            self.edges.iter().any(|edge| edge.from_index == from_idx && edge.to_index == to_idx)
        } else {
            false
        }
    }
    
    /// Remove an edge from the graph
    pub fn remove_edge(&mut self, from_id: &str, to_id: &str) -> Result<(), String> {
        let from_idx = *self.node_map.get(from_id)
            .ok_or_else(|| format!("Node not found: {}", from_id))?;
        let to_idx = *self.node_map.get(to_id)
            .ok_or_else(|| format!("Node not found: {}", to_id))?;
        
        let initial_len = self.edges.len();
        self.edges.retain(|edge| !(edge.from_index == from_idx && edge.to_index == to_idx));
        
        if self.edges.len() == initial_len {
            Err(format!("Edge not found: {} -> {}", from_id, to_id))
        } else {
            Ok(())
        }
    }
}

impl ProgressionGraphLike for ProgressionGraph {
    type NodeIter<'a> = std::slice::Iter<'a, ProgressionNode>;
    
    fn progression_options<'a>(&self, from: impl Into<NodeRef<'a>>) -> Option<ProgressionOptions> {
        let from_node = match from.into() {
            NodeRef::Static(node) => {
                // For static node references, find the matching node in our dynamic graph
                self.nodes.iter().find(|n| n.display_name == node.display_name)?
            },
            NodeRef::Dynamic(id) => {
                let index = self.node_map.get(&id)?;
                self.nodes.get(*index)?
            },
        };
        
        let mut options = ProgressionOptions::new();
        
        // Find the index of the from_node in our nodes vector
        let from_index = self.nodes.iter().position(|n| std::ptr::eq(n, from_node))?;
        
        // Find strong connections (explicit edges)
        for edge in &self.edges {
            if edge.from_index == from_index {
                if let Some(to_node) = self.nodes.get(edge.to_index) {
                    options.strong.push(to_node);
                }
            }
        }
        
        // Find moderate and weak connections (jumps to all other nodes)
        for (i, node) in self.nodes.iter().enumerate() {
            if i == from_index {
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
        let index = self.node_map.get(id)?;
        self.nodes.get(*index)
    }
    
    fn nodes(&self) -> Self::NodeIter<'_> {
        self.nodes.iter()
    }
    
    fn node_count(&self) -> usize {
        self.nodes.len()
    }
    
    fn edge_count(&self) -> usize {
        self.edges.len()
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
        let nodes: Vec<_> = graph.nodes().collect();
        assert!(!nodes.is_empty(), "Major graph should have nodes");
    }
    
    #[test]
    fn test_static_minor_graph_creation() {
        let graph = StaticMinorGraph::new();
        let nodes: Vec<_> = graph.nodes().collect();
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