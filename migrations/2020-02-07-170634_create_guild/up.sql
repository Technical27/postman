create table guilds (
  id bigint not null primary key
);

create table users (
  id bigint not null primary key,
  guild_id bigint not null,
  foreign key(guild_id) references guild(id)
);

create table guildusers (
  guild_id bigint not null primary key,
  user_id bigint not null,
  foreign key(guild_id) references guild(id),
  foreign key(user_id) references users(id)
);
