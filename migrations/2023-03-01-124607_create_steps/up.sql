CREATE TABLE steps (
    id SERIAL PRIMARY KEY,
    title VARCHAR( 50 ) NOT NULL,
    variation_id SERIAL REFERENCES funnels(id),
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);
