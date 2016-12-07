// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(rustc_private)]
#![feature(proc_macro)]

extern crate cargo;
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate hyper;
extern crate racer;
extern crate rls_analysis as analysis;
extern crate rls_vfs as vfs;
extern crate rustfmt;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate languageserver_types as ls_types;

use std::sync::Arc;

mod build;
mod ide;
mod server;
mod actions;
mod lsp_data;

#[cfg(test)]
mod test;

// Timeout = 0.5s (totally arbitrary).
const COMPILER_TIMEOUT: u64 = 500;

pub fn main() {
    env_logger::init().unwrap();

    let analysis = Arc::new(analysis::AnalysisHost::new(analysis::Target::Debug));
    let vfs = Arc::new(vfs::Vfs::new());
    let build_queue = Arc::new(build::BuildQueue::new(vfs.clone()));

    server::run_server(analysis, vfs, build_queue);
}
