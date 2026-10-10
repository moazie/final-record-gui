use std::env;

use chrono::Local;
use diesel::prelude::*;
use diesel::{Connection, RunQueryDsl, SelectableHelper, SqliteConnection};
use dotenvy::dotenv;

use crate::models::{NewReview, Review};
use crate::{reviews, schema};

pub fn date_print() -> String {
    let date = Local::now();
    let formated: String = date.format("%Y-%m-%d").to_string();
    formated
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_review(
    conn: &mut SqliteConnection,
    title: &str,
    rating: &f32,
    time_date: &str,
) -> Review {
    use crate::schema::reviews;

    let new_review = NewReview {
        title,
        rating,
        time_date,
    };

    diesel::insert_into(reviews::table)
        .values(&new_review)
        .returning(Review::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn delete_review(idx: i32) -> Result<(), diesel::result::Error> {
    let connection = &mut establish_connection();

    let old_count = reviews.count().first::<i64>(connection);
    diesel::delete(reviews.filter(schema::reviews::id.eq(idx))).execute(connection)?;
    assert_eq!(
        old_count.map(|count| count - 1),
        reviews.count().first(connection)
    );

    Ok(())
}

pub fn delete_all() -> Result<(), diesel::result::Error> {
    let connection = &mut establish_connection();

    diesel::delete(reviews).execute(connection)?;
    diesel::sql_query("DELETE FROM sqlite_sequence WHERE name = 'reviews';").execute(connection)?;
    assert_eq!(Ok(0), reviews.count().first::<i64>(connection));

    Ok(())
}

pub fn display_reviews() -> &'static str {
    let connection = &mut establish_connection();
    let mut final_string = String::new();
    let new_line = String::from("\n");
    let results = reviews
        .select(Review::as_select())
        .load(connection)
        .expect("Error Loading");
    for review in results {
        let id_string = review.id.to_string();
        final_string.push_str(&id_string);
        final_string.push_str(" | ");
        final_string.push_str(&review.title);
        final_string.push_str(" | ");
        final_string.push_str(&review.time_date);
        final_string.push_str(&new_line);

        final_string.push_str(&star_display(review.rating));
        final_string.push_str(&new_line);
    }

    let static_ref: &'static mut str = final_string.leak();
    static_ref
}

fn star_display(mut stars: f32) -> String {
    let mut chars = [' '; 10];
    stars /= 2.;
    let full_stars: i32 = stars as i32;
    let fraction = stars - full_stars as f32;

    for count in 0..5 {
        let arr_idx = (count * 2) as usize;

        if count < full_stars {
            chars[arr_idx] = '★';
        } else if count == full_stars && fraction > 0. {
            if fraction >= 0.75 {
                chars[arr_idx] = '¾';
            } else if fraction >= 0.5 {
                chars[arr_idx] = '½';
            } else if fraction >= 0.25 {
                chars[arr_idx] = '¼';
            } else {
                chars[arr_idx] = '☆';
            }
        } else {
            chars[arr_idx] = '☆';
        }

        if (arr_idx + 1) < chars.len() {
            chars[arr_idx + 1] = ' ';
        }
    }
    let final_str: String = chars.iter().collect();
    final_str
}
