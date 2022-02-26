mod cli;

use crate::cli::SbrdGenApp;
use clap::Parser;
use human_panic::setup_panic;

fn main() {
    setup_panic!();
    let app: SbrdGenApp = SbrdGenApp::parse();
    app.run();
}
