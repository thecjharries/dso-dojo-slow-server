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

You work for Acme Corp as a platform engineer. Acme's premiere application, `dso_dojo_slow_postgres`, is well known for its data-crunching power. However, it's also very slow. Ten years ago, customers of the `/api/<id>` endpoint were very happy to wait 10 seconds for the behemoth to power through its calculations. Today, they're not so happy. You've been tasked with figuring out how to speed up responses. The application itself has been optimized as much as possible, so you'll need to look elsewhere.

### The App

#### Index

The application itself provides a homepage, `/` (or `/index.html`). This is a placeholder to get new customers hooked and has no bearing on your task.

#### Ping

Acme adopted the `/ping` standard several years ago. This endpoint allows for a simple connection test. It will always return this simple JSON:

```json
{
  "message": "pong"
}
```

#### API

The `/api/<id>` endpoint is the main attraction. It takes a single unsigned 64-bit integer parameter, `id`, and returns a JSON object with the following schema:

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://dso_dojo_slow_postgres.com/api.schema.json",
  "title": "API",
  "description": "The API endpoint",
  "type": "object",
  "properties": {
    "id": {
      "description": "The ID passed to the endpoint",
      "type": "integer",
      "minimum": 0,
      "maximum": 18446744073709551615
    },
    "token": {
      "description": "The token returned by the endpoint",
      "type": "string",
      "minLength": 64,
      "maxLength": 64
    }
  },
  "required": [
    "id",
    "token"
  ]
}
```

Here's an example return:

```json
{
  "id": 10,
  "token": "hjupifwjnzholhbcehxlmdgaayihhjfbsnkmaecvmumzcmyfqueruzayamxhpflo"
}
```

The `id` field is the same as the parameter passed to the endpoint. The `token` field is the magic that Acme sells and is a result of the hefty calculations performed by the application. Within a reasonable time frame, it's safe to assume that rerunning the same `id` will return the same `token`.

### Suspending Your Disbelief

Behind the scenes, `/api/<id>` runs [a delay in Postgres](https://web.archive.org/web/20220701141902/https://database.guide/how-pg_sleep-works-in-postgresql/) for the number of seconds specified by the `API_WAIT_SECONDS` environment variable (default 10s). It then seeds a random number generator with the `id` parameter and returns a 64-character string of random characters. It is deterministic in this way.

You could easily solve the problem by removing the delay, but that would be cheating. There's at least one way to fix this by editing the server itself. Since our goal is to LARP a problem with an external system we have no control over, the ideal solution would be to fix the problem without changing the server. The call is deterministic. That should be enough to get you started.

### Getting Set Up

The first thing you'll need to do is get the server running locally.

```bash
git clone https://github.com/thecjharries/dso-dojo-slow-server
cd dso-dojo-slow-server
make docker-build
```

You'll now have a local image called `thecjharries/dso_dojo_slow_postgres` (note that we're building locally because that's a useful skill and to avoid any hosting platform retention policy). You can launch the exercise with this command (still inside the `dso-dojo-slow-server` directory):

```bash
make exercise
```

You'll now be able to access the exercise in your browser. It's up to you to figure out how!

### Building a solution

If you choose to use `docker-compose` to build your solution, you can use [the provided test runner](./exercise-unit-test.sh) to validate your solution. It uses [a solution stack](./solution-stack.yaml) that cheats by setting `API_WAIT_SECONDS` to 0. Remove the env var and make any changes you then run `./exercise-unit-test.sh` to test. That might mean you build your solution in the `dso-dojo-slow-server` directory or you might do things elsewhere and just validate your solution in this directory.

If you choose not to use `docker-compose`, not a problem! You can always take apart the test script and adapt it to your solution.


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
