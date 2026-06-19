CREATE TABLE source (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    index_date TEXT NOT NULL
);

CREATE TABLE item (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source_id INTEGER NOT NULL REFERENCES source(id) ON DELETE CASCADE,
    item_offset INTEGER NOT NULL,
    item_line_num INTEGER NOT NULL,
    item_col_num INTEGER NOT NULL,
    comment_content TEXT NOT NULL,
    -- REFERENCES vec(rowid)
    comment_vec_id INTEGER NOT NULL
);

CREATE VIRTUAL TABLE vec USING vec0(
    value FLOAT[768] DISTANCE_METRIC=COSINE
);

CREATE TABLE setting (
    key TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL
) WITHOUT ROWID;

CREATE INDEX idx_source_path ON source(path);
