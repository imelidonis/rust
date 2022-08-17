// Impementation of the Algorithm to Find Post-Dominators in a Control-Flow Graph

use super::{ControlFlowGraph, WithExitNode};
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
    // See the code for `exit_node`.
    pub fn is_constructed(&self) -> bool {
        self.is_constructed
    }

    pub fn is_reachable(&self, node: N) -> bool {
        assert!(self.is_constructed(), "Immediate Post-Dominators were not found.");
        self.immediate_post_dominators[node].is_some()
    }

    pub fn immediate_post_dominator(&self, node: N) -> N {
        assert!(self.is_constructed(), "Immediate Post-Dominators were not found.");
        assert!(self.is_reachable(node), "Node {:?} is not reachable.", node);
        self.immediate_post_dominators[node].unwrap()
    }
}

/// Algorithm to find Immediate Post-Dominators in a Graph.
/// It is based on the algorithm from [David August's Lecture](
/// https://www.cs.princeton.edu/courses/archive/spr04/cos598C/lectures/02-ControlFlow.pdf)
pub fn post_dominators<G: ControlFlowGraph + WithExitNode>(graph: G) -> PostDominators<G::Node> {

    println!("--> calculating post-dominators");

    if let Some(exit_node) = graph.exit_node() {

        let total_nodes = graph.num_nodes();

        // Initialize pdom for each node to all, except exit,
        // which pdoms only itself.
        let mut pdom: IndexVec<G::Node, BitSet<G::Node>> = 
            IndexVec::from_fn_n(|node| {
                if node == exit_node {
                    BitSet::new_empty(total_nodes)
                } else {
                    BitSet::new_filled(total_nodes)
                }
            }, total_nodes);
        pdom[exit_node].insert(exit_node);
        
        let mut change = true;

        while change {
            change = false;

            for node in (0..total_nodes).map(|i| G::Node::new(i)) {
                // Skip exit node.
                if node == exit_node { continue; }

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

        // For each node v, keep all its post-dominators u
        // that u != v. 
        for (index, pdoms) in pdom.iter_enumerated_mut() {
            pdoms.remove(index);
        }

        let mut ipdom: IndexVec<G::Node, Option<G::Node>> = IndexVec::from_elem_n(None, total_nodes);

        let mut queue: WorkQueue<G::Node> = WorkQueue::with_none(total_nodes);
        queue.insert(exit_node);

        // Starting from the exit node, remove it from every other
        // node's post-dominators. Then if a node doesn't have any
        // other post-dominator, add 'node' as its immediate post-
        // domimator and insert that in the queue to repeat the
        // process.
        while let Some(node) = queue.pop() {
            for (index, pdoms) in pdom.iter_enumerated_mut() {
                pdoms.remove(node);

                if pdoms.is_empty() && ipdom[index].is_none() {
                    ipdom[index] = Some(node);
                    queue.insert(index);
                }
            }
        }

        PostDominators {
            immediate_post_dominators: ipdom,
            is_constructed: true
        }
    } else {
        PostDominators {
            immediate_post_dominators: IndexVec::new(),
            is_constructed: false
        }
    }
}
