CREATE TABLE IF NOT EXISTS edge (
    edge_id         INTEGER PRIMARY KEY,
    node_ini        INTEGER,
    node_end        INTEGER,
    length_m        DOUBLE,
    bearing_deg     DOUBLE;
