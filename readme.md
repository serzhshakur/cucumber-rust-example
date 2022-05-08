# Cucumber tests written in Rust

[![ci](https://github.com/serzhshakur/cucumber-rust-example/actions/workflows/ci.yml/badge.svg)](https://github.com/serzhshakur/cucumber-rust-example/actions)

## Description

This project aims to cover with API tests at least the following scenarios:

- Connect to a exchange (let's call it X-Change) public API
  - Retrieve the server time, validates response;
  - Retrieve some trading pair, validates response;
- Connect to the private API
  - Correctly handle 2fa requirements;
  - Retrieve open orders on the account and validate response;
- Report test results

## Running tests

### Unit tests

```
cargo test --lib
```

### Cucumber tests

```
cargo test --test xchange
```
