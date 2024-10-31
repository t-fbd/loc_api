use loc_api::loc_client::ApiClient;
use loc_api::param_models::FacetReq;
use loc_api::attribute_models::AttributesSelect;

/// Example of searching for items and retrieving the results
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ApiClient::new();
    let response = client.search(
        "constitution",
        true,
        AttributesSelect {
            include: vec!["pagination".to_string(), "results".to_string()],
            exclude: vec![],
        }.into(),
        FacetReq { 
            filters: vec![
                "subject:united+states".to_string(), 
                "online-format:online+text".to_string()
            ] 
        }.into(),
        25.into(),
        1.into(),
        None,
    )?;

    println!("url: {}", response.1);

    // Handle the search results
    if let Some(results) = response.0.results {
        for item in results {
            println!("{:#?}", item);
        }
    }

    Ok(())
}
