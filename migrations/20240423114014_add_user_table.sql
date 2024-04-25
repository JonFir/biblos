CREATE TABLE IF NOT EXISTS `users` (
    `id` INTEGER PRIMARY KEY NOT NULL,
    `password` TEXT NOT NULL,
    `last_login` NUMERIC DEFAULT NULL,
    `is_superuser` NUMERIC NOT NULL,
    `username` TEXT NOT NULL,
    `is_staff` NUMERIC NOT NULL,
    `is_active` NUMERIC NOT NULL,
    `date_joined` NUMERIC NOT NULL,
    `email` TEXT NOT NULL,
    `boosty_email` TEXT NOT NULL,
    `boosty_level` TEXT NOT NULL,
    `github_nikname` TEXT NOT NULL
);