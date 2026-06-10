CREATE TABLE IF NOT EXISTS traj_edge (
    traj_edge_id   INTEGER PRIMARY KEY,
    traj_id        INTEGER NOT NULL,
    edge_id        INTEGER NOT NULL,
    FOREIGN KEY (traj_id) REFERENCES trajectory(traj_id),
    FOREIGN KEY (edge_id) REFERENCES edge(edge_id)
);
