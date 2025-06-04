
-- Your SQL goes here
CREATE TABLE photo_storages_table (
       id INTEGER not null PRIMARY KEY AUTOINCREMENT,
       img_paths TEXT NOT NULL,
       is_enable BOOLEAN NOT NULL DEFAULT true,
       is_delete BOOLEAN NOT NULL DEFAULT false,
       create_time BIGINT default 0 NOT NULL,
       update_time BIGINT default 0 NOT NULL
);

CREATE TABLE photo_table (
       id INTEGER not null PRIMARY KEY AUTOINCREMENT, -- id 自动增长主键
       img_path TEXT NOT NULL,                        -- 图像路径
       img_name TEXT NOT NULL,                        -- 文件名称
       hash TEXT NOT NULL UNIQUE,                     -- 文件 Hash（唯一 ID）
       width INTEGER NOT NULL,                        -- 图片宽度
       height INTEGER NOT NULL,                       -- 图片高度
       aspect_ratio REAL NOT NULL,                    -- 图片比例（宽/高）
       file_size BIGINT NOT NULL,                     -- 文件大小（字节）
       format TEXT NOT NULL,                          -- 图片格式（如 JPEG, PNG, WebP）
       notes TEXT,                                    -- 备注信息
       make TEXT,                                     -- 相机制造商
       model TEXT,                                    -- 相机型号
       software TEXT,                                 -- 软件版本
       exposure_time REAL,                            -- 曝光时间
       flash TEXT,                                    -- 闪光灯
       f_number REAL,                                 -- 光圈
       iso INTEGER,                                   -- ISO
       date_time_original BIGINT,                     -- 创建日期（Unix 时间戳）
       max_aperture_value TEXT,                       -- 最大光圈值
       focal_length REAL,                             -- 焦距
       image_width INTEGER,                           -- 宽度
       image_height INTEGER,                          -- 长度
       gps_info TEXT,                                 -- GPS 信息
       exposure_program TEXT,                         -- 曝光程序
       metering_mode TEXT,                            -- 测光模式
       artist TEXT,                                   -- 作者（艺术家）
       is_delete BOOLEAN NOT NULL DEFAULT 0,          -- 是否删除，默认为 0（未删除）
       create_time BIGINT NOT NULL default 0,         -- 创建时间（Unix 时间戳）
       update_time BIGINT NOT NULL default 0,         -- 更新时间（Unix 时间戳）
       is_algorithm BOOLEAN NOT NULL DEFAULT 0,       -- 是否经过算法
       algorithm_score INTEGER,                        -- 算法得分
       last_viewed_time BIGINT,                          -- 最后一次查看时间
       offset_time TEXT,                               -- 时区
       rating INTEGER                                  -- 评分
);
