use loc_api::loc_client::ApiClient;
use loc_api::attribute_models::AttributesSelect;
use loc_api::attribute_models::SortField;

/// Example of retrieving results for all collections
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ApiClient::new();
    let response = client.get_collections(
        None,
        Some(AttributesSelect {
            include: vec!["pagination".to_string(), "results".to_string()],
            exclude: vec![],
        }),
        None,
        Some(10),
        Some(1),
        Some(SortField::TitleS),
    )?;

    println!("url: {}", response.1);

    // Handle the collections
    if let Some(results) = response.0.results {
        for collection in results {
            println!("{:#?}", collection);
        }
    }

    Ok(())
}

