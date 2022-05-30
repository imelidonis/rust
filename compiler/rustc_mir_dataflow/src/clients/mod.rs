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
}