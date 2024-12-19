-- Your SQL goes here
CREATE TABLE posts (
                       id          INTEGER               not null
                           primary key autoincrement,
                       is_delete BOOLEAN NOT NULL DEFAULT false,
                       create_time BIGINT default 0 NOT NULL,
                       update_time BIGINT default 0 NOT NULL,
                       title VARCHAR NOT NULL,
                       body TEXT NOT NULL,
                       published BOOLEAN NOT NULL DEFAULT false
);
CREATE TABLE photo_storages (
                                id          INTEGER               not null
                                    primary key autoincrement,
                                img_paths TEXT NOT NULL,
                                is_enable BOOLEAN NOT NULL DEFAULT true,
                                is_delete BOOLEAN NOT NULL DEFAULT false,
                                create_time BIGINT default 0 NOT NULL,
                                update_time BIGINT default 0 NOT NULL
);
