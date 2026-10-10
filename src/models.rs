use diesel::prelude::*;

use crate::schema::reviews;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::reviews)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Review {
    pub id: i32,
    pub title: String,
    pub rating: f32,
    pub time_date: String,
}

#[derive(Insertable)]
#[diesel(table_name = reviews)]
pub struct NewReview<'a> {
    pub title: &'a str,
    pub rating: &'a f32,
    pub time_date: &'a str,
}
