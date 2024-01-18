use crate::schema::gps_real_time;
use diesel::prelude::*;
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::gps_real_time)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GpsTag {
    pub id: i32,
    pub lat: f64,
    pub lon: f64,
    pub alt: f64,
    pub speed: f64,
    pub heading: f64,
}

#[derive(Insertable)]
#[diesel(table_name = gps_real_time)]
pub struct NewTag<'a> {
    pub lat: &'a f64,
    pub lon: &'a f64,
    pub alt: &'a f64,
    pub speed: &'a f64,
    pub heading: &'a f64,
}
