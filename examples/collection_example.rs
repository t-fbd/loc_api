use loc_api::loc_client::ApiClient;
use loc_api::param_models::FacetReq;
use loc_api::attribute_models::AttributesSelect;
use loc_api::attribute_models::SortField;

/// Example of retrieving results for a specific collection
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ApiClient::new();
    let response = client.get_collection(
        "civil war maps",
        None,
        AttributesSelect {
            include: vec!["pagination".to_string(), "results".to_string()],
            exclude: vec![],
        }.into(),
        FacetReq { filters: vec!["subject:geography".to_string()] }.into(),
        10.into(),
        1.into(),
        SortField::TitleS.into(),
    )?;

    println!("url: {}", response.1);

    // Handle the specific collection details
    if let Some(results) = response.0.results {
        for item in results {
            println!("{:#?}", item);
        }
    }

    Ok(())
}
