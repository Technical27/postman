# postman
![Cargo CI](https://github.com/Technical27/postman/workflows/Cargo%20CI/badge.svg)
![Docker CI](https://github.com/Technical27/postman/workflows/Docker%20CI/badge.svg)

a discord bot that gets images from reddit. simple.

## building
### config files
first, create a `.env` file that has 2 keys: a `DISCORD_TOKEN` and `DATABASE_URL`.

`DISCORD_TOKEN` is your discord token and `DATABASE_URL` is the location of a postgres database.

```env
DISCORD_TOKEN=your.token.here
DATABASE_URL=postgres://user:pass@hostname:port/database
```

then, create a `config.json` with these properties:
  - `default_sub` - the default subreddit that will be used if the user doesn't specify one
  - `admin` - the discord id of the bot admin (used to allow access to admin commands)
  - `cooldown_time` - the cooldown time between commands
  - `prefix` - the prefix that the bot will use

example:
```json
{
  "default_sub": "memes",
  "admin": 99999999999,
  "cooldown_time": 3,
  "prefix": "~"
}
```

### manual build

install the `diesel` cli
```bash
$ cargo install --no-default-features --features postgres diesel_cli
```

run migrations
```bash
$ diesel migration run
```

build
```bash
$ cargo build --release
```

to run use
```bash
$ cargo run --release
```
or
```bash
$ ./target/release/postman
```

### docker build

build the image
```bash
$ docker-compose build
```

run
```bash
$ docker-compose up -d
```

find the bot image name
```bash
$ sudo docker-compose ps
       Name                     Command              State    Ports
---------------------------------------------------------------------
postman_bot_1        postman                         Up
postman_database_1   docker-entrypoint.sh postgres   Up      5432/tcp
```
and run migrations
```bash
$ sudo docker exec postman_bot_1 /bin/bash -c "diesel migration run"
```

## license

following rust conventions, this bot is dual licensed under the MIT and APACHE-2.0 licenses.
