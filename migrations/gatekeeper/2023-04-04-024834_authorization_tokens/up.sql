CREATE TABLE authorization_tokens (
    id serial PRIMARY KEY,
    user_id INT NOT NULL,
    permissions jsonb,
    FOREIGN KEY (user_id)
        REFERENCES users(id)
);
