//! `oxidize` provides an interface in python for building highly performant
//! web applications using python for business logic and rust for everything 
//! that needs to be fast. Compiling SQL Queries, Rendering Templates, and 
//! routing with complex regular expressions are all done in rust for maximum
//! performance and memory efficiency. Then when its time to inject business logic,
//! oxidize binds back into python to get logic for rendering pages, retrieving objects
//! from the database, and others. 
//!
#![deny(missing_docs, rust_2018_idioms)]

mod routing;
mod views;
