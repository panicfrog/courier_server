-- Add migration script here
CREATE TABLE IF NOT EXISTS `kv` (
    `kv_id` INTEGER UNIQUE AUTO_INCREMENT,
    `key` VARCHAR(50) NOT NULL,
    `value` VARCHAR(255) NOT NULL,
    `create_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`kv_id`)
)ENGINE=InnoDB DEFAULT CHARSET=utf8 AUTO_INCREMENT=100000;