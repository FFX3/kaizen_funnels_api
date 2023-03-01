ALTER TABLE steps
ADD "order" INT;

UPDATE steps
SET "order" = 0;

ALTER TABLE steps
ALTER COLUMN "order" SET NOT NULL;
