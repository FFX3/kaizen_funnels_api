CREATE TABLE authorization_tokens (
    id serial PRIMARY KEY,
    user_id INT NOT NULL,
    permissions jsonb NOT NULL,
    key VARCHAR(255) NOT NULL,
    FOREIGN KEY (user_id)
        REFERENCES users(id)
);
