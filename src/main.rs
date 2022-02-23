mod cli;

use crate::cli::SbrdGenApp;
use clap::Parser;

fn main() {
    let app: SbrdGenApp = SbrdGenApp::parse();
    app.run();
}
