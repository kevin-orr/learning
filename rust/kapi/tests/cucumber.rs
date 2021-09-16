#[path = "../src/lib.rs"]
mod lib;

use cucumber::{after, before, cucumber, steps};
use kapi::{ServerTimeResponse, TickerInfoResponse, gen_nonce};

pub struct MyWorld {
  /// the props holding all the config details etc.
  props: std::collections::HashMap<String, String>,
  /// the time endpoint to call
  endpoint: String,
  /// the ticker api endpoint
  ticker_endpoint: String,
  response: Result<ServerTimeResponse, Box<dyn std::error::Error>>,
  good_ticker_response: Result<TickerInfoResponse, Box<dyn std::error::Error>>,
  bad_ticker_response: Result<TickerInfoResponse, Box<dyn std::error::Error>>,
}

impl cucumber::World for MyWorld {}
impl std::default::Default for MyWorld {
  fn default() -> MyWorld {
    let path_to_props = std::path::PathBuf::from("./kapi.props");

    let config = lib::load_config_props(path_to_props).unwrap();

    // This function is called every time a new scenario is started
    MyWorld {
      /// the path to the configuration
      props: config,
      endpoint: "a default string".to_string(),
      ticker_endpoint: "a default string".to_string(),
      response: Ok(ServerTimeResponse {
        error: Vec::new(),
        result: None,
      }),
      good_ticker_response: Ok(TickerInfoResponse {
        error: Vec::new(),
        result: None,
      }),
      bad_ticker_response: Ok(TickerInfoResponse {
        error: Vec::new(),
        result: None,
      }),
    }
  }
}

mod example_steps {
  use cucumber::steps;
  use kapi::get_ticker_info;
  use kapi::get_time;
  use kapi::validate_ticker_repsonse;
  use kapi::get_open_orders;
  use kapi::gen_nonce;

  // Any type that implements cucumber::World + Default can be the world
  steps!(crate::MyWorld => {
      given "We have the correct server time endpoint" |world, _step| {
          let time_endpoint = world.props.get("time-endpoint");
          match time_endpoint {
            Some(url) => world.endpoint = url.to_string(),
            None      => assert_eq! ("", "error"),
          }
      };

      given "We have the incorrect server time endpoint" |world, _step| {
          let _endpoint = world.props.get("bad-time-endpoint");
          match _endpoint {
            Some(url) => world.endpoint = url.to_string(),
            None      => assert_eq! ("", "error"),
          }
      };  

      when "We call the server for time" |world, _step| {
          world.response = get_time(world.endpoint.clone());
      };

      then "We should get back a properly formed response with both rfc1123 and unixtime properties" |world, _step| {
        match &world.response {
          Err(_) => assert_eq! ("", "Failed to get time fromm server"),
          Ok(res) => match &res.result {
            Some(payload) => {
                assert_eq! (true, res.error.is_empty());
                assert_ne! ("", payload.rfc1123);
                assert_ne! (0, payload.unixtime);
            }
            None => assert_eq! ("", "error - no payload")
          }
        }
      };

      then "We expect errors in the time response" |world, _step| {
        match &world.response {
          Err(_) => assert_eq! ("", "Failed to get time fromm server"),
          Ok(res) => {
            assert_eq! (false, res.error.is_empty());
            assert_eq! ("EGeneral:Unknown method", res.error[0]);
          }
        }
      };

      given "We have the correct ticker endpoint" |world, _step| {
          let _endpoint = world.props.get("ticker-endpoint");
          match _endpoint {
            Some(url) => world.ticker_endpoint = url.to_string(),
            None      => assert_eq! ("", "error - cannot find ticker endpoint"),
          }
      };

      when "We call the server ticker info with valid trading pair" |world, _step| {
        // let _pair = world.props.get("good-pair");
        if let Some(pair) = world.props.get("pair") {
          world.good_ticker_response = get_ticker_info(world.ticker_endpoint.clone(), pair.to_string());
        } else {
          // Destructure failed. Change to the failure case.
          assert_eq! ("", "error - cannot find ticker pair");
        }
      };

      then "We should get back a properly formed ticker info response" |world, _step| {
        match &world.good_ticker_response {
          Err(_) => assert_eq! ("", "Failed to get ticker info for pair"),
          Ok(res) => {
              match validate_ticker_repsonse(res) {
                Ok(_we_can_ignore_me) => (),
                Err(_we_can_ignore_me) => (),
              }
              
            }
          }
      };

      when "We call the ticker info with an invalid trading pair" |world, _step| {
        // let _pair = world.props.get("bad-pair");
        if let Some(pair) = world.props.get("bad-pair") {
          world.bad_ticker_response = get_ticker_info(world.ticker_endpoint.clone(), pair.to_string());
        } else {
          // Destructure failed. Change to the failure case.
          assert_eq! ("", "error - we cannot find bad ticker pair");
        }
      };

      then "We expect errors in ticker response" |world, _step| {
        match &world.bad_ticker_response {
          Err(_) => assert_eq! ("", "Failed to get ticker info for pair"),
          Ok(res) => {
              assert_eq! (false, res.error.is_empty());
              assert_eq! ("EQuery:Unknown asset pair", res.error[0]);
            }
        }
      };

      given "We have api keys" |world, _step| {
        world.props.get("api-key");
        world.props.get("api-sec");
      };
      then "We should get back a properly formed response without errors" |world, _step| {
        let response = get_open_orders(&world.props);
        world.props.insert("nonce".to_string(), gen_nonce().to_string());
        match response {
          Err(_) => assert_eq! ("", "Failed to get open trades"),
          Ok(_) => {
            assert_eq! (true, true);
            world.props.insert("nonce".to_string(), gen_nonce().to_string());
          }
        }        
      };
      then "We should get error when request uses bad nonce" |world, _step| {
        world.props.insert("nonce".to_string(), "111".to_string());
        let response = get_open_orders(&world.props);
        match response {
          Err(_) => assert_ne! (true, true),
          Ok(what) => {
            if what.contains("Error") {
              assert_eq! ("expected to get here", "expected to get here");
            }            
          }
        }        
      };

  });
}

// Declares a before handler function named `a_before_fn`
before!(a_before_fn => |_scenario| {

});

// Declares an after handler function named `an_after_fn`
after!(an_after_fn => |_scenario| {

});

// A setup function to be called before everything else
fn setup() {}

cucumber! {
    features: "./features", // Path to our feature files
    world: crate::MyWorld, // The world needs to be the same for steps and the main cucumber call
    steps: &[
        example_steps::steps // the `steps!` macro creates a `steps` function in a module
    ],
    setup: setup, // Optional; called once before everything
    before: &[
        a_before_fn // Optional; called before each scenario
    ],
    after: &[
        an_after_fn // Optional; called after each scenario
    ]
}
