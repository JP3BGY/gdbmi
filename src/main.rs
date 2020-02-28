extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;

#[derive(Parser)]
#[grammar = "gdbmi_output.pest"]
pub struct MIOutputParser;


fn main() {
    let successful_parse = MIOutputParser::parse(Rule::output,r##"=thread-group-added,id="i1"
~"GNU gdb (Ubuntu 8.3-0ubuntu1) 8.3\n"
=cmd-param-changed,param="print pretty",value="on"
(gdb) 
"##);
    println!("{:?}", successful_parse);
}
