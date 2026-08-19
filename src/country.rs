use serde::{Deserialize, Serialize};
use time::PlainDateTime;
use crate::edition::Edition;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Country {
    code2: Option<String>,
    code3: Option<String>,
    created_at: PlainDateTime,
    editions: Vec<Edition>,
    id: u64,
    intermediate_region: Option<String>,
    intermediate_region_code: Option<String>,
    iso_3166: Option<String>,
    name: Option<String>,
    phone_code: Option<String>,
    region: Option<String>,
    region_code: Option<String>,
    sub_region: Option<String>,
    sub_region_code: Option<String>,
    updated_at: PlainDateTime,
}