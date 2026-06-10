CREATE TABLE IF NOT EXISTS traj_node (
    traj_node_id   INTEGER PRIMARY KEY,
    traj_id        INTEGER NOT NULL,
    node_id        INTEGER NOT NULL,
    FOREIGN KEY (traj_id) REFERENCES trajectory(traj_id),
    FOREIGN KEY (node_id) REFERENCES node(node_id)
);
