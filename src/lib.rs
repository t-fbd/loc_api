//!# Rust API for the [Library of Congress](https://www.loc.gov/apis/)
//!
//!## Overview
//!
//!This library provides a Rust interface for the [Library of Congress
//!API](https://www.loc.gov/apis/). It allows users to interact with the API
//!endpoints, construct URLs, and parse the responses into Rust data structures.
//!
//!## Features
//!
//!- **Endpoints**: Enumerated types for different API endpoints.
//!- **Parameters**: Structs for query parameters and attributes.
//!- **Formats**: Enumerated types for response formats.
//!- **Attributes**: Structs for selecting attributes in the response.
//!- **Responses**: Structs for parsing API responses.
//!- **Pagination**: Structs for pagination information in the response.
//!- **Filters**: Structs for facet filters in the response.
//!- **Sorting**: Enumerated types for sorting fields.
//!
//!## Usage
//!
//!The only module in this library that will auto-handle the API request is the `simple_builders`
//!module. The other modules are used to construct the request URL and parse the response.
//!The `simple_builders` module provides a simple interface to make requests and get responses
//!using these endpoints and parameters models.
//!
//!## Examples
//!
//!### Using the modules directly to construct a URL for querying specific types of `MediaType`
//!
//!```rust
//!use loc_api::{endpoints::*, param_models::*, format_models::*, attribute_models::*,
//!response_models::*};
//!
//!fn main() -> Result<(), Box<dyn std::error::Error>> {
//!   let common_params = CommonParams {
//!       format: Some(Format::Json),
//!       attributes: Some(AttributesSelect {
//!           include: vec!["pagination".to_string(), "results".to_string()],
//!           exclude: vec![],
//!       }),
//!       query: Some("dog".to_string()),
//!       filter: Some(FacetReq {
//!           filters: vec!["subject:animals".to_string()],
//!       }),
//!       per_page: Some(25),
//!       page: Some(1),
//!       sort: Some(SortField::TitleS),
//!   };
//!   
//!   let format_params = CommonParams {
//!       format: Some(Format::Json),
//!       ..common_params.clone()
//!   };
//!   
//!   let format_endpoint = Endpoints::Format {
//!       format: MediaType::FilmAndVideos,
//!       params: format_params,
//!   };
//!   
//!   let url = format_endpoint.to_url().unwrap();
//!    
//!   println!("{}", url);
//!
//!   Ok(())
//!}
//!```
//!
//!### Using the simple_builders module to make a search request
//!
//!```rust
//!use loc_api::simple_builders::ApiClient;
//!use loc_api::param_models::FacetReq;
//!use loc_api::attribute_models::AttributesSelect;
//!use loc_api::attribute_models::SortField;
//!
//!fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    let client = ApiClient::new();
//!    let response = client.search(
//!        "baseball",
//!        false,
//!        Some(AttributesSelect {
//!            include: vec!["pagination".to_string(), "results".to_string()],
//!            exclude: vec![],
//!        }),
//!        Some(FacetReq { filters: vec!["subject:sports".to_string()] }),
//!        Some(25),
//!        Some(1),
//!        Some(SortField::DateDesc),
//!    )?;
//!
//!    println!("url: {}", response.1);
//!
//!    // Handle the search results
//!    if let Some(results) = response.0.results {
//!        for item in results {
//!            println!("{:#?}", item);
//!        }
//!    }
//!
//!    Ok(())
//!}
//!```
//!

pub mod attribute_models;
pub mod endpoints;
pub mod format_models;
pub mod param_models;
pub mod response_models;
pub mod simple_builders;
