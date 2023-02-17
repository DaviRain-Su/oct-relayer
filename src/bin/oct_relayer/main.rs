//! Main entry point for OctRelayer

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use oct_relayer::application::APP;

/// Boot OctRelayer
fn main() {
    abscissa_core::boot(&APP);
}
