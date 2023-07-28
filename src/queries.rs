use std::fmt::Display;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct CitiesQuery {
    pub country: Option<String>,
    pub radius: Option<usize>,
    pub point: Option<String>,
    pub geometry_in: Option<String>,
    pub geometry_out: Option<String>,
    pub sort_by_random: Option<bool>,
    pub sort_by_distance: Option<SortOrder>,
    pub sort_by_population: Option<SortOrder>,
    pub minimum_population: Option<i32>,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct DistQuery {
    pub city_id1: i64,
    pub city_id2: i64,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct CountryQuery {
    pub country_code: Option<String>
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum SortOrder {
    ASC,
    DESC,
}

impl Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SortOrder::ASC => write!(f, "ASC"),
            SortOrder::DESC => write!(f, "DESC"),
        }
    }
}