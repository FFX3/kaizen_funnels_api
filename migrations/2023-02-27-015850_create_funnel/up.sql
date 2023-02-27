CREATE TABLE funnels (
    id SERIAL PRIMARY KEY,
    label VARCHAR( 50 ) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP
)