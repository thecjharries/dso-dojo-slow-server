# DSO Dojo Slow Server

[![Test](https://github.com/thecjharries/dso-dojo-slow-server/actions/workflows/rust.yaml/badge.svg)](https://github.com/thecjharries/dso-dojo-slow-server/actions/workflows/rust.yaml)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Contents

<!-- no toc -->
- [IMPORTANT SECURITY NOTE](#important-security-note)
- [Exercise](#exercise)
- [Development](#development)

## IMPORTANT SECURITY NOTE

For demonstration purposes, there is a lot of insecure configuration in this repo. Do not use this code in production.

The following things are insecure. If there are others, feel free to open a PR to add them to this list.

- [The dev compose file](./dev-stack.yaml) defines a `POSTGRES_PASSWORD` environment variable. Never store this in a repository.
- [The Rocket config file](./Rocket.toml) defines a connection string in plaintext. Never store this in a repository.
- [The Dockerfile](./Dockerfile) defines a connection string in plaintext. Never store this in a repository.
- [The exercise compose file](./exercise-stack.yaml) defines a connection string in plaintext. Never store this in a repository.

Note that all of these things are the same secret. One of the best ways to handle this in a production environment would be to store the secret in a secret store then consume it from the application itself.

## Exercise

TBD

## Development

You'll need the following tools:

- Rust
- Docker
- `docker-compose`

To develop locally, you'll want to understand [the dev compose file](./dev-stack.yaml). This provides a simple Postgres container locally to test against. For example, to run the code tests,

```bash
make dev
cargo test --verbose
make dev
```
