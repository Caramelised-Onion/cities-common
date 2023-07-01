use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct CitiesQuery {
    pub country: Option<String>,
    pub radius: Option<usize>,
    pub point: Option<String>,
    pub sort_by_random: Option<bool>,
    pub sort_by_distance: Option<bool>,
    pub sort_by_population: Option<bool>,
    pub minimum_population: Option<i32>,
    pub limit: Option<usize>,
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct DistQuery {
    pub city_id1: i64,
    pub city_id2: i64,
}

