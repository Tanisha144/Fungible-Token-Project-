# Fungible-Token-Project-

This is a ready-to-run minimal ICP (Internet Computer) Rust canister project with a simple frontend.

## What's included
- `Cargo.toml` and `src/lib.rs` — the Rust canister implementing the token logic (init, balance_of, total_supply, transfer, mint).
- `dfx.json` — dfx project configuration.
- `token_canister.did` — Candid interface for the canister.
- `frontend/index.html` — simple UI that calls the canister methods using `@dfinity/agent`.
- `frontend/package.json` — minimal package.json for frontend dependencies.

## Quick start (local development)

1. Install DFINITY SDK (dfx) and Rust toolchain (stable).
2. Initialize the project:
   ```bash
   cd /path/to/project
   dfx start --background
   dfx deploy
   ```
3. Build the canister wasm (dfx will run the `cargo build --target wasm32-unknown-unknown --release` step for the Rust canister).
4. After deploy, dfx will generate the candid JS bindings under `.dfx/local/canisters/token_canister/`.
   - Copy the generated `token_canister.did.js` into `frontend/.dfx/local/canisters/token_canister/token_canister.did.js`
   - Or serve the frontend through dfx (assets canister) so the path resolves automatically.
5. Update `frontend/index.html` replacing `<REPLACE_WITH_CANISTER_ID>` with the deployed canister id (dfx will print it when deploying).
6. Open the frontend (if using assets canister, dfx will host it; otherwise open the file locally and ensure the agent points to the right host).
7. Use dfx identity principals as the "Your Principal" field for testing.

## Notes
- For local testing you might need to `agent.fetchRootKey()` in the frontend for the HttpAgent when using the local replica (UNSAFE for production).
- This is a minimal demo — for production you'll want better identity handling, input validation, and stable storage across upgrades.
