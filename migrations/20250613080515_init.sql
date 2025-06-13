create table polls (
    id integer primary key generated always as identity,
    title varchar(255) not null,
    id uuid not null,
    allowed_usernames text[]
);

create table polls_options (
    id biginteger primary key generated always as identity,
    name varchar(255) not null,
    poll_id integer not null,
    foreign key (poll_id) references poll(id)
);

create table user_poll (
    id biginteger primary key generated always as identity,
    name varchar(32) not null,
    poll_id integer not null,
    voted_at timestamp not null default now,
    -- uniq name,poll_id
    foreign key (poll_id) references poll(id)
);

create table vote (
    option_id biginteger not null,
    user_poll_id biginteger not null,
    note smallinteger not null,

    primary key (option_id, user_poll_id),

    foreign key (option_id) references option(id),
    foreign key (user_poll_id) references user_poll(id)
);
