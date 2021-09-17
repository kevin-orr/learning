use structopt::StructOpt;
use std::path::Path;
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

  let props_file = args.config_path;

  assert!(Path::new(&props_file).exists());
      
  if args.open_trades {
    let props = load_config_props(props_file.clone());  
    let response = get_open_orders(&props.unwrap());
    match response {
      Ok(result) => println!("Obtained {:?} response from open trade endpoint", result),
      Err(error) => eprintln!("Open trade failure response: {:?}", error),
    }
  }
  
  if args.stime {
    let props = load_config_props(props_file.clone()).unwrap();  
    let _time_endpoint = props.get("time-endpoint").unwrap();

    let response = get_time(_time_endpoint.to_string());
    match response {
      Ok(result) => println!("Server Time response: {:?}", result),
      Err(error) => eprintln!("Server time Failure response: {:?}", error),
    }
  }

  if args.pair {
    let props = load_config_props(props_file.clone()).unwrap();  
    let _endpoint = props.get("ticker-endpoint").unwrap();
    let _pair = props.get("pair").unwrap();
    let response = get_ticker_info(_endpoint.to_string(), _pair.to_string());
    match response {
      Ok(res) => {
        match validate_ticker_repsonse(&res) {
          Ok(symbol) => {
            println!("Response from ticker endpoint");
            println!("{:?}", res.result.as_ref().unwrap().get(&symbol).unwrap()["a"]);
            println!("{:?}", res.result.as_ref().unwrap().get(&symbol).unwrap()["b"]);
            println!("{:?}", res.result.as_ref().unwrap().get(&symbol).unwrap()["c"]);
            println!("{:?}", res.result.as_ref().unwrap().get(&symbol).unwrap()["v"]);
            println!("{:?}", res.result.as_ref().unwrap().get(&symbol).unwrap()["p"]);
            println!("{:?}", res.result.as_ref().unwrap().get(&symbol).unwrap()["t"]);
            println!("{:?}", res.result.as_ref().unwrap().get(&symbol).unwrap()["l"]);
            println!("{:?}", res.result.as_ref().unwrap().get(&symbol).unwrap()["h"]);
            println!("{:?}", res.result.as_ref().unwrap().get(&symbol).unwrap()["o"]);
          },
          Err(_) => println!("Failed to validate ticker response"),
        }        
      }
      Err(error) => eprintln!("Ticker failure response: {:?}", error),
    }
  }

  
}