CREATE TABLE moods (
  id INTEGER NOT NULL PRIMARY KEY,
  value INTEGER NOT NULL UNIQUE,
  name VARCHAR NOT NULL UNIQUE
);

INSERT INTO moods(value, name) VALUES(-2, 'awful');
INSERT INTO moods(value, name) VALUES(-1, 'bad');
INSERT INTO moods(value, name) VALUES(0, 'OK');
INSERT INTO moods(value, name) VALUES(1, 'good');
INSERT INTO moods(value, name) VALUES(2, 'awesome');