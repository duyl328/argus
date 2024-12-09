pub struct DbInit {
    /// 表名称
    pub name: String,
    /// 创建所需 sql
    pub sql: String,
}

/// 获取初始化数据库列表
pub fn get_init_sql_list() -> Vec<DbInit> {
    let migration_list: Vec<DbInit> = vec![
        // =======================================================================
        DbInit {
            name: "posts".to_string(),
            sql: "
create table posts
(
    id          INTEGER               not null
        primary key autoincrement,
    title       VARCHAR               not null,
    body        TEXT                  not null,
    published   BOOLEAN default 0     not null,
    create_time BIGINT  default 0     not null,
    update_time BIGINT  default 0     not null,
    is_delete   BOOLEAN default FALSE not null
);
".to_string(),
        },
        // =======================================================================
        DbInit {
            name: "db_version".to_string(),
            sql: "
create table user
(
    id   INTEGER
        primary key autoincrement,
    version  INTEGER,
    create_time BIGINT  default 0     not null,
    update_time BIGINT  default 0     not null,
);
".to_string(),
        },
        // =======================================================================
    ];

    migration_list
}