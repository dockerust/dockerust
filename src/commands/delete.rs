use clap::Parser;

#[derive(Parser, Debug)]
pub struct Delete {}

impl Delete {
    pub fn exec(&self) {
        unimplemented!("Delete");
    }
}
