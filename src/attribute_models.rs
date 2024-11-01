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
/// - ... etc.
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
    /// Converts the [`AttributesSelect`] struct into a query parameter string.
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
