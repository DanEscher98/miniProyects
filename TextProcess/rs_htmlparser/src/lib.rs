pub mod parser;

extern crate pest;
#[macro_use]
extern crate pest_derive;


pub type Result<T> = anyhow::Result<T, anyhow::Error>;
