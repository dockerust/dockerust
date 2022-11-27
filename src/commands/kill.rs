use clap::Parser;

#[derive(Parser, Debug)]
pub struct Kill {}

impl Kill {
    pub fn exec(&self) {
        unimplemented!("Kill");
    }
}
