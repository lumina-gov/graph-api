# graph-api

A serverless function that implements the GraphQL API for Lumina services. It uses a PostgeSQL database, Juniper to serve GraphQL queries and is designed to be deployed to AWS Lambda.

Further documentation for the AWS Rust runtime is available [here](https://github.com/awslabs/aws-lambda-rust-runtime).

### Configuration

Create a .env file based on the following template:

```
PG_DATABASE=
PG_HOST=
PG_USER=
PG_PASSWORD=
DATABASE_URL="postgres://${PG_USER}:${PG_PASSWORD}@${PG_HOST}/${PG_DATABASE}?sslmode=require"
JWT_SECRET=
STRIPE_SECRET_KEY=
OPENAI_KEY=

```

### Local Development

1. Clone the repository to your computer
2. Ensure you have the rust toolchain installed
3. Run `cargo lambda watch`
4. You can now use the endpoint `http://localhost:9000/lambda-url/graph-api/`

### Deployment

> Deployment has already been automated so we should not be doing this anymore

1. Install [cargo-lambda](https://www.cargo-lambda.info/), advisedly with `cargo install cargo-lambda`.

2. Set [AWS credentials](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html) in `~/.aws/credentials` (and optionally region)

3. Build the project with `cargo lambda build --release`

4. Deploy with `cargo lambda deploy --env-file .env --enable-function-url graph-api`

5. You can now use the endpoint returned by the previous command.

### Generate ORM

- Load .env with `set -o allexport; source .env; set +o allexport`

- Generate ORM: `sea-orm-cli generate entity -u $DATABASE_URL -o src/entities`

### Migrations

Edit schema.sql and then run `./migrate.sh`.