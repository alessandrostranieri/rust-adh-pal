CREATE TABLE mood (
  id SERIAL PRIMARY KEY,
  value INT NOT NULL UNIQUE,
  name VARCHAR NOT NULL UNIQUE
);

INSERT INTO mood(value, name) VALUES(-2, 'awful');
INSERT INTO mood(value, name) VALUES(-1, 'bad');
INSERT INTO mood(value, name) VALUES(0, 'OK');
INSERT INTO mood(value, name) VALUES(1, 'good');
INSERT INTO mood(value, name) VALUES(2, 'awesome');
