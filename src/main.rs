// use crate::error_rc::*;
use crate::error_dc::*;
use structopt::StructOpt;

mod error_rc;
mod error_dc;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "v")]
    val: String,
}

fn main() -> Result<()> {
    let opts = Opt::from_args();
    let v : f64 = opts.val.parse()?;
    println!("{:#?}", v);
    general_error("just some error") 

}
