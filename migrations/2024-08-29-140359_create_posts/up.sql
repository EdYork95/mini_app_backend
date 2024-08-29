-- Your SQL goes here
CREATE TABLE posts( 
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    body TEXT NOT NULL,
    imageUrl VARCHAR NOT NULL,
    date_created TIMESTAMP  DEFAULT CURRENT_TIMESTAMP NOT NULL
)