use stable_eyre::eyre::*;
use structopt::StructOpt;


#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "v")]
    val: String,
}

fn parse_val(s: &String) -> Result<f64> {
    let v : f64 = s.parse()?;
    Ok(v)
}

fn main() -> Result<()> {
    // Backtrace needs install() and "export RUST_BACKTRACE=1" in compile shell.
    stable_eyre::install()?; 

    let opts = Opt::from_args();
    let v = parse_val(&opts.val)
                .with_context(|| format!("Could not parse --val argument as float : {}", opts.val))?;
    println!("Successfully parsed: opts.val = {} as {}", opts.val, v);
    Err(eyre!("But now we have an error anyway.") )

}
