use loc_api::simple_builders::ApiClient;
use loc_api::attribute_models::ItemAttributes;

/// Example of retrieving an item response
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ApiClient::new();
    let response = client.get_item(
        "2014717546",
        Some(ItemAttributes {
            cite_this: Some(true),
            item: Some(true),
            resources: Some(true),
        }),
    )?;

    println!("url: {}", response.1);

    // Handle the item details
    if let Some(item) = response.0.item {
        println!("{:#?}", item);
    }

    if let Some(resources) = response.0.resources {
        println!("{:#?}", resources);
    }

    Ok(())
}
