CREATE TABLE IF NOT EXISTS main.trajectory (
    traj_id     INTEGER PRIMARY KEY,
    vehicle_id  INTEGER NOT NULL,
    trip_id     INTEGER NOT NULL,
    length_m    DOUBLE,
    dt_ini      TEXT,
    dt_end      TEXT,
    duration_s  DOUBLE,
    h3_12_ini   INTEGER,
    h3_12_end   INTEGER
);
