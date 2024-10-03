-- crates table
create table if not exists crates (
    `id` integer primary key autoincrement,
    `name` varchar(255) not null unique,
    `description` varchar(4096),
--     `created_at` varchar(25) not null,
--     `updated_at` varchar(25) not null,
--     `downloads` bigint not null default 0,
    `documentation` varchar(1024),
    `repository` varchar(1024)
);

-- crate version
create table if not exists crate_versions (
    `id` integer primary key autoincrement,
    `name` varchar(255) not null,
    `version` varchar(127) not null,
    `downloads` bigint not null default 0,
    `created_at` integer not null,
    UNIQUE (`name`, `version`)
);

-- user accounts
create table if not exists users (
    `id` integer primary key autoincrement,
    `email` varchar(255) not null unique,
    `name` varchar(255) not null,
    `password` varchar(255) not null
);

-- user sessions
create table if not exists sessions (
    `token` varchar(255) primary key,
    `user_id` integer not null,
    `expires` integer not null,
    foreign key (`user_id`) references `users`(`id`) on update cascade on delete cascade
);

-- api tokens
create table if not exists tokens (
    `id` integer primary key autoincrement,
    `name` varchar(255) not null,
    `token` varchar(255) not null unique,
    `scope` integer not null,
    `user_id` integer not null,
    foreign key (`user_id`) references `users`(`id`) on update cascade on delete cascade
);

-- crate owners
create table if not exists crate_owners (
    `id` integer primary key,
    `crate_id` integer not null,
    `user_id` integer not null,
    foreign key (`crate_id`) references `crates`(`id`) on update cascade on delete cascade,
    foreign key (`user_id`) references `users`(`id`) on update cascade on delete cascade
);
