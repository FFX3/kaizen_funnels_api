ALTER TABLE funnels
ADD deleted_at TIMESTAMP;

ALTER TABLE variations
ADD deleted_at TIMESTAMP;
