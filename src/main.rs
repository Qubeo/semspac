use aragog::schema::DatabaseSchema;
use aragog::{AuthMode, DatabaseConnection, DatabaseRecord, OperationOptions, Record};
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate aragog;
// extern crate env_logger;
use aragog::query::{Comparison, Filter};

#[derive(Serialize, Debug, Deserialize, Clone, Record)]
#[collection_name = "test"]
pub struct Test {
    pub dog: String,
    pub cat: String,
}

#[tokio::main]
async fn main() {
    println!("a");

    let db_connection = DatabaseConnection::builder()
        // You can specify a host and credentials with this method.
        // Otherwise, the builder will look for the env vars: `DB_HOST`, `DB_NAME`, `DB_USER` and `DB_PASSWORD`.
        .with_credentials("http://localhost:8529", "SST", "revial", "120888")
        // You can specify a authentication mode between `Basic` and `Jwt`
        // Otherwise the default value will be used (`Basic`).
        .with_auth_mode(AuthMode::Basic)
        // You can specify some operations options that will be used for every `write` operations like
        // `create`, `save` and `delete`.
        .with_operation_options(OperationOptions::default())
        // You can specify a schema path to initialize the database connection
        // Otherwise the env var `SCHEMA_PATH` or the default value `config/db/schema.yaml` will be used.
        // .with_schema_path("config/db/schema.yaml")
        // If you prefer you can use your own custom schema
        // .with_schema(DatabaseSchema::default())
        // The schema wil silently apply to the database, useful only if you don't use the CLI and migrations
        .apply_schema()
        // You then need to build the connection
        .build()
        .await
        .unwrap();

    let mut test = Test {
        dog: "blub".to_string(),
        cat: "mleouw".to_string(),
    };

    // We create the user
    // TODO: Make idempotent - this isn't.
    let mut test_record = DatabaseRecord::create(test, &db_connection).await.unwrap();
    // You can access and edit the document
    // user_record.dog = String::from("LeRevenant1524356");
    // And directly save it
    test_record.save(&db_connection).await.unwrap();

    let document_key = test_record.key();
    // The key can be used to retrieve documents, returning again a `DatabaseRecord<User>`
    let found_test = Test::find(document_key, &db_connection).await.unwrap();
    // We can access the document data from the database record
    println!("Key: {:?}. Document: {:?} ", document_key, found_test);
    println!("Dog: {:?}, cat: {:?}: ", found_test.dog, found_test.cat);
    // Or delete it
    // user_record.delete(&database_connection).await.unwrap();
}
