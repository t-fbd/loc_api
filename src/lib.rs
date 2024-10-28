//! # Rust API for the [Library of Congress](https://www.loc.gov/apis/)

/// # Endpoints Module
///
/// This module defines the various endpoints available in the Library of Congress API.
/// It includes enums and methods for constructing URLs based on different API endpoints
/// and their respective parameters.
pub mod endpoints {
    use serde::{Serialize, Deserialize};
    use super::{param_models::*, format_models::*};

    /// Represents the various endpoints available in the Library of Congress API.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub enum Endpoints {
        /// The `/search/` endpoint for general searches across the LOC website.
        Search(SearchParams),
        /// The `/collections/` endpoint to retrieve all digital collections.
        Collections(CommonParams),
        /// The `/collections/{name_of_collection}/` endpoint for a specific collection.
        Collection {
            name: String,
            params: CommonParams,
        },
        /// The `/{format}/` endpoint to retrieve items of a specific format.
        ///
        /// The format can be one of the following:
        /// - Audio
        /// - Books
        /// - Film and Videos
        /// - Legislation
        /// - Manuscripts
        /// - Maps
        /// - Newspapers
        /// - Photos
        /// - Notated Music
        /// - Web Archives
        ///
        /// # Fields
        /// - `format`: The specific format type.
        /// - `params`: Parameters specific to the format endpoint, such as formatting and
        /// attribute selection.
        Format {
            format: MediaType,
            params: CommonParams,
        },
        /// The `/item/{item_id}/` endpoint to retrieve detailed information about a specific item.
        ///
        /// # Fields
        ///
        /// - `item_id`: The unique identifier of the item. This should be the part of the URL that identifies the item.
        /// - `params`: Parameters specific to the item endpoint, such as formatting and attribute selection.
        Item {
            item_id: String,
            params: ItemParams,
        },
        /// The `/resource/{resource_id}/` endpoint to retrieve information about a specific resource.
        Resource {
            resource_id: String,
            params: ResourceParams,
        },
    }

    fn to_url_helper(common: &CommonParams) -> String {
        let format = common.format.unwrap_or(Format::Json).slug();
        let attributes = match common.attributes {
            Some(ref attrs) => attrs.to_query_param(),
            None => "".to_string(),
        };
        let query = match common.query {
            Some(ref q) => format!("&q={}", q),
            None => "".to_string(),
        };
        let filter = match common.filter {
            Some(ref f) => format!("&fa={}", f.filters.join("|")),
            None => "".to_string(),
        };
        let per_page = match common.per_page {
            Some(c) => format!("&c={}", c),
            None => "".to_string(),
        };
        let page = match common.page {
            Some(p) => format!("&sp={}", p),
            None => "1".to_string(),
        };
        let sort = match common.sort {
            Some(s) => format!("&sb={}", s.slug()),
            None => "".to_string(),
        };

        format!(
            "?fo={}&{}{}{}{}{}{}",
            format, attributes, query, filter, per_page, page, sort
        )

    }

    impl Endpoints {
        /// Constructs the full URL for the API request based on the endpoint and its parameters.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use loc_api::{endpoints::*, param_models::*, format_models::*,
        /// attribute_models::*};
        ///
        /// let common_params = CommonParams {
        ///     format: Some(Format::Json),
        ///     attributes: Some(AttributesSelect {
        ///         include: vec!["pagination".to_string(), "results".to_string()],
        ///         exclude: vec![],
        ///     }),
        ///     query: Some("dog".to_string()),
        ///     filter: Some(FacetReq {
        ///         filters: vec!["subject:animals".to_string()],
        ///     }),
        ///     per_page: Some(25),
        ///     page: Some(1),
        ///     sort: Some(SortField::TitleS),
        /// };
        ///
        /// let format_params = CommonParams {
        ///     format: Some(Format::Json),
        ///     ..common_params.clone()
        /// };
        ///
        /// let format_endpoint = Endpoints::Format {
        ///     format: MediaType::FilmAndVideos,
        ///     params: format_params,
        /// };
        ///
        /// let url = format_endpoint.to_url().unwrap();
        /// assert_eq!(url, "https://www.loc.gov/film-and-videos/?fo=json&at=pagination,results&q=dog&fa=subject:animals&c=25&sp=1&sb=title_s");
        /// ```
        pub fn to_url(&self) -> Result<String, serde_urlencoded::ser::Error> {
            let base_url = "https://www.loc.gov";

            match self {
                Endpoints::Search(params) => {
                    let mut url = format!("{}/search/", base_url);

                    let query_string = to_url_helper(&params.common);

                    if !query_string.is_empty() {
                        url.push_str(&query_string);
                    } else {
                        return Err(serde_urlencoded::ser::Error::Custom("No query parameters provided".to_string().into()));
                    }

                    Ok(url)
                },
                Endpoints::Collections(params) => {
                    // collection param must be in "kebab-case"
                    let mut url = format!("{}/collections/", base_url);

                    let query_string = to_url_helper(&params);

                    if !query_string.is_empty() {
                        url.push_str(&query_string.replace(" ", "-"));
                    } else {
                        return Err(serde_urlencoded::ser::Error::Custom("No query parameters provided".to_string().into()));
                    }

                    Ok(url)
                },
                Endpoints::Collection { name, params } => {
                    let mut url = format!("{}/collections/{}/", base_url, name);

                    let query_string = to_url_helper(&params);

                    if !query_string.is_empty() {
                        url.push_str(&query_string.replace(" ", "-"));
                    } else {
                        return Err(serde_urlencoded::ser::Error::Custom("No query parameters provided".to_string().into()));
                    }

                    Ok(url)
                },
                Endpoints::Format { format, params } => {
                    let format_slug = format.slug();
                    let mut url = format!("{}/{}/", base_url, format_slug);

                    let query_string = to_url_helper(&params);

                    if !query_string.is_empty() {
                        url.push_str(&query_string);
                    } else {
                        return Err(serde_urlencoded::ser::Error::Custom("No query parameters provided".to_string().into()));
                    }

                    Ok(url)
                },
                Endpoints::Item { item_id, params } => {
                    let mut url = format!("{}/item/{}/", base_url, item_id);

                    let format = params.format.unwrap_or(Format::Json).slug();

                    let attributes = match params.attributes {
                        Some(ref attrs) => {
                            let mut parts = Vec::new();

                            if let Some(item_attrs) = attrs.item {
                                if item_attrs {
                                    parts.push("at=item".to_string());
                                }
                            }

                            if let Some(resource_attrs) = attrs.resources {
                                if resource_attrs {
                                    parts.push("at=resources".to_string());
                                }
                            }

                            if let Some(cite_this) = attrs.cite_this {
                                if cite_this {
                                    parts.push("at=cite_this".to_string());
                                }
                            }

                            parts.join("&")
                        }
                        None => "".to_string(),
                    };

                    let query_string = format!("?fo={}&{}", format, attributes);

                    if !query_string.is_empty() {
                        url.push_str(&query_string);
                    }

                    Ok(url)
                },
                Endpoints::Resource { resource_id, params } => {
                    let mut url = format!("{}/resource/{}/", base_url, resource_id);

                    let format = params.format.unwrap_or(Format::Json).slug();
                    
                    let attributes = match params.attributes {
                        Some(ref attrs) => {
                            let mut parts = Vec::new();

                            if let Some(resource_attrs) = attrs.resource {
                                if resource_attrs {
                                    parts.push("at=resource".to_string());
                                }
                            }

                            if let Some(page_attrs) = attrs.page {
                                if page_attrs {
                                    parts.push("at=page".to_string());
                                }
                            }

                            if let Some(segment_attrs) = attrs.segments {
                                if segment_attrs {
                                    parts.push("at=segments".to_string());
                                }
                            }

                            if let Some(cite_this) = attrs.cite_this {
                                if cite_this {
                                    parts.push("at=cite_this".to_string());
                                }
                            }

                            if let Some(resources) = attrs.resources {
                                if resources {
                                    parts.push("at=resources".to_string());
                                }
                            }

                            parts.join("&")
                        }
                        None => "".to_string(),
                    };

                    let query_string = format!("?fo={}&{}", format, attributes);

                    if !query_string.is_empty() {
                        url.push_str(&query_string);
                    }

                    Ok(url)
                },
            }
        }
    }
}

pub mod param_models {
    use super::{format_models::*, attribute_models::*};
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
}

pub mod attribute_models {
    use serde::{Serialize, Deserialize};

    /// Represents the possible attributes (query parameters) that can be used in API requests.
    #[derive(Debug, Serialize, Deserialize, Clone, Copy)]
    pub enum Attributes {
        /// Common attributes applicable to multiple endpoints.
        Common(CommonAttributes),
        /// Attributes specific to the Item endpoint.
        Item(ItemAttributes),
        /// Attributes specific to the Resource endpoint.
        Resource(ResourceAttributes),
    }

    /// Common attributes used across multiple endpoints.
    #[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
    pub struct CommonAttributes {
        /// Include facet information in the response (`at=facets`).
        pub facets: Option<bool>,
        /// Include pagination information in the response (`at=pagination`).
        pub pagination: Option<bool>,
        /// Include result summaries in the response (`at=results`).
        pub results: Option<bool>,
    }

    /// Represents the detailed bibliographic information of an item.
    ///
    /// # Fields
    ///
    /// - `place_of_publication`: The place where the item was published.
    /// - `source_collection`: A list of collections that source the item.
    /// - `display_offsite`: Indicates if the item is displayed offsite.
    /// - `contributors`: A list of contributors to the item.
    /// - `location_county`: Counties associated with the item's location.
    /// - `access_restricted`: Indicates if access to the item is restricted.
    /// - `site`: Sites associated with the item.
    /// - `original_format`: Original formats of the item.
    /// - `partof_title`: Titles indicating that the item is part of a larger collection.
    /// - `date`: Publication date of the item.
    /// - `item_type`: The type/category of the item.
    /// - `url`: URL of the item on the LOC website.
    /// - `subject_headings`: Subject headings associated with the item.
    /// - ... (continue for other fields)
    ///
    /// # Notes
    ///
    /// Fields like `additional` capture any extra data not explicitly defined in the struct.
    /// This ensures forward compatibility with potential future changes in the API response.
    #[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
    pub struct ItemAttributes {
        /// Include citation information in the response (`at=cite_this`).
        pub cite_this: Option<bool>,
        /// Include the item details in the response (`at=item`).
        pub item: Option<bool>,
        /// Include resource links in the response (`at=resources`).
        pub resources: Option<bool>,
    }

    /// Attributes for the Resource endpoint.
    #[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
    pub struct ResourceAttributes {
        /// Include citation information in the response (`at=cite_this`).
        pub cite_this: Option<bool>,
        /// Include the item details in the response (`at=item`).
        pub item: Option<bool>,
        /// Include page information in the response (`at=page`).
        pub page: Option<bool>,
        /// Include individual resource details in the response (`at=resource`).
        pub resource: Option<bool>,
        /// Include multiple resource links in the response (`at=resources`).
        pub resources: Option<bool>,
        /// Include segment information in the response (`at=segments`).
        pub segments: Option<bool>,
    }

    /// Represents the possible sort fields for the `sort` attribute.
    #[derive(Debug, Serialize, Deserialize, Clone, Copy)]
    pub enum SortField {
        #[serde(rename = "date")]
        Date, // Sort by date (earliest to latest) - sb=date
        #[serde(rename = "date_desc")]
        DateDesc, // Sort by date descending (latest to earliest) - sb=date_desc
        #[serde(rename = "title_s")]
        TitleS, // Sort by title - sb=title_s
        #[serde(rename = "title_s_desc")]
        TitleSDesc, // Sort by title descending - sb=title_s_desc
        #[serde(rename = "shelf_id")]
        ShelfId, // Sort by shelf ID (call number/physical location) - sb=shelf_id
        #[serde(rename = "shelf_id_desc")]
        ShelfIdDesc, // Sort by shelf ID descending - sb=shelf_id_desc
    }

    impl SortField {
        /// Returns the corresponding slug used in the API URL for each sort field.
        pub fn slug(&self) -> &'static str {
            match self {
                SortField::Date => "date",
                SortField::DateDesc => "date_desc",
                SortField::TitleS => "title_s",
                SortField::TitleSDesc => "title_s_desc",
                SortField::ShelfId => "shelf_id",
                SortField::ShelfIdDesc => "shelf_id_desc",
            }
        }
    }

    /// Represents the selection of attributes to include or exclude in the response.
    #[derive(Debug, Serialize, Deserialize, Default, Clone)]
    pub struct AttributesSelect {
        /// Attributes to include in the response.
        ///
        /// Example: `["item", "resources"]`
        pub include: Vec<String>,
        /// Attributes to exclude from the response.
        ///
        /// Example: `["more_like_this"]`
        pub exclude: Vec<String>,
    }

    impl AttributesSelect {
        /// Converts the `AttributesSelect` struct into a query parameter string.
        ///
        /// The function concatenates included attributes with commas and appends exclamation marks for exclusions.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use loc_api::attribute_models::AttributesSelect;
        ///
        /// let attrs = AttributesSelect {
        ///     include: vec!["item".to_string(), "resources".to_string()],
        ///     exclude: vec!["more_like_this".to_string()],
        /// };
        /// assert_eq!(attrs.to_query_param(), "at=item,resources&at!=more_like_this");
        /// ```
        pub fn to_query_param(&self) -> String {
            let mut parts = Vec::new();

            if !self.include.is_empty() {
                parts.push(format!("at={}", self.include.join(",")));
            }

            if !self.exclude.is_empty() {
                parts.push(format!("at!={}", self.exclude.join(",")));
            }

            parts.join("&")
        }
    }
}

pub mod format_models {
    use serde::{Serialize, Deserialize};

    /// Represents the possible response formats for API requests.
    #[derive(Debug, Serialize, Deserialize, Clone, Copy)]
    pub enum Format {
        /// JSON format (`fo=json`).
        #[serde(rename = "json")]
        Json,
        /// YAML format (`fo=yaml`).
        #[serde(rename = "yaml")]
        Yaml,
    }

    impl Default for Format {
        fn default() -> Self {
            Format::Json
        }
    }

    impl Format {
        /// Returns the corresponding slug used in the API URL for each format type.
        pub fn slug(&self) -> &'static str {
            match self {
                Format::Json => "json",
                Format::Yaml => "yaml",
            }
        }
    }

    /// Enum to represent specific format types for the `/{format}/` endpoint.
    #[derive(Debug, Serialize, Deserialize, Clone, Copy)]
    pub enum MediaType {
        /// Audio recordings (`/audio/`).
        Audio,
        /// Books or printed material (`/books/`).
        Books,
        /// Films and videos (`/film-and-videos/`).
        FilmAndVideos,
        /// Legislation (`/legislation/`).
        Legislation,
        /// Manuscripts or mixed material (`/manuscripts/`).
        Manuscripts,
        /// Maps (`/maps/`).
        Maps,
        /// Newspapers (`/newspapers/`).
        Newspapers,
        /// Photos, prints, or drawings (`/photos/`).
        Photos,
        /// Notated music such as sheet music (`/notated-music/`).
        NotatedMusic,
        /// Web archives (`/web-archives/`).
        WebArchives,
    }

    impl MediaType {
        /// Returns the corresponding slug used in the API URL for each format type.
        pub fn slug(&self) -> &'static str {
            match self {
                MediaType::Audio => "audio",
                MediaType::Books => "books",
                MediaType::FilmAndVideos => "film-and-videos",
                MediaType::Legislation => "legislation",
                MediaType::Manuscripts => "manuscripts",
                MediaType::Maps => "maps",
                MediaType::Newspapers => "newspapers",
                MediaType::Photos => "photos",
                MediaType::NotatedMusic => "notated-music",
                MediaType::WebArchives => "web-archives",
            }
        }
    }
}

pub mod response_models {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    /// Represents a value that can be either a single String or a Vec<String>.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(untagged)]
    pub enum StringOrArray {
        String(String),
        Array(Vec<String>),
    }

    /// Represents a value that can be either a u32 or a String.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(untagged)]
    pub enum NumberOrString {
        Number(u32),
        String(String),
    }

    /// Represents a value that can be either a bool or a String.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(untagged)]
    pub enum BoolOrString {
        Bool(bool),
        String(String),
    }

    /// Represents a value that can be either a single item or an array of items.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(untagged)]
    pub enum ItemOrArray<T> {
        Item(T),
        Array(Vec<T>),
    }

    /// Represents a single facet category.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct FacetRes {
        /// The name of the facet field (e.g., "subject", "location").
        /// A list of filters within the facet.
        pub filters: Option<ItemOrArray<FilterItem>>,
    }

    /// Represents a single filter within a facet.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct FilterItem {
        /// The number of results matching this filter.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub count: Option<NumberOrString>,
        /// URL to exclude this filter.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub not: Option<StringOrArray>,
        /// URL to toggle off this filter.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub off: Option<StringOrArray>,
        /// URL to toggle on this filter.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub on: Option<StringOrArray>,
        /// The term associated with the filter.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub term: Option<StringOrArray>,
        /// The display title of the filter.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title: Option<StringOrArray>,
    }

    /// Represents the pagination information in the response.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Pagination {
        /// Index number of the first result item on the current page.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub from: Option<NumberOrString>,
        /// Range of result items on the current page (e.g., "1 - 25").
        #[serde(skip_serializing_if = "Option::is_none")]
        pub results: Option<StringOrArray>,
        /// URL of the last page of results.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub last: Option<StringOrArray>,
        /// Total number of result items across all pages.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total: Option<NumberOrString>,
        /// URL of the previous page of results, if any.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub previous: Option<StringOrArray>,
        /// Number of result items per page.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub perpage: Option<NumberOrString>,
        /// Available options for `perpage`.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub perpage_options: Option<ItemOrArray<u32>>,
        /// Total number of pages available.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub of: Option<NumberOrString>,
        /// URL of the next page of results, if any.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next: Option<StringOrArray>,
        /// Current page number.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub current: Option<NumberOrString>,
        /// Index number of the last result item on the current page.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub to: Option<NumberOrString>,
        /// List of pages available for navigation.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub page_list: Option<ItemOrArray<PageListItem>>,
        /// URL of the first page of results.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub first: Option<StringOrArray>,
    }

    /// Represents a single page in the pagination list.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct PageListItem {
        /// URL of the page, if available.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<StringOrArray>,
        /// Page number or placeholder (e.g., "...").
        #[serde(skip_serializing_if = "Option::is_none")]
        pub number: Option<NumberOrString>,
    }

    /// Represents a single item in the search results.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct ResultItem {
        /// Indicates if access to the item is restricted.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub access_restricted: Option<BoolOrString>,
        /// Alternative identifiers for the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub aka: Option<ItemOrArray<String>>,
        /// Campaigns associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub campaigns: Option<ItemOrArray<String>>,
        /// Contributors to the creation of the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub contributor: Option<ItemOrArray<String>>,
        /// Publication date of the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub date: Option<StringOrArray>,
        /// List of dates related to the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dates: Option<ItemOrArray<String>>,
        /// Description of the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<StringOrArray>,
        /// Indicates if the item has been digitized.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub digitized: Option<BoolOrString>,
        /// Timestamp of the latest ETL process.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub extract_timestamp: Option<StringOrArray>,
        /// Groups associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub group: Option<ItemOrArray<String>>,
        /// Indicates if the item has segmented data.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub hassegments: Option<BoolOrString>,
        /// URL identifier for the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<StringOrArray>,
        /// URLs to images associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub image_url: Option<ItemOrArray<String>>,
        /// Index number of the item in the results.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub index: Option<NumberOrString>,
        /// Summary information of the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub item: Option<ItemOrArray<ItemSummary>>,
        /// Languages associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub language: Option<ItemOrArray<String>>,
        /// Locations related to the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub location: Option<ItemOrArray<String>>,
        /// MIME types available for the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mime_type: Option<ItemOrArray<String>>,
        /// Numeric identifiers associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub number_field: Option<ItemOrArray<String>>,
        /// Online formats available for the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub online_format: Option<ItemOrArray<String>>,
        /// Original formats of the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub original_format: Option<ItemOrArray<String>>,
        /// Alternative titles for the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub other_title: Option<ItemOrArray<String>>,
        /// Collections or divisions the item is part of.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub partof: Option<ItemOrArray<String>>,
        /// Publication frequency of the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub publication_frequency: Option<ItemOrArray<String>>,
        /// Shelf identifier for sorting.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub shelf_id: Option<StringOrArray>,
        /// Sites associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub site: Option<ItemOrArray<String>>,
        /// Subjects associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub subject: Option<ItemOrArray<String>>,
        /// Type of the item (e.g., "web page").
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_field: Option<StringOrArray>, // Updated to handle multiple types
        /// Captures any additional fields not explicitly defined.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(flatten)]
        pub additional: Option<Value>,
    }

    /// Represents the summary information of an item in the search results.
    #[derive(Debug, Serialize, Deserialize, Clone, Default)]
    pub struct ItemSummary {
        /// Call numbers associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub call_number: Option<ItemOrArray<String>>,
        /// Names of contributors to the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub contributor_names: Option<ItemOrArray<String>>,
        /// Information about where the item was created or published.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created_published: Option<ItemOrArray<String>>,
        /// Date when the item was issued.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub date_issued: Option<StringOrArray>,
        /// Label indicating if the item has been digitized.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub digitized_label: Option<StringOrArray>,
        /// Genres associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub genre: Option<ItemOrArray<String>>,
        /// Languages associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub language: Option<ItemOrArray<String>>,
        /// Locations related to the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub location: Option<ItemOrArray<String>>,
        /// Medium of the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub medium: Option<StringOrArray>,
        /// Alternative titles for the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub other_title: Option<ItemOrArray<String>>,
        /// Publication frequency details.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub publication_frequency: Option<ItemOrArray<String>>,
        /// Relevance score of the item in search results.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub score: Option<NumberOrString>,
        /// Subject headings associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub subject_headings: Option<ItemOrArray<String>>,
        /// Subjects associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub subjects: Option<ItemOrArray<String>>,
        /// Summary descriptions of the item.
        /// Can be a string or a list of strings.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub summary: Option<StringOrArray>,
        /// Title of the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title: Option<StringOrArray>,
        /// Captures any additional fields not explicitly defined.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(flatten)]
        pub additional: Option<Value>,
    }

    /// Represents the response from the `/item/{item_id}/` endpoint.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct ItemResponse {
        /// Various views available for the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub views: Option<ItemOrArray<Value>>,
        /// Timestamp indicating when the item was indexed.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub timestamp: Option<NumberOrString>,
        /// Locations associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub locations: Option<ItemOrArray<String>>,
        /// URL to the full-text service.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fulltext_service: Option<StringOrArray>,
        /// Information about the next issue.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_issue: Option<StringOrArray>,
        /// URL to newspaper holdings.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub newspaper_holdings_url: Option<StringOrArray>,
        /// URL to the title page.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title_url: Option<StringOrArray>,
        /// Pages associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub page: Option<ItemOrArray<Page>>,
        /// Pagination details.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pagination: Option<ItemOrArray<Pagination>>,
        /// Resource details associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource: Option<ItemOrArray<Value>>,
        /// Citation information in various formats.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cite_this: Option<ItemOrArray<CiteThis>>,
        /// Calendar URL related to the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub calendar_url: Option<StringOrArray>,
        /// Information about the previous issue.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub previous_issue: Option<StringOrArray>,
        /// Segments within the resource.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub segments: Option<ItemOrArray<Segment>>,
        /// Related items to this item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub related_items: Option<ItemOrArray<RelatedItem>>,
        /// Query details for word coordinates.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub word_coordinates_query: Option<ItemOrArray<Value>>,
        /// "More like this" recommendations.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub more_like_this: Option<ItemOrArray<MoreLikeThis>>,
        /// Articles and essays related to the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub articles_and_essays: Option<ItemOrArray<String>>,
        /// Traditional knowledge labels associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub traditional_knowledge_labels: Option<ItemOrArray<String>>,
        /// Detailed bibliographic information.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub item: Option<ItemOrArray<ItemAttribute>>,
        /// Word coordinates on pages.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub word_coordinates_pages: Option<ItemOrArray<Value>>,
        /// Type of the response (e.g., "Item").
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_field: Option<StringOrArray>, // Updated to handle multiple types
        /// Additional options or metadata.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub options: Option<ItemOrArray<Value>>,
        /// Resources associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resources: Option<ItemOrArray<ResourceObject>>,
        /// Captures any additional fields not explicitly defined.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(flatten)]
        pub additional: Option<Value>,
    }

    /// Represents the response from the `/resource/{resource_id}/` endpoint.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct ResourceResponse {
        /// Various views available for the resource.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub views: Option<ItemOrArray<Value>>,
        /// Timestamp indicating when the resource was indexed.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub timestamp: Option<NumberOrString>,
        /// Locations associated with the resource.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub locations: Option<ItemOrArray<String>>,
        /// URL to the full-text service.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fulltext_service: Option<StringOrArray>,
        /// Information about the next issue.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_issue: Option<StringOrArray>,
        /// URL to newspaper holdings.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub newspaper_holdings_url: Option<StringOrArray>,
        /// URL to the title page.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title_url: Option<StringOrArray>,
        /// Pages associated with the resource.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub page: Option<ItemOrArray<Page>>,
        /// Pagination details.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pagination: Option<ItemOrArray<Pagination>>,
        /// Detailed information about the resource.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource: Option<ItemOrArray<ResourceDetail>>,
        /// Citation information in various formats.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cite_this: Option<ItemOrArray<CiteThis>>,
        /// Calendar URL related to the resource.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub calendar_url: Option<StringOrArray>,
        /// Information about the previous issue.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub previous_issue: Option<StringOrArray>,
        /// Segments within the resource.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub segments: Option<ItemOrArray<Segment>>,
        /// Related items to this resource.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub related_items: Option<ItemOrArray<RelatedItem>>,
        /// Query details for word coordinates.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub word_coordinates_query: Option<ItemOrArray<Value>>,
        /// "More like this" recommendations.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub more_like_this: Option<ItemOrArray<MoreLikeThis>>,
        /// Articles and essays related to the resource.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub articles_and_essays: Option<ItemOrArray<String>>,
        /// Traditional knowledge labels associated with the resource.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub traditional_knowledge_labels: Option<ItemOrArray<String>>,
        /// Detailed bibliographic information.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub item: Option<ItemOrArray<ItemAttribute>>,
        /// Word coordinates on pages.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub word_coordinates_pages: Option<ItemOrArray<Value>>,
        /// Type of the response (e.g., "Resource").
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_field: Option<StringOrArray>, // Updated to handle multiple types
        /// Additional options or metadata.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub options: Option<ItemOrArray<Value>>,
        /// Resources associated with the resource.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resources: Option<ItemOrArray<ResourceObject>>,
        /// Captures any additional fields not explicitly defined.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(flatten)]
        pub additional: Option<Value>,
    }

    /// Represents the detailed information about a resource.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct ResourceDetail {
        /// Caption for the resource.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub caption: Option<ItemOrArray<String>>,
        /// List of files associated with the resource.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub files: Option<ItemOrArray<Vec<File>>>,
        /// URL to the audio file, if applicable.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub audio: Option<StringOrArray>,
        /// URL to the background resource, if applicable.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub background: Option<StringOrArray>,
        /// Start time for media files.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub begin: Option<StringOrArray>,
        /// Date range captured in the resource.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub capture_range: Option<ItemOrArray<String>>,
        /// URL to the DJVu text file, if applicable.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub djvu_text_file: Option<StringOrArray>,
        /// Indicates if downloading the resource is restricted.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub download_restricted: Option<BoolOrString>,
        /// Duration of the media file in seconds.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub duration: Option<NumberOrString>,
        /// End time for media files.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end: Option<StringOrArray>,
        /// URL to the full-text derivative.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fulltext_derivative: Option<StringOrArray>,
        /// URL to the full-text file.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fulltext_file: Option<StringOrArray>,
        /// Height of the resource in pixels.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub height: Option<NumberOrString>,
        /// Unique identifier of the resource.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<StringOrArray>,
        /// URL to additional information about the resource.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub info: Option<StringOrArray>,
        /// URL to the image file, if applicable.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub image: Option<StringOrArray>,
        /// Path to the resource in the Paprika system.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub paprika_resource_path: Option<StringOrArray>,
        /// URL to the PDF version of the resource, if applicable.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pdf: Option<StringOrArray>,
        /// Representative index number.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub representative_index: Option<NumberOrString>,
        /// Type of the resource (e.g., "audio").
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_field: Option<StringOrArray>,
        /// URL to access the resource.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<StringOrArray>,
        /// Universally Unique Identifier for the resource.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub uuid: Option<StringOrArray>,
        /// Version number of the resource.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub version: Option<NumberOrString>,
        /// URL to the video stream, if applicable.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub video_stream: Option<StringOrArray>,
        /// URL to the video file, if applicable.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub video: Option<StringOrArray>,
        /// Width of the resource in pixels.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub width: Option<NumberOrString>,
        /// URL to word coordinates data, if applicable.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub word_coordinates: Option<StringOrArray>,
        /// Captures any additional fields not explicitly defined.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(flatten)]
        pub additional: Option<Value>,
    }

    /// Represents a single file associated with a resource.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct File {
        /// Captions associated with the file.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub caption: Option<ItemOrArray<String>>,
        /// Duration of the file in seconds, if applicable.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub duration: Option<NumberOrString>,
        /// Format information of the file.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub format: Option<ItemOrArray<Value>>, // Could represent IIIF, audio, or video specifics
        /// Height of the media file in pixels, if applicable.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub height: Option<NumberOrString>,
        /// URL to additional information about the file.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub info: Option<StringOrArray>,
        /// Zoom levels available for the file, typically used for images or maps.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub levels: Option<NumberOrString>,
        /// MIME type of the file.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mimetype: Option<StringOrArray>,
        /// Alternative name for the file, if applicable.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub other_name: Option<StringOrArray>,
        /// Profile information, potentially used for IIIF configurations.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub profile: Option<ItemOrArray<String>>,
        /// Protocol used to access the file.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub protocol: Option<StringOrArray>,
        /// Size of the file in bytes.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub size: Option<NumberOrString>,
        /// Streams associated with the file, if applicable.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub streams: Option<ItemOrArray<String>>,
        /// Tiles associated with the file, used in tiled media presentations.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tiles: Option<ItemOrArray<String>>,
        /// Type of the file (e.g., "audio").
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_field: Option<StringOrArray>,
        /// URL to access the file.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<StringOrArray>,
        /// Usage description of the file (e.g., "newspaper").
        #[serde(skip_serializing_if = "Option::is_none")]
        pub use_field: Option<StringOrArray>, // `use` is a reserved keyword in Rust
        /// Width of the media file in pixels, if applicable.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub width: Option<NumberOrString>,
        /// Captures any additional fields not explicitly defined.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(flatten)]
        pub additional: Option<Value>,
    }

    /// Represents citation information in various formats.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct CiteThis {
        /// Citation formatted in the Chicago style.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub chicago: Option<StringOrArray>,
        /// Citation formatted in the MLA style.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mla: Option<StringOrArray>,
        /// Citation formatted in the APA style.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub apa: Option<StringOrArray>,
    }

    /// Represents a segment within a resource.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Segment {
        /// Captures any additional fields not explicitly defined.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(flatten)]
        pub additional: Option<Value>,
    }

    /// Represents related items to the current item/resource.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct RelatedItem {
        /// Captures any additional fields not explicitly defined.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(flatten)]
        pub additional: Option<Value>,
    }

    /// Represents "more like this" recommendations.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct MoreLikeThis {
        /// Captures any additional fields not explicitly defined.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(flatten)]
        pub additional: Option<Value>,
    }

    /// Represents a single page in the response.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Page {
        /// Captures any additional fields not explicitly defined.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(flatten)]
        pub additional: Option<Value>,
    }

    /// Represents the item attribute object within `ItemResponse` and `ResourceResponse`.
    #[derive(Debug, Serialize, Deserialize, Clone, Default)]
    pub struct ItemAttribute {
        /// Place of publication.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub place_of_publication: Option<StringOrArray>,
        /// Source collections.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub source_collection: Option<ItemOrArray<String>>,
        /// Indicates if the item is displayed offsite.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub display_offsite: Option<BoolOrString>,
        /// Contributors to the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub contributors: Option<ItemOrArray<String>>,
        /// County locations associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub location_county: Option<ItemOrArray<String>>,
        /// Indicates if access to the item is restricted.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub access_restricted: Option<BoolOrString>,
        /// Sites associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub site: Option<ItemOrArray<String>>,
        /// Original formats of the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub original_format: Option<ItemOrArray<String>>,
        /// Titles indicating part of a larger collection.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub partof_title: Option<ItemOrArray<String>>,
        /// Publication date.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub date: Option<StringOrArray>,
        /// Type of the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub item_type: Option<StringOrArray>,
        /// URL of the item on the LOC website.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<StringOrArray>,
        /// Subject headings associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub subject_headings: Option<ItemOrArray<String>>,
        /// Newspaper titles associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub newspaper_title: Option<ItemOrArray<String>>,
        /// Information about creation/publication.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created_published: Option<ItemOrArray<String>>,
        /// Extract URLs related to the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub extract_urls: Option<ItemOrArray<String>>,
        /// Divisions the item is part of.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub partof_division: Option<ItemOrArray<String>>,
        /// Contents of the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub contents: Option<ItemOrArray<String>>,
        /// Subjects associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub subject: Option<ItemOrArray<String>>,
        /// Index number of the item in search results.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub index: Option<NumberOrString>,
        /// Digital identifiers.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub digital_id: Option<ItemOrArray<String>>,
        /// Call numbers associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub call_number: Option<ItemOrArray<String>>,
        /// Groups associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub group: Option<ItemOrArray<String>>,
        /// Relevance score in search results.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub score: Option<NumberOrString>,
        /// Countries associated with the item's location.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub location_country: Option<ItemOrArray<String>>,
        /// Title of the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title: Option<StringOrArray>,
        /// Descriptions of the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<StringOrArray>,
        /// Related items.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub related_items: Option<ItemOrArray<String>>,
        /// URL identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<StringOrArray>,
        /// Online formats available for the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub online_format: Option<ItemOrArray<String>>,
        /// Subjects associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub subjects: Option<ItemOrArray<String>>,
        /// Languages associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub language: Option<ItemOrArray<String>>,
        /// Rights information.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rights: Option<ItemOrArray<String>>,
        /// Locations associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub locations: Option<ItemOrArray<String>>,
        /// Notes associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub notes: Option<ItemOrArray<String>>,
        /// Shelf identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub shelf_id: Option<StringOrArray>,
        /// Batch information.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub batch: Option<ItemOrArray<String>>,
        /// Summary descriptions.
        /// Can be a string or a list of strings.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub summary: Option<StringOrArray>,
        /// Indicates if the item has been digitized.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub digitized: Option<BoolOrString>,
        /// Publication frequency details.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub publication_frequency: Option<ItemOrArray<String>>,
        /// Resources associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resources: Option<ItemOrArray<String>>,
        /// Alternative identifiers.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub aka: Option<ItemOrArray<String>>,
        /// Names of contributors.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub contributor_names: Option<ItemOrArray<String>>,
        /// URLs to images associated with the item.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub image_url: Option<ItemOrArray<String>>,
        /// Advisory information regarding access.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub access_advisory: Option<ItemOrArray<String>>,
        /// Captures any additional fields not explicitly defined.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(flatten)]
        pub additional: Option<Value>,
    }

    /// Represents a single resource object within `ItemResponse` and `ResourceResponse`.
    #[derive(Debug, Serialize, Deserialize, Clone, Default)]
    pub struct ResourceObject {
        /// Files associated with the resource.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub files: Option<ItemOrArray<ItemOrArray<File>>>,
        /// Caption for the resource.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub caption: Option<ItemOrArray<String>>,
        /// URL to the file, if applicable.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<ItemOrArray<String>>,
        /// image URL for the resource, if applicable.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub image: Option<ItemOrArray<String>>,
        /// Type of the resource.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_field: Option<StringOrArray>,
        /// Height of the resource in pixels.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub height: Option<ItemOrArray<NumberOrString>>,
        /// Width of the resource in pixels.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub width: Option<ItemOrArray<NumberOrString>>,
        /// Duration of the resource in seconds.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub duration: Option<ItemOrArray<NumberOrString>>,
        /// MIME type of the resource.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mimetype: Option<ItemOrArray<String>>,
        /// Size of the resource in bytes.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub size: Option<ItemOrArray<NumberOrString>>,
        /// Identifier of the resource.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<ItemOrArray<String>>,
        /// title of the resource.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title: Option<ItemOrArray<String>>,
        #[serde(flatten)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub additional: Option<Value>,
    }

    /// Represents the response from Search Result Endpoints like `/search/`, `/collections/`, or `/{format}/`.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct SearchResultResponse {
        /// Facet information for filtering results.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub facets: Option<FacetRes>,
        /// Pagination details for navigating through result pages.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pagination: Option<Pagination>,
        /// A list of result items matching the search criteria.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub results: Option<Vec<ResultItem>>,
        /// Captures any additional fields not explicitly defined.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(flatten)]
        pub additional: Option<Value>,
    }

    /// Represents the response from the `/collections/` endpoint.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct CollectionsResponse {
        /// Facet information for collections.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub facets: Option<FacetRes>,
        /// Pagination details.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pagination: Option<Pagination>,
        /// List of collections.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub results: Option<Vec<CollectionItem>>,
        /// Captures any additional fields not explicitly defined.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(flatten)]
        pub additional: Option<Value>,
    }

    /// Represents a single collection item in the `/collections/` response.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct CollectionItem {
        /// Unique identifier of the collection.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<StringOrArray>,
        /// Title of the collection.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title: Option<StringOrArray>,
        /// Description of the collection.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<StringOrArray>,
        /// Private notes regarding the collection.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub private_note: Option<StringOrArray>,
        /// Slug used in the collection's URL.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub collection_slug: Option<StringOrArray>,
        /// Organization responsible for the collection.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub organization: Option<StringOrArray>,
        /// URL to the collection's page.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<StringOrArray>,
        /// URL to the site's map, if any.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub site_map: Option<StringOrArray>,
        /// Type of the collection.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_field: Option<StringOrArray>,
        /// Normalized slug for the collection.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub normalized_slug: Option<StringOrArray>,
        /// Timestamp when the collection was created.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created_at: Option<StringOrArray>,
        /// Timestamp when the collection was last updated.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<StringOrArray>,
        /// Captures any additional fields not explicitly defined.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(flatten)]
        pub additional: Option<Value>,
    }

    /// Represents a single collection response (`/collections/{name_of_collection}/`).
    #[derive(Debug, Serialize, Deserialize, Clone, Default)]
    pub struct CollectionResponse {
        /// Facet information for the collection.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub facets: Option<FacetRes>,
        /// Pagination details.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pagination: Option<Pagination>,
        /// List of items within the collection.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub results: Option<Vec<CollectionItem>>,
        /// Captures any additional fields not explicitly defined.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(flatten)]
        pub additional: Option<Value>,
    }

    /// Represents a format-specific response.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct FormatResponse {
        /// Facet information for the format.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub facets: Option<FacetRes>,
        /// Pagination details.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pagination: Option<Pagination>,
        /// List of items matching the format.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub results: Option<Vec<ResultItem>>,
        /// Captures any additional fields not explicitly defined.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(flatten)]
        pub additional: Option<Value>,
    }

    /// Represents the detailed information about a single collection item.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct CollectionDetail {
        /// Captures any additional fields not explicitly defined.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(flatten)]
        pub additional: Option<Value>,
    }

    /// Represents a generic search response for various endpoints.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct SearchResponse {
        /// Facet information for filtering results.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub facets: Option<FacetRes>,
        /// Pagination details.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pagination: Option<Pagination>,
        /// Results matching the search criteria.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub results: Option<Vec<ResultItem>>,
        /// Additional fields not explicitly defined.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(flatten)]
        pub additional: Option<Value>,
    }
}

/// The `simple_builders` module provides a high-level interface for interacting with
/// the Library of Congress API. It abstracts the complexities of endpoint construction,
/// parameter management, and HTTP requests, offering straightforward methods for common operations.
///
/// All methods return a tuple containing the deserialized JSON response and the final URL used
pub mod simple_builders {
    use super::{response_models::*, param_models::*, attribute_models::*, format_models::*, endpoints::*};
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
        ///     Ok(response) => response,
        ///     Err(e) => {
        ///         eprintln!("Error: {}", e);   
        ///         loc_api::CollectionResponse::default()
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
}

