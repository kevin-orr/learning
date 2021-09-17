# learning Some Rust

This app was generated as part of a test to show how to chat to some API endpoints.

The test was to chat to a public endpoint that returned the server time, chat to a public endpoint that display some info of a trading pair and finally to chat to a private (api key secured) endpoint that returned some info on open trades.

To chat with the private endpoint required creating an account to generate API keys for the app but unfortunately I didn't perform any real trades so this secured endpoint only demonstrates actually contacting the private endpoint and returning a successfully response.


For both the app and the cucumber tests you'll need to add properties to the **kapi.props** file:
```
time-endpoint=https://<host-name>/0/public/Time
bad-time-endpoint=https://<host-name>/0/public/Times

ticker-endpoint=https://<host-name>/0/public/Ticker
pair=XBTUSD

bad-ticker-endpoint=https://<host-name>/0/public/Ticker
bad-pair=XBTUS222D

api-key=********
api-sec=********
base-api-url=https://<host-name>
open-orders-uri-path=/0/private/OpenOrders
nonce=
```

You can leave the `nonce` property as is - it's really only used during the `cucumber` tests to force the same nonce to be used.

You can run the app via:
```
cargo run -- -c -t -o -p kapi.props 
```
where `-c`, `-t` and `-o` are optional but `-p` is required and is the location of the props file.
For example, running `cargo run -- -c -p kapi.props` will give the following (assuming valid props):
```
Server Time response: ServerTimeResponse { error: [], result: Some(ServerTime { unixtime: 1631828165, rfc1123: "Thu, 16 Sep 21 21:36:05 +0000" }) }

```

## Notes
- To run the cumber tests: `cargo test --test cucumber`
- Went with `blocking` version of the http client - didn't really see the need to do `async` calls for this app/test.
- need to write `Dockerfile`