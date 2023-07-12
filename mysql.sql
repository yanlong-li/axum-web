CREATE schema `axum` collate utf8mb4_general_ci;

CREATE TABLE `user`
(
    `id`       bigint unsigned NOT NULL AUTO_INCREMENT,
    `username` varchar(32)     NOT NULL,
    PRIMARY KEY (`id`)
);