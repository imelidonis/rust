//! Clients for my Post-Dominators Analysis

use rustc_middle::ty::query::Providers;
use rustc_middle::ty::{self, TyCtxt};

pub(crate) fn provide(providers: &mut Providers) {
    *providers = Providers {
        post_dominators_analysis,
        ..*providers
    };
}

fn post_dominators_analysis<'tcx>(
    tcx: TyCtxt<'tcx>,
    (): ()
) {
    println!(">>> My Post-Dominators Analysis");

    for def_id in tcx.mir_keys(()) {

        let def = ty::WithOptConstParam::unknown(def_id.to_def_id());
        let body = &tcx.instance_mir(ty::InstanceDef::Item(def));
        
        println!(">>> Basic Blocks: {}", body.basic_blocks().len());

        body.post_dominators();
    }
}
