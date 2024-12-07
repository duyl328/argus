create table __diesel_schema_migrations
(
    version VARCHAR(50)                         not null
        primary key,
    run_on  TIMESTAMP default CURRENT_TIMESTAMP not null
);

create table posts
(
    id        INTEGER           not null
        primary key autoincrement,
    title     VARCHAR           not null,
    body      TEXT              not null,
    published BOOLEAN default 0 not null
);

create table sqlite_master
(
    type     TEXT,
    name     TEXT,
    tbl_name TEXT,
    rootpage INT,
    sql      TEXT
);

create table sqlite_sequence
(
    name,
    seq
);

create table user
(
    id   INTEGER
        primary key autoincrement,
    name TEXT not null,
    age  INTEGER
);
