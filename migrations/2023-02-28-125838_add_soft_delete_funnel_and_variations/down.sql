-- This file should undo anything in `up.sql`
ALTER TABLE funnels
DROP deleted_at;

ALTER TABLE variations
DROP deleted_at;
