CREATE TABLE source (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    index_date TEXT NOT NULL
);

CREATE TABLE item (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source_id INTEGER NOT NULL REFERENCES source(id) ON DELETE CASCADE,
    offset INTEGER NOT NULL,
    line INTEGER NOT NULL,
    col INTEGER NOT NULL,
    comment_content TEXT NOT NULL,
    comment_vec_id INTEGER REFERENCES vec(rowid)
);

CREATE VIRTUAL TABLE vec USING vec0(
    value float[768]
);

CREATE TABLE setting (
    key TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL
) WITHOUT ROWID;

CREATE INDEX idx_source_path ON source(path);
