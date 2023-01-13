# lumina-rust-graph

A serverless function that implements the GraphQL API for Lumina services. It uses a PostgeSQL database, Juniper to serve GraphQL queries and is designed to be deployed to AWS Lambda.

Further documentation for the AWS Rust runtime is available [here](https://github.com/awslabs/aws-lambda-rust-runtime).

### Configuration

Create a .env file with the following variables

    # PostgreSQL connection string
    DATABASE_URL=

### Deployment

1. Install [cargo-lambda](https://www.cargo-lambda.info/), advisedly with `cargo install cargo-lambda`.

2. Set [AWS credentials](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html) in `~/.aws/credentials` (and optionally region)

3. Build the project with `cargo lambda build --release`

4. Deploy with `cargo lambda deploy --env-file .env --enable-function-url lumina-rust-graph`

5. You can now use the endpoint returned by the previous command.

### Development

1. Run `cargo lambda watch`

2. You can now use the endpoint `http://localhost:9000/lambda-url/lumina-rust-graph/`

### Generate ORM

1. Make sure you have libpg installed (on Gentoo, `emerge dev-db/postgresql`).

2. Install diesel-cli with `cargo install diesel_cli --no-default-features --features postgres`.

3. Run `diesel print-schema > src/graph/schema.rs`.
