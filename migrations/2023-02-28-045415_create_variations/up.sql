CREATE TABLE variations (
    id SERIAL PRIMARY KEY,
    label VARCHAR( 50 ) NOT NULL,
    funnel_id SERIAL REFERENCES funnels(id),
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP
)
