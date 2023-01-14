# graph-api

A serverless function that implements the GraphQL API for Lumina services. It uses a PostgeSQL database, Juniper to serve GraphQL queries and is designed to be deployed to AWS Lambda.

Further documentation for the AWS Rust runtime is available [here](https://github.com/awslabs/aws-lambda-rust-runtime).

### Configuration

Create a .env file with the following variables

PostgreSQL connection string, with the database password obtained from supabase
```
DATABASE_URL=
```

### Local Development

1. Clone the repository to your computer
2. Ensure you have the rust toolchain installed
3. Run `cargo lambda watch`
4. You can now use the endpoint `http://localhost:9000/lambda-url/graph-api/`

#### MacOS Installation Instructions

Run the following commands in order

1. `brew install postgresql libpq`
2. `cargo clean`
3. `cargo build`
4. `cargo install diesel_cli --no-default-features --features postgres`

### Deployment

> Deployment has already been automated so we should not be doing this anymore

1. Install [cargo-lambda](https://www.cargo-lambda.info/), advisedly with `cargo install cargo-lambda`.

2. Set [AWS credentials](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html) in `~/.aws/credentials` (and optionally region)

3. Build the project with `cargo lambda build --release`

4. Deploy with `cargo lambda deploy --env-file .env --enable-function-url graph-api`

5. You can now use the endpoint returned by the previous command.
### Generate ORM

1. Make sure you have libpg installed (on Gentoo, `emerge dev-db/postgresql`).

2. Install diesel-cli with `cargo install diesel_cli --no-default-features --features postgres`.

3. Run `diesel print-schema > src/models/schema.rs`.
