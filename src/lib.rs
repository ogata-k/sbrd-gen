#![deny(missing_docs, rustdoc::broken_intra_doc_links, unused_variables, deprecated, unsafe_code, dead_code, path_statements)]
//! Library Crate for Schema Based Random GENerator.

pub use generator_type::*;
pub use schema::*;

pub mod builder;
pub mod error;
pub mod eval;
pub mod file;
pub mod generator;
mod generator_type;
pub mod parser;
mod schema;
pub mod value;
pub mod writer;
