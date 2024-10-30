use loc_api::loc_client::ApiClient;
use loc_api::param_models::FacetReq;
use loc_api::attribute_models::AttributesSelect;
use loc_api::attribute_models::SortField;

/// Example of searching for items and retrieving the results
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ApiClient::new();
    let response = client.search(
        "baseball",
        false,
        Some(AttributesSelect {
            include: vec!["pagination".to_string(), "results".to_string()],
            exclude: vec![],
        }),
        Some(FacetReq { filters: vec!["subject:sports".to_string()] }),
        Some(25),
        Some(1),
        Some(SortField::DateDesc),
    )?;

    println!("url: {}", response.1);

    // Handle the search results
    if let Some(results) = response.0.results {
        for item in results {
            println!("{:?}", item);
        }
    }

    Ok(())
}
