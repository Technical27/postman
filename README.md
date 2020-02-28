# postman
![Cargo CI](https://github.com/Technical27/postman/workflows/Cargo%20CI/badge.svg)
![Docker CI](https://github.com/Technical27/postman/workflows/Docker%20CI/badge.svg)

a discord bot that gets images from reddit. simple.

## building
### config files
first, create a `.env` file that has 2 keys: a `DISCORD_TOKEN` and `DATABASE_URL`.

`DISCORD_TOKEN` is your discord token and `DATABASE_URL` is the location of a postgres database.

*note*: `DATABASE_URL` isn't required for docker only `DISCORD_TOKEN` is.
```bash
DISCORD_TOKEN=your.token.here
# not required for docker
DATABASE_URL=postgres://user:pass@hostname:port/database
```

then, create a `config.json` with these properties:
  - `default_sub`:
       the default subreddit that will be used if the user doesn't specify one
  - `admin`:
      the discord id of the bot admin (used to allow access to admin commands)
  - `cooldown_time`:
      the cooldown time between commands
  - `prefix`:
      the prefix that the bot will use

example:
```json
{
  "default_sub": "memes",
  "admin": 99999999999,
  "cooldown_time": 3,
  "prefix": "~"
}
```

### building with `cargo`

*note:* you need

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

### building with `docker`

build the image
```bash
$ docker-compose build
```

run
```bash
$ docker-compose up -d
```

## backing up the database
### docker
backup
```bash
$ sudo docker exec postman_database_1 pg_dumpall -c -U postman > dump.sql
```
to restore copy the sql as `backup.sql` into the root directory
```bash
$ cp ../your-backup.sql backup.sql
```
and then rebuild the image and run
```bash
$ sudo docker-compose build
$ sudo docker-compose up -d
```
### cargo
backup
```bash
$ pg_dumpall -c -U database_user > dump.sql
```
restore
```bash
$ cat dump.sql | psql -U database_user
```
## license

following rust conventions, this bot is dual licensed under the MIT and APACHE-2.0 licenses.
