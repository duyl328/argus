-- Your SQL goes here
ALTER TABLE photo_table ADD COLUMN is_algorithm BOOLEAN;

ALTER TABLE photo_table ADD COLUMN algorithm_score INTEGER;

ALTER TABLE photo_table ADD COLUMN last_viewed_time BIGINT;

ALTER TABLE photo_table ADD COLUMN offset_time TEXT;

ALTER TABLE photo_table ADD COLUMN rating INTEGER;
