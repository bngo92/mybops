#[cfg(feature = "azure")]
#[tokio::main]
async fn main() {
    use std::{
        fs,
        sync::{Arc, RwLock},
    };

    use azure_data_cosmos::{clients::CosmosClient, resources::permission::AuthorizationToken};
    use cosmos::CosmosSessionClient;
    use mybops::{
        List,
        storage::{CosmosQuery, QueryDocumentsBuilder, SessionClient},
    };
    use mybops_web::Item;
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let authorization_token =
        AuthorizationToken::primary_from_base64(&master_key).expect("cosmos config");
    let db = CosmosClient::new(account, authorization_token).database_client("topbops");
    let client = CosmosSessionClient::new(db.clone(), Arc::new(RwLock::new(None)));
    let mut builder =
        QueryDocumentsBuilder::new("items", CosmosQuery::new("SELECT * FROM c".to_owned()));
    builder.query_cross_partition = true;
    let items: Vec<Item> = client.query_documents(builder).await.unwrap();
    fs::write("items.json", serde_json::to_string_pretty(&items).unwrap()).unwrap();
    let mut builder =
        QueryDocumentsBuilder::new("lists", CosmosQuery::new("SELECT * FROM c".to_owned()));
    builder.query_cross_partition = true;
    let lists: Vec<List> = client.query_documents(builder).await.unwrap();
    fs::write("lists.json", serde_json::to_string_pretty(&lists).unwrap()).unwrap();
}
#[cfg(not(feature = "azure"))]
fn main() {}
