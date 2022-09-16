// Impementation of the Algorithm to Find Post-Dominators in a Control-Flow Graph

use super::{ControlFlowGraph, WithExitNodes};
use rustc_index::bit_set::BitSet;
use rustc_index::vec::{Idx, IndexVec};
use crate::work_queue::WorkQueue;


#[derive(Clone, Debug)]
pub struct PostDominators<N: Idx> {
    immediate_post_dominators: IndexVec<N, Option<N>>,
    is_constructed: bool,
}

impl<N: Idx> PostDominators<N> {
    // In some cases the algorithm isn't able to find post-dominators.
    // See the code for `exit_nodes`.
    pub fn is_constructed(&self) -> bool {
        self.is_constructed
    }

    // Because None can be encountered inside an MIR, due to multiple exit nodes,
    // we replaced `is_reachable` as shown in `Dominators` with `is_found` here
    // as more accurate description.
    pub fn is_found(&self, node: N) -> bool {
        assert!(self.is_constructed(), "Immediate Post-Dominators were not found.");
        self.immediate_post_dominators[node].is_some()
    }

    pub fn immediate_post_dominator(&self, node: N) -> N {
        assert!(self.is_constructed(), "Immediate Post-Dominators were not found.");
        assert!(self.is_found(node), "Node {:?} is not reachable.", node);
        self.immediate_post_dominators[node].unwrap()
    }
}

/// Algorithm to find Immediate Post-Dominators in a Graph.
/// It is based on the algorithm from [David August's Lecture](
/// https://www.cs.princeton.edu/courses/archive/spr04/cos598C/lectures/02-ControlFlow.pdf)
/// 
/// If a graph doesn't have an exit node, post-dominators can't be calculated.
/// Also, if a CFG has multiple nodes without successors then each of them
/// is considered as exit node.
/// 
/// I assume that each exit node has as immediate post dominator itself.
pub fn post_dominators<G: ControlFlowGraph + WithExitNodes>(graph: G) -> PostDominators<G::Node> {

    let exit_nodes = graph.exit_nodes();

    if exit_nodes.len() == 0 {
        return PostDominators {
            immediate_post_dominators: IndexVec::new(),
            is_constructed: false
        };
    }

    let total_nodes = graph.num_nodes();

    // Initialize pdom for each node to all, except exits,
    // which pdom only themselves.
    let mut pdom: IndexVec<G::Node, BitSet<G::Node>> = 
        IndexVec::from_fn_n(|node| {
            if exit_nodes.contains(&node) {
                BitSet::new_empty(total_nodes)
            } else {
                BitSet::new_filled(total_nodes)
            }
        }, total_nodes);
    for exit_node in &exit_nodes {
        pdom[*exit_node].insert(*exit_node);
    }
    
    let mut change = true;

    while change {
        change = false;

        for node in (0..total_nodes).map(|i| G::Node::new(i)) {
            // Skip exit nodes.
            if exit_nodes.contains(&node) { continue; }

            // First, calculate the intersection of the pdom
            // of every successor.
            let mut tmp = BitSet::new_filled(total_nodes);

            for succ in graph.successors(node) {
                tmp.intersect(&pdom[succ]);
            }

            // Then add the node itself to the tmp.
            tmp.insert(node);

            if tmp != pdom[node] {
                change = true;
                pdom[node] = tmp;
            }
        }
    }

    let mut ipdom: IndexVec<G::Node, Option<G::Node>> = IndexVec::from_elem_n(None, total_nodes);

    let mut queue: WorkQueue<G::Node> = WorkQueue::with_none(total_nodes);
    for exit_node in exit_nodes {
        queue.insert(exit_node);
        ipdom[exit_node] = Some(exit_node); // Exit Nodes post-dominate themselves.
    }

    // For each node v, keep all its post-dominators u
    // that u != v. Then if it doesn't have other
    // post-dominators, we add it to the queue.
    for (index, pdoms) in pdom.iter_enumerated_mut() {
        pdoms.remove(index);
        if pdoms.is_empty() {
            queue.insert(index);
        }
    }

    // Starting from the exit nodes, remove it from every other
    // node's post-dominators. Then if by removing it from a node,
    // the node doesn't have any other post-dominator, add it
    // as its immediate post-domimator and insert that in the
    // queue to repeat the process.
    while let Some(node) = queue.pop() {
        for (index, pdoms) in pdom.iter_enumerated_mut() {
            if pdoms.remove(node)
                && pdoms.is_empty()
                && ipdom[index].is_none()
            {
                ipdom[index] = Some(node);
                queue.insert(index);
            }
        }
    }

    PostDominators {
        immediate_post_dominators: ipdom,
        is_constructed: true
    }
}
