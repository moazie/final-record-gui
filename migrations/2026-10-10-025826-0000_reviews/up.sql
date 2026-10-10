-- Your SQL goes here
CREATE table reviews (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  title VARCHAR NOT NULL,
  rating FLOAT NOT NULL,
  time_date DATE NOT NULL
)
