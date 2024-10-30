//! The `simple_builders` module provides a high-level interface for interacting with
//! the Library of Congress API. It abstracts the complexities of endpoint construction,
//! parameter management, and HTTP requests, offering straightforward methods for common operations.
//!
//! All methods return a tuple containing the deserialized JSON response and the final URL used

use crate::{response_models::*, param_models::*, attribute_models::*, format_models::*, endpoints::*};
use std::error::Error;
use reqwest::blocking::Client;
use std::env;

pub const DEFAULT_BASE_URL: &str = "https://www.loc.gov/";

/// A client for interacting with the Library of Congress API.
///
/// Provides high-level methods to perform API requests without manually constructing
/// parameters or URLs.
pub struct ApiClient {
    base_url: String,
    client: Client,
}

impl ApiClient {
    /// Creates a new `ApiClient` instance.
    ///
    /// The base URL can be overridden by setting the `LOC_API_BASE_URL` environment variable.
    ///
    /// base_url
    ///
    /// # Examples
    ///
    /// ```rust
    /// use loc_api::simple_builders::ApiClient;
    ///
    /// let client = ApiClient::new();
    /// ```
    pub fn new() -> Self {
        let base_url = env::var("LOC_API_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());
        let client = Client::new();
        ApiClient { base_url, client }
    }

    /// Performs a search query using the `/search/` endpoint.
    ///
    /// # Parameters
    ///
    /// - `query`: The search query string.
    /// - `include_collections`: Whether to include collections in the search results.
    /// - `attributes`: Attributes to include or exclude in the response.
    /// - `filters`: Facet filters to apply.
    /// - `per_page`: Number of results per page.
    /// - `page`: Page number to retrieve.
    /// - `sort`: Sorting field.
    ///
    /// # Returns
    ///
    /// Returns a `SearchResultResponse` on success.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use loc_api::simple_builders::ApiClient;
    /// use loc_api::param_models::{FacetReq, SearchParams};
    /// use loc_api::attribute_models::{AttributesSelect, SortField};
    /// use loc_api::format_models::Format;
    ///
    /// let client = ApiClient::new();
    /// let response = client.search(
    ///     "baseball",
    ///     false,
    ///     Some(AttributesSelect {
    ///         include: vec!["pagination".to_string(), "results".to_string()],
    ///         exclude: vec![],
    ///     }),
    ///     Some(FacetReq { filters: vec!["subject:sports".to_string()] }),
    ///     Some(25),
    ///     Some(1),
    ///     Some(SortField::DateDesc),
    /// ).unwrap();
    /// ```
    pub fn search(
        &self,
        query: &str,
        include_collections: bool,
        attributes: Option<AttributesSelect>,
        filters: Option<FacetReq>,
        per_page: Option<u32>,
        page: Option<u32>,
        sort: Option<SortField>,
    ) -> Result<(SearchResultResponse, String), Box<dyn Error>> {
        let common_params = CommonParams {
            format: Some(Format::default()),
            attributes,
            query: Some(query.to_string().replace(" ", "+")),
            filter: filters,
            per_page,
            page,
            sort,
        };

        let search_params = SearchParams {
            common: common_params,
            include_collections,
        };

        let endpoint = Endpoints::Search(search_params);
        let url = endpoint.to_url()?;

        // Replace the default base URL with the client's base_url
        let final_url = self.replace_base_url(&url)?;

        let response = self.client.get(&final_url).send()?.error_for_status()?;
        let json = response.json::<SearchResultResponse>()?;
        Ok((json, final_url))
    }

    /// Retrieves detailed information about a specific item using the `/item/{item_id}/` endpoint.
    ///
    /// # Parameters
    ///
    /// - `item_id`: The unique identifier of the item.
    /// - `attributes`: Attributes to include in the response.
    ///
    /// # Returns
    ///
    /// Returns an `ItemResponse` on success.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use loc_api::simple_builders::ApiClient;
    /// use loc_api::param_models::ItemParams;
    /// use loc_api::attribute_models::ItemAttributes;
    /// use loc_api::format_models::Format;
    ///
    /// let client = ApiClient::new();
    /// let response = client.get_item(
    ///     "2014717546",
    ///     Some(ItemAttributes {
    ///         cite_this: Some(true),
    ///         item: Some(true),
    ///         resources: Some(true),
    ///     }),
    /// ).unwrap();
    /// ```
    pub fn get_item(
        &self,
        item_id: &str,
        attributes: Option<ItemAttributes>,
    ) -> Result<(ItemResponse, String), Box<dyn Error>> {
        let item_params = ItemParams {
            format: Some(Format::default()),
            attributes,
        };

        let endpoint = Endpoints::Item {
            item_id: item_id.to_string(),
            params: item_params,
        };
        let url = endpoint.to_url()?;

        // Replace the default base URL with the client's base_url
        let final_url = self.replace_base_url(&url)?;

        let response = self.client.get(&final_url).send()?.error_for_status()?;
        let json = response.json::<ItemResponse>()?;
        Ok((json, final_url))
    }

    /// Retrieves items of a specific format using the `/{format}/` endpoint.
    ///
    /// # Parameters
    ///
    /// - `format_type`: The specific format type (e.g., `MediaType::Maps`).
    /// - `query`: The search query string.
    /// - `attributes`: Attributes to include or exclude in the response.
    /// - `filters`: Facet filters to apply.
    /// - `per_page`: Number of results per page.
    /// - `page`: Page number to retrieve.
    /// - `sort`: Sorting field.
    ///
    /// # Returns
    ///
    /// Returns a `FormatResponse` on success.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use loc_api::simple_builders::ApiClient;
    /// use loc_api::param_models::FacetReq;
    /// use loc_api::attribute_models::{AttributesSelect, SortField};
    /// use loc_api::format_models::{Format, MediaType};
    ///
    /// let client = ApiClient::new();
    /// let response = client.get_format(
    ///     MediaType::Maps,
    ///     Some("mountain"),
    ///     Some(AttributesSelect {
    ///         include: vec!["pagination".to_string(), "results".to_string()],
    ///         exclude: vec![],
    ///     }),
    ///     Some(FacetReq { filters: vec!["subject:geography".to_string()] }),
    ///     Some(10),
    ///     Some(1),
    ///     Some(SortField::TitleS),
    /// ).unwrap();
    /// ```
    pub fn get_format(
        &self,
        format_type: MediaType,
        query: Option<&str>,
        attributes: Option<AttributesSelect>,
        filters: Option<FacetReq>,
        per_page: Option<u32>,
        page: Option<u32>,
        sort: Option<SortField>,
    ) -> Result<(FormatResponse, String), Box<dyn Error>> {
        let query = if let Some(q) = query { Some(q.replace(" ", "+")) } else { None };
        let common_params = CommonParams {
            format: Some(Format::default()),
            attributes,
            query,
            filter: filters,
            per_page,
            page,
            sort,
        };

        let endpoint = Endpoints::Format {
            format: format_type,
            params: common_params,
        };
        let url = endpoint.to_url()?;

        // Replace the default base URL with the client's base_url
        let final_url = self.replace_base_url(&url)?;

        let response = self.client.get(&final_url).send()?.error_for_status()?;
        let json = response.json::<FormatResponse>()?;
        Ok((json, final_url))
    }

    /// Retrieves detailed information about a specific collection using `/collections/{name_of_collection}/`.
    ///
    /// # Parameters
    ///
    /// - `collection_name`: The name of the collection in kebab-case, auto-conversion will
    /// replace spaces and '_' with hyphens.
    /// - `query`: The search query string.
    /// - `attributes`: Attributes to include or exclude in the response.
    /// - `filters`: Facet filters to apply.
    /// - `per_page`: Number of results per page.
    /// - `page`: Page number to retrieve.
    /// - `sort`: Sorting field.
    ///
    /// # Returns
    ///
    /// Returns a `CollectionResponse` on success.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use loc_api::simple_builders::ApiClient;
    /// use loc_api::param_models::FacetReq;
    /// use loc_api::attribute_models::{AttributesSelect, SortField};
    /// use loc_api::format_models::Format;
    /// use loc_api::response_models::CollectionResponse;
    ///
    /// let client = ApiClient::new();
    /// let response = match client.get_collection(
    ///     "maps",
    ///     Some("mountain"),
    ///     Some(AttributesSelect {
    ///         include: vec!["pagination".to_string(), "results".to_string()],
    ///         exclude: vec![],
    ///     }),
    ///     Some(FacetReq { filters: vec!["subject:geography".to_string()] }),
    ///     Some(10),
    ///     Some(1),
    ///     Some(SortField::TitleS),
    /// ) {
    ///     Ok(response) => {
    ///         println!("URL: {}", response.1);
    ///         response.0
    ///     },
    ///     Err(e) => {
    ///         eprintln!("Error: {}", e);   
    ///         CollectionResponse::default()
    ///     }
    /// };
    /// ```
    pub fn get_collection(
        &self,
        collection_name: &str,
        query: Option<&str>,
        attributes: Option<AttributesSelect>,
        filters: Option<FacetReq>,
        per_page: Option<u32>,
        page: Option<u32>,
        sort: Option<SortField>,
    ) -> Result<(CollectionResponse, String), Box<dyn Error>> {
        let query = if let Some(q) = query { Some(q.replace(" ", "+")) } else { None };

        let common_params = CommonParams {
            format: Some(Format::default()),
            attributes,
            query,
            filter: filters,
            per_page,
            page,
            sort,
        };

        let endpoint = Endpoints::Collection {
            name: collection_name.to_string().replace(" ", "-").replace("_", "-"),
            params: common_params,
        };

        let url = endpoint.to_url()?;

        // Replace the default base URL with the client's base_url
        let final_url = self.replace_base_url(&url)?;

        let response = self.client.get(&final_url).send()?.error_for_status()?;
        let json = response.json::<CollectionResponse>()?;
        Ok((json, final_url))
    }

    /// Retrieves all collections using the `/collections/` endpoint.
    ///
    /// # Parameters
    ///
    /// - `query`: The search query string.
    /// - `attributes`: Attributes to include or exclude in the response.
    /// - `filters`: Facet filters to apply.
    /// - `per_page`: Number of results per page.
    /// - `page`: Page number to retrieve.
    /// - `sort`: Sorting field.
    ///
    /// # Returns
    ///
    /// Returns a `CollectionsResponse` on success.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use loc_api::simple_builders::ApiClient;
    /// use loc_api::param_models::FacetReq;
    /// use loc_api::attribute_models::{AttributesSelect, SortField};
    /// use loc_api::format_models::Format;
    ///
    /// let client = ApiClient::new();
    /// let response = client.get_collections(
    ///     Some("mountain"),
    ///     Some(AttributesSelect {
    ///         include: vec!["pagination".to_string(), "results".to_string()],
    ///         exclude: vec![],
    ///     }),
    ///     Some(FacetReq { filters: vec!["subject:geography".to_string()] }),
    ///     Some(10),
    ///     Some(1),
    ///     Some(SortField::TitleS),
    /// ).unwrap();
    /// ```
    pub fn get_collections(
        &self,
        query: Option<&str>,
        attributes: Option<AttributesSelect>,
        filters: Option<FacetReq>,
        per_page: Option<u32>,
        page: Option<u32>,
        sort: Option<SortField>,
    ) -> Result<(CollectionsResponse, String), Box<dyn Error>> {
        let query = if let Some(q) = query { Some(q.replace(" ", "+")) } else { None };
        let common_params = CommonParams {
            format: Some(Format::default()),
            attributes,
            query,
            filter: filters,
            per_page,
            page,
            sort,
        };

        let endpoint = Endpoints::Collections(common_params);
        let url = endpoint.to_url()?;

        // Replace the default base URL with the client's base_url
        let final_url = self.replace_base_url(&url)?;

        let response = self.client.get(&final_url).send()?.error_for_status()?;
        let json = response.json::<CollectionsResponse>()?;
        Ok((json, final_url))
    }

    /// Helper method to replace the default base URL in the endpoint URL with the client's base_url.
    ///
    /// This is necessary because the `Endpoints::to_url()` method includes a hardcoded base URL.
    ///
    /// # Parameters
    ///
    /// - `url`: The URL generated by the `to_url` method.
    ///
    /// # Returns
    ///
    /// Returns the modified URL with the client's base URL.
    fn replace_base_url(&self, url: &str) -> Result<String, Box<dyn Error>> {
        let default_base = "https://www.loc.gov";
        if url.starts_with(default_base) {
            let suffix = &url[default_base.len()..];
            Ok(format!("{}{}", self.base_url, suffix))
        } else {
            Err(format!("URL does not start with the expected base URL: {}", default_base).into())
        }
    }
}
