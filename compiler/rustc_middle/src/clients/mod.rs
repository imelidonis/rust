//! Clients for my Post-Dominators Analysis

use rustc_middle::ty::query::Providers;
use rustc_middle::ty::{TyCtxt};

pub(crate) fn provide(providers: &mut Providers) {
    *providers = Providers {
        post_dominators_analysis,
        ..*providers
    };
}

fn post_dominators_analysis<'tcx>(
    _tcx: TyCtxt<'tcx>,
    (): ()
) {

    println!(">>> My Post-Dominators Analysis");
<<<<<<< HEAD:compiler/rustc_middle/src/clients/mod.rs

=======
>>>>>>> de3cdbba12f83b716e0bcfe23c12c43f209d350c:compiler/rustc_mir_dataflow/src/clients/mod.rs
}
