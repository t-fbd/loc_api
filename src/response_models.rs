use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a value that can be either a single [`String`] or a `Vec<String>`.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum StringOrArray {
    String(String),
    Array(Vec<String>),
}

/// Represents a value that can be either a [`u32`] or a [`String`].
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum NumberOrString {
    Number(u32),
    String(String),
}

/// Represents a value that can be either a [`bool`] or a [`String`].
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

/// Represents a single filter within a [`FacetRes`].
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
    /// Available options for [`perpage`].
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
    pub use_field: Option<StringOrArray>, // [`use`] is a reserved keyword in Rust
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

/// Represents the item attribute object within [`ItemResponse`] and [`ResourceResponse`].
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

/// Represents a single resource object within [`ItemResponse`] and [`ResourceResponse`].
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
