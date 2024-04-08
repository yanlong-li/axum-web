CREATE schema if not exists `axum` collate utf8mb4_general_ci;

CREATE TABLE if not exists `user`
(
    `id`       bigint unsigned NOT NULL AUTO_INCREMENT,
    `username` varchar(32)     NOT NULL,
    PRIMARY KEY (`id`)
);

insert into `user`(username)
values ('test');