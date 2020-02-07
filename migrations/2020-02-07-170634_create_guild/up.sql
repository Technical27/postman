create table guilds (
  id bigint not null primary key
);

create table users (
  id bigint not null primary key,
  guild_id bigint not null,
  foreign key(guild_id) references guild(id)
);
