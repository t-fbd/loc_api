# loc-api

![Crates.io](https://img.shields.io/crates/v/cdg_api)
![License](https://img.shields.io/crates/l/cdg_api)
![MIT License](https://img.shields.io/crates/l/cdg_api)

**loc-api** is a Rust library that provides a comprehensive interface for interacting with the [Library of Congress (LOC) APIs](https://www.loc.gov/apis/). It simplifies the process of constructing API requests, managing parameters, and handling responses, enabling developers to seamlessly integrate LOC data into their Rust applications.

## Features

- **Comprehensive Endpoint Support**: Access various LOC API endpoints such as search, collections, items, and resources.
- **Flexible Parameter Management**: Easily construct and customize query parameters, including filtering, sorting, and formatting options.
- **Robust Response Models**: Utilize well-defined Rust structures to parse and interact with LOC API responses.
- **High-Level API Client**: Engage with LOC APIs through an `ApiClient` that handles endpoint construction and HTTP requests.
- **Customizable Configurations**: Override default settings like the base URL for testing or alternative deployments.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
  - [Creating an API Client](#creating-an-api-client)
  - [Performing a Search](#performing-a-search)
  - [Retrieving an Item](#retrieving-an-item)
  - [Fetching a Specific Format](#fetching-a-specific-format)
  - [Managing Collections](#managing-collections)
- [Modules](#modules)
- [License](#license)

## Installation

Add `loc-api` to your project's `Cargo.toml`:

```toml
[dependencies]
loc-api = "0.1.0"
```
~OR~

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

## Usage

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

### Performing a Search

Use the `search` method to perform general searches across the LOC website.

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

    // Handle the search results
    if let Some(results) = response.results {
        for item in results {
            println!("{:?}", item);
        }
    }

    Ok(())
}
```

### Retrieving an Item

Fetch detailed information about a specific item using its `item_id`.

```rust
use loc_api::simple_builders::ApiClient;
use loc_api::attribute_models::ItemAttributes;

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

    // Handle the item details
    if let Some(item) = response.item {
        println!("{:?}", item);
    }

    if let Some(resources) = response.resources {
        println!("{:?}", resources);
    }

    Ok(())
}
```

### Fetching a Specific Format

Retrieve items of a specific format, such as maps or books.

```rust
use loc_api::simple_builders::ApiClient;
use loc_api::param_models::FacetReq as FRequest;
use loc_api::attribute_models::{AttributesSelect, SortField};
use loc_api::format_models::FormatType;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ApiClient::new();
    let response = client.get_format(
        FormatType::Maps,
        "mountain",
        Some(AttributesSelect {
            include: vec!["pagination".to_string(), "results".to_string()],
            exclude: vec![],
        }),
        Some(FRequest { filters: vec!["subject:geography".to_string()] }),
        Some(10),
        Some(1),
        Some(SortField::TitleS),
    )?;

    // Handle the format-specific results
    if let Some(results) = response.results {
        for item in results {
            println!("{:?}", item);
        }
    }

    Ok(())
}
```

### Managing Collections

#### Retrieve All Collections

Fetch all available collections from the LOC API.

```rust
use loc_api::simple_builders::ApiClient;
use loc_api::param_models::FacetReq as FRequest;
use loc_api::attribute_models::{AttributesSelect, SortField};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ApiClient::new();
    let response = client.get_collections(
        "maps",
        Some(AttributesSelect {
            include: vec!["pagination".to_string(), "results".to_string()],
            exclude: vec![],
        }),
        Some(FRequest { filters: vec!["subject:geography".to_string()] }),
        Some(10),
        Some(1),
        Some(SortField::TitleS),
    )?;

    // Handle the collections
    if let Some(results) = response.results {
        for collection in results {
            println!("{:?}", collection);
        }
    }

    Ok(())
}
```

#### Retrieve a Specific Collection

Fetch detailed information about a specific collection using its name.

```rust
use loc_api::simple_builders::ApiClient;
use loc_api::param_models::FacetReq as FRequest;
use loc_api::attribute_models::{AttributesSelect, SortField};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ApiClient::new();
    let response = client.get_collection(
        "maps",
        "mountain",
        Some(AttributesSelect {
            include: vec!["pagination".to_string(), "results".to_string()],
            exclude: vec![],
        }),
        Some(FRequest { filters: vec!["subject:geography".to_string()] }),
        Some(10),
        Some(1),
        Some(SortField::TitleS),
    )?;

    // Handle the specific collection details
    if let Some(results) = response.results {
        for item in results {
            println!("{:?}", item);
        }
    }

    Ok(())
}
```

## Modules

### `endpoints`

LOC API endpoints, providing enums and methods to construct URLs based on different endpoints and parameters.

### `param_models`

Contains structures representing query parameters applicable to multiple endpoints, such as `CommonParams`, `SearchParams`, `ItemParams`, and `ResourceParams`.

### `attribute_models`

Defines the possible attributes for query parameters that can be used in API requests, including structures like `AttributesSelect` and enums for sorting fields.

### `format_models`

Represents the possible response formats (`JSON` or `YAML`) and specific media types for endpoints like `audio`, `books`, `maps`, etc.

### `response_models`

Structures that model the responses from LOC API endpoints, such as `SearchResultResponse`, `ItemResponse`, `FormatResponse`, and others.

### `simple_builders`

Provides a high-level `ApiClient` for interacting with the LOC API, abstracting endpoint construction, parameter management, and HTTP requests.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
```
