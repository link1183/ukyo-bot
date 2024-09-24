CREATE TABLE IF NOT EXISTS `users` (
	`id` int AUTO_INCREMENT NOT NULL UNIQUE,
	`discord_id` bigint UNSIGNED NOT NULL UNIQUE,
	PRIMARY KEY (`id`)
);

CREATE TABLE IF NOT EXISTS `boot` (
	`id` int AUTO_INCREMENT NOT NULL UNIQUE,
	`user_id` int NOT NULL,
	`score` double NOT NULL,
	`date` datetime NOT NULL,
	PRIMARY KEY (`id`)
);

CREATE TABLE IF NOT EXISTS `messages` (
	`id` int AUTO_INCREMENT NOT NULL UNIQUE,
	`message` varchar(200) NOT NULL,
	`suggestion_date` datetime NOT NULL,
	`user_id` int NOT NULL,
	PRIMARY KEY (`id`)
);


ALTER TABLE `boot` ADD CONSTRAINT `boot_fk1` FOREIGN KEY (`user_id`) REFERENCES `users`(`id`);
ALTER TABLE `messages` ADD CONSTRAINT `messages_fk3` FOREIGN KEY (`user_id`) REFERENCES `users`(`id`);
