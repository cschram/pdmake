use pest::Parser;

#[derive(Parser)]
#[grammar = "src/plua/plua.pest"]
pub struct PluaParser;
