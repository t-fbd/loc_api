//! # Endpoints Module
//!
//! This module defines the various endpoints available in the Library of Congress API.
//! It includes enums and methods for constructing URLs based on different API endpoints
//! and their respective parameters.

use std::error::Error;

use serde::{Serialize, Deserialize};
use crate::{param_models::*, format_models::*};

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
    pub fn to_url(&self) -> Result<String, Box<dyn Error>> {
        let base_url = "https://www.loc.gov";

        match self {
            Endpoints::Search(params) => {
                let mut url = format!("{}/search/", base_url);

                let query_string = to_url_helper(&params.common);

                if !query_string.is_empty() {
                    url.push_str(&query_string);
                } else {
                    return Err("No query parameters provided".to_string().into());
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
                    return Err("No query parameters provided".to_string().into());
                }

                Ok(url)
            },
            Endpoints::Collection { name, params } => {
                let mut url = format!("{}/collections/{}/", base_url, name);

                let query_string = to_url_helper(&params);

                if !query_string.is_empty() {
                    url.push_str(&query_string.replace(" ", "-"));
                } else {
                    return Err("No query parameters provided".to_string().into());
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
                    return Err("No query parameters provided".to_string().into());
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
