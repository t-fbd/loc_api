use loc_api::simple_builders::ApiClient;
use loc_api::param_models::FacetReq;
use loc_api::attribute_models::AttributesSelect;
use loc_api::attribute_models::SortField;
use loc_api::format_models::MediaType;

/// Example of retrieving search results for a specific format of media
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ApiClient::new();
    let response = client.get_format(
        MediaType::Maps,
        Some("usa"),
        Some(AttributesSelect {
            include: vec!["pagination".to_string(), "results".to_string()],
            exclude: vec![],
        }),
        Some(FacetReq { filters: vec!["subject:geography".to_string()] }),
        Some(10),
        Some(1),
        Some(SortField::TitleS),
    )?;

    println!("url: {}", response.1);

    // Handle the format-specific results
    if let Some(results) = response.0.results {
        for item in results {
            println!("{:#?}", item);
        }
    }

    Ok(())
}
