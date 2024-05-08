---
keywords: [intermediate, rust, http, get, http get]
---

# HTTP: GET

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/rust/send_http_get)

The purpose of this dapp is to give developers a minimal dapp that uses the IC's HTTPS outcalls feature to make a `GET` request.

This demo goes in hand with the [developer documentation on HTTPS outcalls](https://internetcomputer.org/docs/current/developer-docs/integrations/https-outcalls/https-outcalls-get).

If you want to start working on your project right away, you might want to try the following commands:

```bash
dfx help
dfx canister --help
```

## Running the project locally
If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```