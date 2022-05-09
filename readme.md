# Cucumber tests written in Rust

[![ci](https://github.com/serzhshakur/cucumber-rust-example/actions/workflows/ci.yml/badge.svg)](https://github.com/serzhshakur/cucumber-rust-example/actions)

## Description

This test project aims to cover at least the following scenarios:

- Connect to a exchange (let's call it X-Change) public API
  - Retrieve a server time and validate response;
  - Retrieve some trading pair and validate response;
- Connect to a private API
  - Correctly handle 2fa requirements;
  - Retrieve open orders on the account and validate response;
- Report test results

## Environment variables

You need to set a number of env vars in order to make tests work.
You can use `./.env.example` file as a template (`cp ./.env.example .env`)

- `API_URL` - API url, e.g. `https://api.xchange.com`
- `API_KEY`- your API key
- `PRIV_KEY` - your API private key
- `TFA_PASSWORD` - a static 2fa password for API calls

## Running tests locally

### Unit tests

```
cargo test --lib
```

### Cucumber tests

```
cargo test --test xchange
```

## Running tests using Docker compose

The project is Dockerized so it is also possible to run tests using Docker compose. In addition a Allure report is generated with tests results.

Assuming you have `.env` file set up in your project's root folder run the following command (note `docker-compose` can read `.ev` files):

```
docker-compose up --build
```

Thus you will execute all Cucumber tests and Allure server will be launched. Now you can navigate to `http://localhost:8080` to check the report.

## CI pipeline

A Github Actions workflow is also configured for a project. [Here](https://github.com/serzhshakur/cucumber-rust-example/actions) you will find a status of latest runs.
