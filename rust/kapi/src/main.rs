use structopt::StructOpt;
use kapi::*;

/// perform some action based on command line switches.
#[derive(Debug, StructOpt)]
#[structopt(name = "api test", about = "An cmd line app to make various api tests.")]
struct CliOptions {
    /// get the time on server
    #[structopt(short = "t", long = "server-time")]
    stime: bool,
    /// get ticket info for a predefined pair
    #[structopt(short = "p", long = "trade-pair")]
    pair: bool,
    /// path to file containing kapi props such as api keys/secrets
    #[structopt(parse(from_os_str), short = "f", long = "file")]
    config_path: std::path::PathBuf,
    /// get the open trades
    #[structopt(short = "o", long = "open-trades")]
    open_trades: bool,   
}

fn main() {

  let args = CliOptions::from_args();

  let config_file = args.config_path;
      
  if args.open_trades {
    println!("open trades");
  }
  
  if args.stime {
    println!("server time");
  }

  if args.pair {
    println!("trade pair");
  }

  
}