---
keywords: [blockchain, rust, icp, github]
---

## Overview

### ICP Node

The ICP node provides a local canister execution environment that enables you to deploy canisters to test your dapps during development. This simulates having a node of the blockchain running.

### Backend Canisters

These are the set of canisters (or smart contracts) which hold the bounty service logic and manage funds and users.

### GitHub App

This GitHub app runs using an `.env` file containing credentials, and it serves to route webhook calls into canister calls, depending on the users' interactions on the GitHub platform.
It also exposes a REST API for the opeartor.
This is useful, for example, to verify the github app can reach the canisters by calling the /bounty/healthcheck endpoint as follows:

```sh
curl http://localhost:3000/bounty/healthcheck
```

## Pre-requisites

- rustc 1.79.0
- cargo 1.79.0
- dfx 0.20.1
- GNU Make 4.4.1
- npm 10.8.1
- Node v20.15.1

### System Upgrade

- rustup update
- dfxvm update
- nvm install --lts --latest-npm

> Note: to update GNU make, depends on your OS.

## Running End-to-End (E2E) Tests

> Note: Each component must be run in its own terminal session, and all commands should be executed from the root directory of the project. This ensures that the processes do not interfere with each other and that the project’s file paths are correctly referenced.

To kick off the E2E tests, you need to set up an environment with the following components:

1. **ICP Node**: Start the local Internet Computer (IC) replica node.

   ```sh
   dfx start --clean
   ```

2. **Backend Canisters**: Deploy the backend canisters to the replica and generates the candid interface.

   ```sh
   make install
   npm run generate
   ```

3. **GitHub App**: Start the GitHub application.
   ```sh
   cd offchain/github && npm start
   ```

Finally, you can run the E2E tests by executing:

```sh
cd offchain/github && npm test
```

> Note: Each step depends on the successful completion of the previous one. Please ensure you run the commands in the specified order to avoid any dependency issues.

### Details of a Healthy Run

#### ICP Node

The replica node is connected and accessible at http://127.0.0.1:4943.

If successful, the console should display:

```sh
Initialized replica.
Dashboard: http://localhost:58788/_/dashboard
```

#### Backend Canisters

If successful, you should see the following output in the console:

```sh
Reinstalling code for canister identity, with canister ID rdmx6-jaaaa-aaaaa-aaadq-cai
Using identity: "default".
Reinstalling code for canister icrc1_ledger, with canister ID mxzaz-hqaaa-aaaar-qaada-cai
Reinstalling code for canister icrc1_index, with canister ID n5wcd-faaaa-aaaar-qaaea-cai
Reinstalling code for canister backend, with canister ID bkyz2-fmaaa-aaaaa-qaaaq-cai
```

#### GitHub App

If successful, the console should display:

```sh
INFO (server): Listening on http://localhost:3000
INFO (server): Connected
```

---

## Running the project locally

> Ensure your server is reachable from the internet.
> If you want to test your project locally, you can use the following commands:
> TODO!
