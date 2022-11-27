use clap::Parser;

#[derive(Parser, Debug)]
pub struct Query {}

impl Query {
    pub fn exec(&self) {
        unimplemented!("Query");
    }
}
