-- Add migration script here

CREATE TABLE patients (
 id SERIAL PRIMARY KEY,
 name TEXT  NOT NULL CHECK(length(name)<=30),
 age INT NOT NULL  CHECK(age >= 18),
 gender TEXT NOT NULL CHECK(length(gender)<=6),
 email  TEXT UNIQUE NOT NULL,
 contact TEXT NOT NULL
)