pub mod error;
pub mod graph;
pub mod models;

use std::sync::Arc;

use dotenv;
pub use graph::{
    context::GeneralContext,
    root::{self, Schema},
};
use juniper::http::{GraphQLRequest, GraphQLResponse};
use lambda_http::{run, service_fn, Body, Error, Request, Response};

/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(
    event: Request,
    schema: &Schema,
    context: &GeneralContext,
) -> Result<Response<Body>, Error> {
    println!("Handling {} request...", event.method());
    let response = Response::builder();

    let mut unique_context = context.new_unique_context().await;

    match event.method().as_str() {
        "OPTIONS" => response
            .status(200)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "POST")
            .header("Access-Control-Allow-Headers", "Content-Type")
            .body(Body::Empty),
        "POST" => {
            // get token from header
            let token = event
                .headers()
                .get("Authorization")
                .map(|v| v.to_str().unwrap().to_string());
            let user_result = match token {
                Some(token) => {
                    let user =
                        models::user::User::authenticate_from_token(&unique_context, token).await;
                    match user {
                        Ok(user) => Ok(Some(user)),
                        Err(e) => Err(GraphQLResponse::error(e)),
                    }
                }
                None => Ok(None),
            };

            let graphql_response = match user_result {
                Ok(user) => {
                    unique_context.user = user;
                    let request_string = std::str::from_utf8(event.body())?;
                    let graphql_request: GraphQLRequest = serde_json::from_str(request_string)?;
                    graphql_request.execute(schema, &unique_context).await
                }
                Err(e) => e,
            };

            let json = serde_json::to_string(&graphql_response)?;

            response
                .status(200)
                .header("content-type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(json.into())
        }
        _ => response
            .status(405)
            .header("Allow", "POST, OPTIONS")
            .body("405: Method not allowed - use POST instead.".into()),
    }
    .map_err(Error::from)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Starting server...");
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let schema = Arc::new(root::create_schema());
    let context = Arc::new(graph::context::GeneralContext::new().await?);

    run(service_fn(|event: Request| {
        let schema = schema.clone();
        let context = context.clone();

        async move { function_handler(event, &schema, &context).await }
    }))
    .await
}


mod tests {
    use chrono::{DateTime, Utc};
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    use juniper::futures::StreamExt;
    use mongodb::{Client, Collection, bson::oid::ObjectId, options::FindOptions};

    use crate::models::applications::Application;
    use crate::models::user::User;
    use crate::models::schema::users::dsl::*;
    use crate::models::utils::jsonb::JsonB;
    use diesel::ExpressionMethods;

    #[ignore]
    #[tokio::test]
    async fn migration() -> Result<(), anyhow::Error> {
        let mut client = Client::with_uri_str("").await?;
        let db = client.database("production");

        dotenv::dotenv().ok();
        let context = crate::graph::context::GeneralContext::new().await?;
        let unique_context = context.new_unique_context().await;

        let mut conn = unique_context.diesel_pool.get().await?;

        // #[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
        // struct MongoUser {
        //     pub _id: ObjectId,
        //     pub emails: Vec<EmailData>,
        //     #[serde(with = "chrono::serde::ts_milliseconds")]
        //     pub joined: DateTime<Utc>,
        //     pub password: String,
        //     pub first_name: String,
        //     pub last_name: String,
        //     pub calling_code: String,
        //     pub country_code: String,
        //     pub phone_number: String,
        //     pub referrer: Option<ObjectId>,
        //     pub roles: Option<Vec<String>>,
        // }

        // #[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
        // pub struct EmailData {
        //     email: String,
        //     verified: bool,
        // }

        // let coll: Collection<MongoUser> = db.collection("users");

        // let mut cursor = coll.find(None, FindOptions::builder().limit(0).build()).await?;
        // while let Some(result) = cursor.next().await {
        //     match result {
        //         Ok(document) => {
        //             let user = User {
        //                 id: uuid::Uuid::new_v4(),
        //                 email: document.emails[0].email.clone(),
        //                 first_name: document.first_name.clone(),
        //                 last_name: document.last_name.clone(),
        //                 calling_code: document.calling_code.clone(),
        //                 country_code: document.country_code.clone(),
        //                 phone_number: document.phone_number.clone(),
        //                 password: document.password.clone(),
        //                 referrer: None,
        //                 role: document.roles.map(|roles| roles.get(0).cloned()).flatten(),
        //                 referrer_mongo: document.referrer.map(|other_id| other_id.to_hex()),
        //                 object_id: Some(document._id.to_hex()),
        //                 joined: document.joined,
        //             };

        //             let result = diesel::insert_into(users)
        //                 .values(&user)
        //                 .execute(&mut conn)
        //                 .await?;
        //         }
        //         Err(e) => println!("Error {:?}", e),
        //     }
        // }

        // now we want to get the citizenship_applications from mongo
        // find the user by the object_id, and reinsert the citizenship_applications into postgres
        // with the user_id from postgres
        #[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
        struct MongoCitizenshipApplicaiton {
            pub _id: ObjectId,
            pub user_id: ObjectId,
            #[serde(with = "chrono::serde::ts_milliseconds")]
            pub submitted_date: DateTime<Utc>,
            #[serde(with = "chrono::serde::ts_milliseconds")]
            pub date_of_birth: DateTime<Utc>,
            pub sex: String,
            pub first_name: String,
            pub last_name: String,
            pub skills: Vec<String>,
            pub occupations: Vec<String>,
            pub country_of_citizenship: Vec<String>,
            pub country_of_birth: String,
            pub country_of_residence: String,
            pub ethnic_groups: Vec<String>,
            pub citizenship_status: MongoCitizenshipStatus,
        }

        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
        pub enum MongoCitizenshipStatus {
            Pending,
            Approved,
            Rejected,
        }

        let coll: Collection<MongoCitizenshipApplicaiton> = db.collection("citizenship_applications");

        let mut cursor = coll.find(None, FindOptions::builder().limit(0).build()).await?;

        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let user = users
                        .filter(object_id.eq(document.user_id.to_hex()))
                        .first::<User>(&mut conn)
                        .await?;

                    let application_data = crate::models::citizenship_application::CitizenshipApplication {
                        user_id: user.id,
                        date_of_birth: document.date_of_birth,
                        sex: document.sex,
                        first_name: document.first_name,
                        last_name: document.last_name,
                        skills: document.skills,
                        occupations: document.occupations,
                        country_of_citizenship: document.country_of_citizenship,
                        country_of_birth: document.country_of_birth,
                        country_of_residence: document.country_of_residence,
                        citizenship_status: crate::models::citizenship_application::CitizenshipStatus::Pending,
                        ethnic_groups: document.ethnic_groups,
                    };

                    let application = Application {
                        application: JsonB(application_data),
                        created_at: document.submitted_date,
                        id: uuid::Uuid::new_v4(),
                        application_type: "citizenship".to_string(),
                    };

                    let result = diesel::insert_into(crate::models::schema::applications::table)
                        .values(&application)
                        .execute(&mut conn)
                        .await?;
                }
                Err(e) => println!("Error {:?}", e),
            }
        }

        Ok(())
    }
}