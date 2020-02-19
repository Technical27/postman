create table guilds (
  id bigint not null primary key
);

create table users (
  id bigint not null primary key,
  rank bigint not null
);

create table guildusers (
  guild_id bigint not null,
  user_id bigint not null,
  primary key (user_id, guild_id),
  foreign key(guild_id) references guilds(id) on update cascade,
  foreign key(user_id) references users(id) on update cascade
);
