create table polls (
    id integer primary key generated always as identity,
    uuid uuid not null,
    title varchar(255) not null,
    allowed_usernames text[]
);

create table polls_options (
    id bigint primary key generated always as identity,
    name varchar(255) not null,
    poll_id integer not null,
    foreign key (poll_id) references polls(id)
);

create table polls_users (
    id bigint primary key generated always as identity,
    name varchar(32) not null,
    poll_id integer not null,
    voted_at timestamp not null default now(),
    -- uniq name,poll_id
    foreign key (poll_id) references polls(id)
);

create table votes (
    option_id bigint not null,
    user_poll_id bigint not null,
    note smallint not null,

    primary key (option_id, user_poll_id),

    foreign key (option_id) references polls_options(id),
    foreign key (user_poll_id) references polls_users(id)
);
