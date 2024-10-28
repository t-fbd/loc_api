use crate::{format_models::*, attribute_models::*};
use serde::{Deserialize, Serialize};

/// Represents common query parameters applicable to multiple endpoints.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct CommonParams {
    /// Specifies the format of the returned results (`fo=json` or `fo=yaml`).
    pub format: Option<Format>,
    /// Selects one or more attributes to include or exclude in the response.
    pub attributes: Option<AttributesSelect>,
    /// Conducts a keyword search in the metadata and any available full text (`q` parameter).
    pub query: Option<String>,
    /// Applies facet filters to narrow down search results (`fa` parameter).
    pub filter: Option<FacetReq>,
    /// Sets the number of results per page (`c` parameter). Default is 25.
    pub per_page: Option<u32>,
    /// Specifies the page number to retrieve (`sp` parameter). The first page is 1.
    pub page: Option<u32>,
    /// Defines the sorting order of the results (`sb` parameter).
    pub sort: Option<SortField>,
}

/// Parameters specific to the `/search/` endpoint.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SearchParams {
    /// Common query parameters.
    pub common: CommonParams,
    /// Determines whether to include collections in the search results.
    ///
    /// **Note**: This is a placeholder for potential future extensions.
    pub include_collections: bool,
}

/// Parameters specific to the `/item/{item_id}/` endpoint.
#[derive(Debug, Serialize, Clone, Default, Deserialize)]
pub struct ItemParams {
    /// Specifies the format of the returned results (`fo=json` or `fo=yaml`).
    pub format: Option<Format>,
    /// Selects specific attributes to include in the item response.
    pub attributes: Option<ItemAttributes>,
}

/// Parameters specific to the `/resource/{resource_id}/` endpoint.
#[derive(Debug, Serialize, Clone, Default, Deserialize)]
pub struct ResourceParams {
    /// Specifies the format of the returned results (`fo=json` or `fo=yaml`).
    pub format: Option<Format>,
    /// Selects specific attributes to include in the resource response.
    pub attributes: Option<ResourceAttributes>,
}

/// Represents the filter/facet parameter (`fa`).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FacetReq {
    /// A list of facet filters (e.g., `"location:ohio"`, `"subject:wildlife"`).
    pub filters: Vec<String>,
}

impl FacetReq {
    /// Converts the `FacetReq` struct into a query parameter string.
    ///
    /// The function joins the filters with a pipe character (`|`) for multiple filters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use loc_api::param_models::FacetReq;
    ///
    /// let filter = FacetReq {
    ///     filters: vec!["location:ohio".to_string(), "subject:wildlife".to_string()],
    /// };
    /// assert_eq!(filter.to_query_param(), "location:ohio|subject:wildlife");
    /// ```
    pub fn to_query_param(&self) -> String {
        self.filters.join("|")
    }
}

/// Represents all possible query parameters for different API requests.
///
/// **Note**: This enum can be expanded to include more variants as needed.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum QueryParam {
    /// Common query parameters.
    Common(CommonParams),
    /// Parameters specific to the search endpoint.
    Search(SearchParams),
    /// Parameters specific to the collections endpoint.
    Collections(CommonParams),
    /// Parameters specific to a single collection.
    Collection {
        /// The name of the collection in kebab-case.
        name: String,
        /// Common query parameters.
        params: CommonParams,
    },
    /// Parameters specific to a format endpoint.
    Format {
        /// The specific format type.
        format: MediaType,
        /// Common query parameters.
        params: CommonParams,
    },
    /// Parameters specific to an item endpoint.
    Item {
        /// The unique identifier of the item.
        item_id: String,
        /// Parameters for the item endpoint.
        params: ItemParams,
    },
    /// Parameters specific to a resource endpoint.
    Resource {
        /// The unique identifier of the resource.
        resource_id: String,
        /// Parameters for the resource endpoint.
        params: ResourceParams,
    },
}
