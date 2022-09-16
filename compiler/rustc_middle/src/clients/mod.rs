//! Clients for my Post-Dominators Analysis

use rustc_middle::ty::query::Providers;
use rustc_middle::ty::{self, TyCtxt};
use rustc_middle::mir::{Body, BasicBlock};
use rustc_data_structures::graph::WithExitNodes;
use rustc_index::bit_set::BitSet;
use rustc_data_structures::graph::post_dominators::PostDominators;
use rustc_span::def_id::LocalDefId;

pub(crate) fn provide(providers: &mut Providers) {
    *providers = Providers {
        post_dominators_analysis,
        ..*providers
    };
}


#[allow(dead_code)]
// This function is used only to debug the result if neccesary and to print the resut
// in a human-readable form. It is not used because we couldn't get the result metrics
// in that format.
fn print_result(def_id: &LocalDefId, body: &Body<'_>, post_dominators: &PostDominators<BasicBlock>) {
    println!("--> Post-Dominators analysis for: {:?}", def_id);
    if post_dominators.is_constructed() {

        println!("\t>>> Post Dominators calculated.");
        println!("\t>>> Solution:");

        print!("\t>>> ");
        for (bb, _) in body.basic_blocks().iter_enumerated() {
            if post_dominators.is_found(bb) {
                let ipdom = post_dominators.immediate_post_dominator(bb);
                print!("IPDOM({:?}) = {:?}, ", bb, ipdom);
            } else {
                print!("IPDOM({:?}) = None, ", bb);
            }
        }

        println!("");
    } else {
        println!("\t>>> Undefined Solution {:?}", def_id);
    }
}

fn post_dominators_analysis<'tcx>(
    tcx: TyCtxt<'tcx>,
    (): ()
) {
    println!(">>> My Post-Dominators Analysis");

    for def_id in tcx.mir_keys(()) {

        let def = ty::WithOptConstParam::unknown(def_id.to_def_id());
        let body = tcx.instance_mir(ty::InstanceDef::Item(def));

        let post_dominators = body.post_dominators();
        
        // Uncomment to print the full result in the console.
        // print_result(def_id, body, &post_dominators);

        if post_dominators.is_constructed() {
            let dominators = body.dominators();
            let total_nodes = body.basic_blocks().len();
            let mut distinct_pdom = BitSet::new_empty(total_nodes);
            let mut distinct_dom = BitSet::new_empty(total_nodes);
            let mut none_count = 0;
    
            for (bb, _) in body.basic_blocks().iter_enumerated() {
                if post_dominators.is_found(bb) {
                    let ipdom = post_dominators.immediate_post_dominator(bb);

                    distinct_pdom.insert(ipdom);
                } else {
                    none_count += 1;
                }

                if dominators.is_reachable(bb) {
                    let idom = dominators.immediate_dominator(bb);

                    distinct_dom.insert(idom);
                }
            }
            
            let total_exit_nodes = body.exit_nodes().len();
            let total_distinct_ipdoms = distinct_pdom.count();
            let total_distinct_idoms = distinct_dom.count();

            println!("\t--> metrics: {:?} {} {} {} {} {}", def_id, total_nodes, total_exit_nodes, none_count, total_distinct_ipdoms, total_distinct_idoms);
        } else {
            println!("\t--> Undefined Solution {:?}", def_id);
        }

    }
}
