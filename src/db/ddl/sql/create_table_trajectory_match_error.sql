CREATE TABLE IF NOT EXISTS trajectory_match_error (
    id          INTEGER PRIMARY KEY,
    traj_id     INTEGER NOT NULL,
    match_error TEXT NOT NULL,
    FOREIGN KEY (traj_id) REFERENCES trajectory(traj_id)
);
