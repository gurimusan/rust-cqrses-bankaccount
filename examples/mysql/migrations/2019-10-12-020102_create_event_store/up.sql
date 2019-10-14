-- Your SQL goes here
CREATE TABLE IF NOT EXISTS tbl_event_store (
    `event_id` bigint(20) UNSIGNED NOT NULL auto_increment,
    `event_body` TEXT NOT NULL,
    `event_type` varchar(250) NOT NULL,
    `stream_id` varchar(250) NOT NULL,
    `stream_version` bigint(20) UNSIGNED NOT NULL,
    `event_occurred_at` datetime NOT NULL,
    KEY (`stream_id`),
    UNIQUE KEY (`stream_id`, `stream_version`),
    PRIMARY KEY (`event_id`)
);

CREATE TABLE IF NOT EXISTS tbl_snapshot (
    `stream_id` varchar(250) NOT NULL,
    `stream_version` bigint(20) UNSIGNED NOT NULL,
    `data` TEXT NOT NULL,
    `created_at` datetime NOT NULL,
    PRIMARY KEY (`stream_id`)
);
