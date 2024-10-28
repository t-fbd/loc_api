#![allow(unused_imports)]

#![allow(unused_imports)]

use loc_api::{self, *};
use loc_api::simple_builders::ApiClient;
use loc_api::attribute_models::{AttributesSelect, ItemAttributes, SortField};
use loc_api::format_models::{MediaType, Format};
use loc_api::param_models::FacetReq;
use reqwest::blocking::Client;
use std::env;

/// Retrieves the base URL for the API, defaulting to the official LOC API.
/// Allows overriding via the `LOC_API_BASE_URL` environment variable.
/// Useful for testing against different environments.
#[test]
fn test_search_endpoint() {
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
    ).unwrap();

    // Assertions based on expected response structure
    let res = response.0.results.unwrap_or_else(|| {
        println!("No results found");
        vec![]
    });
    assert!(res.len() > 0, "Total results should be greater than 0");
    assert!(res.len() <= 25, "Results length should not exceed per_page limit");
}

#[test]
fn test_item_endpoint() {
    let client = ApiClient::new();
    let response = client.get_item(
        "2014717546",
        Some(ItemAttributes {
            cite_this: Some(true),
            item: Some(true),
            resources: Some(true),
        }),
    ).unwrap();

    // Assertions based on expected response structure
    if response.0.resources.is_some() {
        let resources = response.0.resources.clone().unwrap();
        match resources {
            ItemOrArray::Item(resource) => {
                assert!(resource.url.is_some(), "Url exists for resource");
            }
            ItemOrArray::Array(resources) => {
                assert!(resources.len() > 0, "Resource length should be greater than 0");
            }
        }
    } else {
        println!("No resources found");
    }

    if response.0.item.is_some() {
        let item = response.0.item.clone().unwrap();
        match item {
            ItemOrArray::Item(item) => {
                assert!(item.title.is_some(), "Title exists for item");
            }
            ItemOrArray::Array(items) => {
                assert!(items.len() > 0, "Item length should be greater than 0");
            }
        }
    }

    if response.0.cite_this.is_some() {
        let cite_this = response.0.cite_this.unwrap();
        match cite_this {
            ItemOrArray::Item(citation) => {
                assert!(citation.apa.is_some(), "Citation exists for item");
            }
            ItemOrArray::Array(citations) => {
                assert!(citations.len() > 0, "Citation length should be greater than 0");
            }
        }
    }

    assert!(response.0.item.is_some() || response.0.resources.is_some(), "Item or resources should be present");
}

#[test]
fn test_format_endpoint() {
    let client = ApiClient::new();
    let response = client.get_format(
        MediaType::Maps,
        Some("mountain"),
        Some(AttributesSelect {
            include: vec!["pagination".to_string(), "results".to_string()],
            exclude: vec![],
        }),
        Some(FacetReq { filters: vec!["subject:geography".to_string()] }),
        Some(10),
        Some(1),
        Some(SortField::TitleS),
    ).unwrap();

    // Assertions based on expected response structure
    let res = response.0.results.unwrap_or_else(|| {
        println!("No results found");
        vec![]
    });
    assert!(res.len() > 0, "Total results should be greater than 0");
    assert!(res.len() <= 10, "Results length should not exceed per_page limit");

    for item in res {
        if let Some(item_details) = item.item {
            match item_details {
                loc_api::ItemOrArray::Item(item) => {
                    if let Some(loc_api::StringOrArray::String(title)) = item.title {
                        assert!(!title.is_empty(), "Item should have a title");
                    } else if let Some(loc_api::StringOrArray::Array(titles)) = item.title {
                        for title in titles {
                            assert!(!title.is_empty(), "Each title in the array should have a length");
                        }
                    } else {
                        println!("Item title missing");
                        assert!(true, "Item title missing");
                    }
                }
                loc_api::ItemOrArray::Array(items) => {
                    assert!(!items.is_empty(), "Array of items should not be empty");
                    for item in items {
                        if let Some(loc_api::StringOrArray::String(title)) = item.title {
                            assert!(!title.is_empty(), "Item should have a title");
                        } else if let Some(loc_api::StringOrArray::Array(titles)) = item.title {
                            for title in titles {
                                assert!(!title.is_empty(), "Each title in the array should have a length");
                            }
                        } else {
                            println!("Item title missing");
                            assert!(true, "Item title missing");
                        }
                    }
                }
            }
        } else {
            println!("No item found");
        }
    }
}
