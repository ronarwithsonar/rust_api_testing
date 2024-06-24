# Rust API Testing

An exercise in using Cucumber, Rust, and Docker to test APIs.

## Prerequisites
- [Docker](https://docs.docker.com/get-docker/) installed.
- Generated API keys (public and private).
- Host URL without trailing forward slashes ( e.g. `https://api.example.com`).
- GBP funds (Steps to use this not implemented yet).
- A stop-loss trigger for buying BTC with GBP, which will not execute in the current market condition (defaulted to 60000) (Steps to use this not implemented yet).

## Important Notes
- There are unimplemented steps that ***will*** cause test failures. This ensures tests do not appear to be passing when they are not actually running.

## Building the Docker Image
Navigate to the project directory and build the Docker image:

```bash
cd path/to/project
docker build -t rust_api_tests .
```

## Running the Tests
Run the Docker container with the necessary environment variables:

```bash
docker run -e PUBLIC_KEY=<PUBLIC_KEY> -e PRIVATE_KEY=<PRIVATE_KEY> -e TRIGGER=<REASONABLE_STOP_LOSS_TRIGGER> -e API_HOST=<API_HOST> rust_api_tests
```

## Improvements
### Code and Architecture
- **Builder Pattern**: Use a builder pattern to better manage the hand-built structs used in the tests.
- **OpenAPI Integration**: Investigate implementing the output of [openapi-generator](https://openapi-generator.tech/docs/usage#generate) in the test framework after processing the OpenAPI specification. This would reduce code maintenance with API changes.
- **Cucumber Integration**: Integrate Cucumber test results into Rust's standard test report.
- **Validation Checks**: Add more validity checks, particularly around the various `World` objects.
- **Error Handling**: Improve error handling throughout the project.
- **Docker Optimization**: Explore using the `slim-buster` variation of Rust Docker to reduce build times.

### Learning and Best Practices
- **Rust Paradigms**: Move away from an OOP mindset and adopt Rust idioms (e.g. ownership/borrowing)
- **Project Structure**: Learn more about structuring Rust projects, particularly around module and library organization.

## Follow Up Tasks
- Implement the step to create a stop-loss order that is unlikely to execute in a real-world environment (potentially using traits to handle various requests)
- Add a cleanup call to cancel the created order after test execution.
- Perform a thorough cleanup of dead code in the project.
