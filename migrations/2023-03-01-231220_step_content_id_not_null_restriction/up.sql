UPDATE steps
SET content_id = (SELECT id FROM contents LIMIT 1);

ALTER TABLE steps
ALTER COLUMN content_id SET NOT NULL;
