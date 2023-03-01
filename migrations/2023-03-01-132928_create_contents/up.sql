CREATE TABLE contents (
    id SERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);

ALTER TABLE steps
ADD content_id INT REFERENCES contents(id);
