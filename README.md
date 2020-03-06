# postman
![Cargo CI](https://github.com/Technical27/postman/workflows/Cargo%20CI/badge.svg)
![Docker CI](https://github.com/Technical27/postman/workflows/Docker%20CI/badge.svg)

a discord bot that gets images from reddit. simple.

## building
### config files
first, create a `.env` file that has the following keys:
- `POSTMAN_DATABASE_URL`: location of a postgres database (not required for docker)
- `POSTMAN_DISCORD_TOKEN`: the discord token the bot will use
- `POSTMAN_DEFAULT_SUB`: the default subreddit that commands use
- `POSTMAN_ADMIN`: the discord id of the admin user (most likely yours)
- `POSTMAN_COOLDOWN_TIME`: the cooldown between comands in seconds (default 3 seconds)
- `POSTMAN_PREFIX`: the prefix the bot will use

```bash
# not required for docker
POSTMAN_DATABASE_URL=postgres://user:pass@hostname:port/database
POSTMAN_DISCORD_TOKEN=your.token.here
POSTMAN_DEFAULT_SUB=memes
POSTMAN_ADMIN=99999999999
# not required (default 3 seconds)
POSTMAN_COOLDOWN_TIME=5
POSTMAN_PREFIX=-
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
