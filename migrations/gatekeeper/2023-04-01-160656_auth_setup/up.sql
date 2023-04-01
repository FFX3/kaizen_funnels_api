CREATE TABLE organizations (
    id serial PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    database_name VARCHAR(50) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP
);

CREATE TABLE users (
    id serial PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    organization_id INT NOT NULL,
    FOREIGN KEY (organization_id)
        REFERENCES organizations(id)
);
