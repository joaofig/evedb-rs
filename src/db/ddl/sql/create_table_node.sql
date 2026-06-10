CREATE TABLE IF NOT EXISTS node (
    node_id         INTEGER PRIMARY KEY,
    latitude        DOUBLE,
    longitude       DOUBLE,
    altitude        DOUBLE,
    h3_12           INTEGER
);
