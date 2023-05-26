# graph-api

A serverless function that implements the GraphQL API for Lumina services. It uses a PostgeSQL database, `async-graphql` to serve GraphQL queries and is designed to be deployed to AWS Lambda.

### Where things go

if it updates the database
-> graphql/mutations

if it reads the database and is a top-level query
-> graphql/queries

if it reads the database and is a method of an object
-> graphlql/types

### Configuration

Create a .env file based on the following template:

```
DATABASE_URL="postgres://${PG_USER}:${PG_PASSWORD}@${PG_HOST}/${PG_DATABASE}?sslmode=require"
JWT_SECRET=
STRIPE_SECRET_KEY=
OPENAI_KEY=
SENDGRID_KEY=
```

### Local Development

1. Clone the repository to your computer
2. Install Rust toolchain
3. Install Docker Engine
4. Start docker
5. Write a test for new feature or development
6. Run the test

### Deployment

> Deployment has been automated via github actions

### Migrations

Edit schema.sql and then run `./migrate.sh`.
