use loc_api::simple_builders::ApiClient;
use loc_api::param_models::FacetReq;
use loc_api::attribute_models::AttributesSelect;
use loc_api::attribute_models::SortField;

/// Example of searching for items and retrieving the results
fn search_example() -> Result<(), Box<dyn std::error::Error>> {
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

use loc_api::attribute_models::ItemAttributes;

/// Example of retrieving an item response
fn item_example() -> Result<(), Box<dyn std::error::Error>> {
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

use loc_api::format_models::MediaType;

/// Example of retrieving search results for a specific format of media
fn mediaformat_example() -> Result<(), Box<dyn std::error::Error>> {
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
            println!("{:?}", item);
        }
    }

    Ok(())
}

/// Example of retrieving search results for all collections related to a search term
fn collections_example() -> Result<(), Box<dyn std::error::Error>> {
    let client = ApiClient::new();
    let response = client.get_collections(
        Some("civil war"),
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
            println!("{:?}", collection);
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ApiClient::new();
    let response = client.get_collection(
        "civil war maps",
        None,
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

    // Handle the specific collection details
    if let Some(results) = response.0.results {
        for item in results {
            println!("{:#?}", item);
        }
    }

    Ok(())
}
