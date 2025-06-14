use {
    super::err::ServerResult,
    crate::utils::wini::cache::add_cache,
    axum::response::Response,
    strum_macros::{EnumIter, EnumString},
};

/// Different cache categories used in the application
#[derive(
    Debug,
    Clone,
    Copy,
    serde::Deserialize,
    Hash,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    EnumIter,
    EnumString,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CacheCategory {
    Html,
    Css,
    Javascript,
    Public,
    Function,
}


/// Add cache to an axum response
pub trait AddCache
where
    Self: Sized,
{
    fn add_cache(self, cache_rule: &str) -> ServerResult<Self>;
}

impl AddCache for Response {
    fn add_cache(self, cache_rule: &str) -> ServerResult<Self> {
        add_cache(self, cache_rule)
    }
}