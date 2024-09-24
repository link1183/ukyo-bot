CREATE TABLE IF NOT EXISTS `messages` (
	`id` int AUTO_INCREMENT NOT NULL UNIQUE,
	`message` varchar(200) NOT NULL,
	`suggestion_date` datetime NOT NULL,
	`user_id` int NOT NULL,
	PRIMARY KEY (`id`)
);

ALTER TABLE `messages` ADD CONSTRAINT `messages_fk3` FOREIGN KEY (`user_id`) REFERENCES `users`(`id`);
