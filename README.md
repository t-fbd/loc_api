# loc-api

![Crates.io](https://img.shields.io/crates/v/loc_api)
![MIT License](https://img.shields.io/crates/l/loc_api)

**loc-api** is a Rust library that provides a comprehensive interface for interacting with the [Library of Congress (LOC) APIs](https://www.loc.gov/apis/). It simplifies the process of constructing API requests, managing parameters, and handling responses, enabling developers to seamlessly integrate LOC data into their Rust applications.

## Features

- **Comprehensive Endpoint Support**: Access various LOC API endpoints such as search, collections, items, and resources.
- **Flexible Parameter Management**: Easily construct and customize query parameters, including filtering, sorting, and formatting options.
- **Robust Response Models**: Utilize well-defined Rust structures to parse and interact with LOC API responses.
- **High-Level API Client**: Engage with LOC APIs through an `ApiClient` that handles endpoint construction and HTTP requests.
- **Customizable Configurations**: Override default settings like the base URL for testing or alternative deployments.

## Table of Contents

- [Installation](#installation)
- [Examples](#examples)
    - [Creating an API Client](#creating-an-api-client)
    - [Using the modules directly to construct a URL for querying specific types of `MediaType`](#using-the-modules-directly-to-construct-a-url-for-querying-specific-types-of-mediatype)
    - [Using the simple_builders module to make a search request](#using-the-simple_builders-module-to-make-a-search-request)
- [Modules](#modules)
- [License](#license)

## Installation

Add `loc-api` to your project's `Cargo.toml`:

```toml
[dependencies]
loc-api = "*"
```
OR

You can use `cargo add` to add the dependency to your `Cargo.toml`:

```sh
cargo add loc-api
```


This library depends on `reqwest`, `serde` and `serde_json` for making HTTP requests and serializing/deserializing JSON data, respectively:

```toml
[dependencies]
reqwest = { version = "0.11", features = ["blocking", "json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Examples

### Creating an API Client

First, initialize the `ApiClient`. You can optionally set the `LOC_API_BASE_URL` environment variable to override the default LOC API base URL.
Other methods of setting the base URL include using the `ApiClient::with_base_url` constructor or directly modifying the `simple_builders::DEFAULT_BASE_URL` constant.

```rust
use loc_api::simple_builders::ApiClient;

fn main() {
    let client = ApiClient::new();
    // Now you can use `client` to interact with the LOC APIs
}
```

### Using the modules directly to construct a URL for querying specific types of `MediaType`

```rust
use loc_api::{endpoints::*, param_models::*, format_models::*, attribute_models::*,
response_models::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
   let common_params = CommonParams {
       format: Some(Format::Json),
       attributes: Some(AttributesSelect {
           include: vec!["pagination".to_string(), "results".to_string()],
           exclude: vec![],
       }),
       query: Some("dog".to_string()),
       filter: Some(FacetReq {
           filters: vec!["subject:animals".to_string()],
       }),
       per_page: Some(25),
       page: Some(1),
       sort: Some(SortField::TitleS),
   };
   
   let format_params = CommonParams {
       format: Some(Format::Json),
       ..common_params.clone()
   };
   
   let format_endpoint = Endpoints::Format {
       format: MediaType::FilmAndVideos,
       params: format_params,
   };
   
   let url = format_endpoint.to_url().unwrap();
    
   println!("{}", url);

   Ok(())
}
```

### Using the simple_builders module to make a search request

```rust
use loc_api::simple_builders::ApiClient;
use loc_api::param_models::FacetReq;
use loc_api::attribute_models::AttributesSelect;
use loc_api::attribute_models::SortField;

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
            println!("{:#?}", item);
        }
    }

    Ok(())
}
```

## Modules

- `endpoints`

LOC API endpoints, providing enums and methods to construct URLs based on different endpoints and parameters.

- `param_models`

Contains structures representing query parameters applicable to multiple endpoints, such as `CommonParams`, `SearchParams`, `ItemParams`, and `ResourceParams`.

- `attribute_models`

Defines the possible attributes for query parameters that can be used in API requests, including structures like `AttributesSelect` and enums for sorting fields.

- `format_models`

Represents the possible response formats (`JSON` or `YAML`) and specific media types for endpoints like `audio`, `books`, `maps`, etc.

- `response_models`

Structures that model the responses from LOC API endpoints, such as `SearchResultResponse`, `ItemResponse`, `FormatResponse`, and others.

- `simple_builders`

Provides a high-level `ApiClient` for interacting with the LOC API, abstracting endpoint construction, parameter management, and HTTP requests.

## Other Projects

- [cdg_api](https://crates.io/crates/cdg_api): A Rust library for interacting with the Congress.gov API.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Repository

[t-fbd](https://github.com/t-fbd/loc_api)
```
