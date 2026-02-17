# pifp-stellar

**Proof-of-Impact Funding Protocol (PIFP)** — a Soroban (Stellar) smart-contract project.

This repository is being built for the **Drips Stellar Wave**.

## Structure

- `contracts/pifp_protocol/` — core Soroban contract crate

## Contract (WIP)

The initial boilerplate includes:

- `Project` data type (creator, goal, proof_hash, balance)
- `create_project()` with `require_auth()` for the creator
- `verify_and_release()` skeleton for future proof verification + fund release

## License

MIT — see `LICENSE`.
