use base64;
use hmac_sha512::HMAC;
use reqwest::blocking::Client;
use serde_json::Value;
use sha2ni::{Digest, Sha256};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use url::form_urlencoded;

#[derive(Debug, serde::Deserialize)]
pub struct OpenOrdersResponse {
  pub error: Vec<String>,
  pub result: Option<HashMap<String, Value>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ServerTimeResponse {
  pub error: Vec<String>,
  pub result: Option<ServerTime>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ServerTime {
  pub unixtime: i64,
  pub rfc1123: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct TickerInfoResponse {
  pub error: Vec<String>,
  pub result: Option<HashMap<String, Value>>,
}

pub fn load_config_props(path_to_config: std::path::PathBuf) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
  match { std::fs::read_to_string(&path_to_config) } {
    Ok(file_contents) => return Ok(parse_props_from_string(file_contents.to_string())),
    Err(error)        => return Err(error.into()),
  };
}

fn parse_props_from_string(file_contents: String) -> HashMap<String, String> {
  
  let mut _props = HashMap::new();
  
  for line in file_contents.lines() {
    let ok_ignore_line = line.trim().is_empty() || line.starts_with("#") || line.starts_with("-");

    if !ok_ignore_line {
      let prop_parts: Vec<&str> = line.split("=").collect();
      _props.insert(String::from(prop_parts[0]), String::from(prop_parts[1]));
    }
  }
  _props
}


pub fn http_client() -> Client {
  Client::new()
}

pub fn get_time(endpoint: String) -> Result<ServerTimeResponse, Box<dyn std::error::Error>> {
  let response: ServerTimeResponse = http_client().get(endpoint).send()?.json()?;
  Ok(response)
}

pub fn get_ticker_info(
  endpoint: String,
  pair: String,
) -> Result<TickerInfoResponse, Box<dyn std::error::Error>> {
  let mut _query = endpoint.clone();

  _query.insert_str(_query.len(), "?pair=");
  _query.insert_str(_query.len(), &pair);

  Ok(http_client().get(_query).send()?.json()?)
}

pub fn validate_ticker_repsonse(ticker_info: &TickerInfoResponse) -> Result<String, &str> {
  assert_eq!(true, ticker_info.error.is_empty());

  if let Some(map) = &ticker_info.result {
    let expect_number_of_keys = 1;

    assert_eq!(expect_number_of_keys, map.keys().len());

    let key_name = map.keys().next().unwrap();

    assert_eq!(false, Value::String(key_name.to_string()).is_null());
    assert_ne!("", key_name);
    assert_eq!(false, map.get(key_name).unwrap()["a"].is_null());
    assert_eq!(true, map.get(key_name).unwrap()["a"].is_array());
    assert_eq!(false, map.get(key_name).unwrap()["b"].is_null());
    assert_eq!(true, map.get(key_name).unwrap()["b"].is_array());

    assert_eq!(false, map.get(key_name).unwrap()["c"].is_null());
    assert_eq!(true, map.get(key_name).unwrap()["c"].is_array());

    assert_eq!(false, map.get(key_name).unwrap()["v"].is_null());
    assert_eq!(true, map.get(key_name).unwrap()["v"].is_array());

    assert_eq!(false, map.get(key_name).unwrap()["p"].is_null());
    assert_eq!(true, map.get(key_name).unwrap()["p"].is_array());
    assert_eq!(false, map.get(key_name).unwrap()["t"].is_null());
    assert_eq!(true, map.get(key_name).unwrap()["t"].is_array());
    assert_eq!(false, map.get(key_name).unwrap()["l"].is_null());
    assert_eq!(true, map.get(key_name).unwrap()["l"].is_array());
    assert_eq!(false, map.get(key_name).unwrap()["h"].is_null());
    assert_eq!(true, map.get(key_name).unwrap()["h"].is_array());
    assert_eq!(false, map.get(key_name).unwrap()["o"].is_null());
    assert_eq!(true, map.get(key_name).unwrap()["o"].is_string());

    Ok(key_name.to_string())
  } else {
    // Destructure failed. Change to the failure case.
    assert_eq!("", "error - we cannot find bad ticker pair");
    Err("BADSYMBOL")
  }
}

pub fn get_open_orders(
  props: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
  
  let _base_api_url = props.get("base-api-url").unwrap();
  let _api_key = props.get("api-key").unwrap();
  let _api_sec = props.get("api-sec").unwrap();
  let _open_orders_path = props.get("open-orders-uri-path").unwrap();
  // build up the open orders endpoint
  let mut _query = _base_api_url.clone();
  _query.insert_str(_query.len(), _open_orders_path);
  // generate the nonce and signature pair
  let (_nonce, _signature_base64) = gen_signature(props);

  let _result = http_client()
    .post(_query.clone())
    .header("API-Key", _api_key)
    .header("API-Sign", _signature_base64)
    .form(&[("nonce", _nonce), ("trades", "true".to_string())])
    .send();
  //
  // WARNING - not going to drill down into the response - just check that I get a good response...
  //
  let _response = match _result {
    Ok(res) => {
      let _r = res.text();
      match _r {
        Ok(info) => {
          let _orders: OpenOrdersResponse =
            serde_json::from_str::<OpenOrdersResponse>(&info).unwrap();
          // ok, so we got a response
          if !_orders.error.is_empty() {
            return Ok("Error - we've got errors".to_string());
          }
        }
        Err(_) => {
          return Ok("Error - we've got an error response".to_string());
        }
      }
    }
    Err(_) => {
      return Ok("Error - something went wrong".to_string());
    }
  };

  Ok("Success".to_string())
}

pub fn gen_signature(props: &HashMap<String, String>) -> (String, String) {
  let _api_url = props.get("base-api-url").unwrap();
  let _api_key = props.get("api-key").unwrap();
  let _api_sec = props.get("api-sec").unwrap();
  let _uri_path = props.get("open-orders-uri-path").unwrap();

  let _nonce: String = if props.get("nonce").is_none() {
    gen_nonce().to_string()
  } else if props.get("nonce").unwrap().to_string() == "" {
    gen_nonce().to_string()
  } else {
    props.get("nonce").unwrap().to_string()
  };
  // form url encod the POST data
  let _post_date: String = form_urlencoded::Serializer::new(String::new())
    .append_pair("nonce", &_nonce)
    .append_pair("trades", "true")
    .finish();
  // then build up "nonce+data"
  let mut _encoded: String = _nonce.to_string();
  _encoded.insert_str(_encoded.len(), &_post_date);
  // next url encode the url
  let _uri_path_encoded = form_urlencoded::Serializer::new(_uri_path.clone().to_string()).finish();
  // next get sha256 digest of encode payload
  let _digest = gen_sha_256(_encoded);
  // next we're going to create mac for uri+sha256(encode)
  let mut _to_hash = vec![];
  _to_hash.append(&mut _uri_path.clone().into_bytes());
  _to_hash.append(&mut _digest.clone());
  // we'll need the secret key in Base64
  let _secret = base64::decode(&_api_sec).unwrap();
  // then generate sha512 mac for the hash info
  let _sha512_res = gen_sha_512(_to_hash, _secret);
  // and finally base64 it
  let _base64 = base64::encode(&_sha512_res);
  // println!("Nonce {:?} and signature {:?}", nonce, base64);
  (_nonce, _base64)
}

pub fn gen_nonce() -> u64 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs()
    * 1000000000
}

pub fn gen_sha_256(input: String) -> Vec<u8> {
  let mut _hasher = Sha256::new();
  _hasher.input(input.as_bytes());
  _hasher.result().to_vec()
}

pub fn gen_sha_512(input: Vec<u8>, secret: Vec<u8>) -> Vec<u8> {
  HMAC::mac(&input, &secret).to_vec()
}
