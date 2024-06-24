# Rust API Testing

An exercise in using Cucumber, Rust, and Docker to test APIs

## Prerequisites
- [Docker](https://docs.docker.com/get-docker/) installed
- generated api keys (public and private)
- host url without trailing forward slashes (/)
- GBP funds -> *Steps to use this not implemented yet*
- a stop-loss trigger for buying BTC with GBP which will not execute in the current market condition (defaulted to 60000) -> *Steps to use this not implemented yet*

## To Note
- there are unimplemented steps which ***will*** cause a test failure. Don't want a test to appear to be passing when it's not actually running..

## Building the docker image
```bash
cd path/to/project
```
```bash
docker build -t rust_api_tests .
```

## Running the tests

```bash
docker run -e PUBLIC_KEY=<PUBLIC_KEY> -e PRIVATE_KEY=<PRIVATE_KEY> -e TRIGGER=<REASONABLE_STOP_LOSS_TRIGGER> -e API_HOST=<API_HOST> rust_api_tests
```


## Improvements
- I would definitely use a builder pattern to better manage the hand built structs used in the tests
- Would investigate how best to implement the output of [openapi-generator](https://openapi-generator.tech/docs/usage#generate) in the test framework after putting the OpenAPI specification through it. Would mean much less code maintenance on every api change
- Cucumber test results needs to be integrated into Rust's standard test report 
- More validity checks, particularly around the various `World` objects, would be useful
- Better error handling!!!
- Investigation around using the `slim-buster` variation of Rust Docker would be useful as the build times for the docker image are a bit too long
- getting out of the OOP mindset is going to be a really good thing when working with Rust
- I need to learn a lot more about how to properly structure a Rust project, particulary around how to structure/access/use modules and libraries, as well as how to best reuse code, as this project is ***very*** messy...

## Follow Up
- implement the step to create a stop-loss order that will not likely execute in a real world env
- also implement a clean up call to cancel the created order after test execution
- do a big clean up of the dead code in the project