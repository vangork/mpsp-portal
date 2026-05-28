-- Your SQL goes here
CREATE TABLE `receivers` (
  `id` INT AUTO_INCREMENT PRIMARY KEY,
  `name` varchar(128) NOT NULL,
  `address` varchar(256) NOT NULL,
  `phone` varchar(32) NOT NULL,
  `email` varchar(128) NOT NULL,
  `default` BOOLEAN NOT NULL,
  `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
