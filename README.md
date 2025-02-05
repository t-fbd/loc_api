# loc-api

![Crates.io](https://img.shields.io/crates/v/loc_api)
![MIT License](https://img.shields.io/crates/l/loc_api)

**loc-api** is a Rust library that provides an interface for interacting with the [Library of Congress (LOC) APIs](https://www.loc.gov/apis/). It simplifies the process of constructing API requests, 
managing parameters, and handling responses, enabling developers to seamlessly integrate LOC data into their Rust applications.

Work in progress, not everything is implemented yet, however, the library is functional and can be used to interact with the LOC API. 
Check out the [examples](#examples) below to see how to use the library. The included [examples directory](examples/) within the repository contains a few more examples as well.

Please report any issues or bugs, I'm sure there are many. I'm also open to suggestions and contributions.

## Installation

Add `loc-api` to your project's `Cargo.toml`:

```toml
[dependencies]
loc-api = "1.0.8"
```
OR

You can use `cargo add` to add the dependency to your `Cargo.toml`:

```sh
cargo add loc-api
```


This library depends on [`reqwest`], [`serde`] and [`serde_json`] for making HTTP requests and serializing/deserializing JSON data, respectively:

```toml
[dependencies]
reqwest = { version = "0.11", features = ["blocking", "json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Examples

### Creating an API Client

First, initialize the [`ApiClient`]. You can optionally set the [`LOC_API_BASE_URL`] environment variable to override the default LOC API base URL.
Other methods of setting the base URL include using the `ApiClient::with_base_url` constructor or directly modifying the `loc_client::DEFAULT_BASE_URL` constant.

```rust
use loc_api::loc_client::ApiClient;

fn main() {
    let client = ApiClient::new();
    // Now you can use [`client`] to interact with the LOC APIs
}
```

### Using the modules directly to construct a URL for querying specific types of [`MediaType`]

```rust
use loc_api::{endpoints::*, param_models::*, format_models::*, attribute_models::*,
response_models::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
   let common_params = CommonParams {
       format: Format::Json.into(),
       attributes: AttributesSelect {
           include: vec!["pagination".to_string(), "results".to_string()],
           exclude: vec![],
       }.into(),
       query: "dog".to_string().into(),
       filter: FacetReq {
           filters: vec![Facet::Subject {value: "animals".to_string()}]
       }.into(),
       per_page: 25.into(),
       page: 1.into(),
       sort: SortField::TitleS.into(),
   };
   
   let format_endpoint = Endpoints::Format {
       format: MediaType::FilmAndVideos,
       params: common_params,
   };
   
   let url = format_endpoint.to_url().unwrap();
    
   println!("{}", url);

   Ok(())
}
```

### Using the loc_client module to make a search request

```rust
use loc_api::loc_client::ApiClient;
use loc_api::param_models::{Facet, FacetReq};
use loc_api::attribute_models::AttributesSelect;
use loc_api::attribute_models::SortField;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ApiClient::new();
    let response = client.search(
        "baseball",
        false,
        AttributesSelect {
            include: vec!["pagination".to_string(), "results".to_string()],
            exclude: vec![],
        }.into(),
        FacetReq { filters: vec![Facet::Subject {value: "sports".to_string()}] }.into(),
        25.into(),
        1.into(),
        SortField::DateDesc.into(),
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

## Related Projects

- **[cdg_api]: A Rust library for interacting with the Congress.gov API.
    - [crate](https://crates.io/crates/cdg_api)
    - [repository](https://github.com/t-fbd/cdg_api)
   
## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Repository

[t-fbd](https://github.com/t-fbd/loc_api)

## Acknowledgements

The data is sourced from the [U.S. Library of Congress](https://www.loc.gov/apis/).

## Contact

For questions or feedback, please contact me on [github](https://www.github.com/t-fbd).

If you find this project helpful, consider donating [PayPal](https://paypal.me/imturn?country.x=US&locale.x=en_US).
