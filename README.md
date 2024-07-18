# Intrastek

## How to develop on it

### Manual method

### This is for developpement only

First thing you will need to install the crate located in `backend/prisma-cli`
For this use the command

```sh
cargo install --locked --path .
```

Then you need to start the database a mysql database on port 3306 example:

```sh
docker run --name docker_name -e MYSQL_ROOT_PASSWORD=example_root_password -e MYSQL_DATABASE=example_name -e MYSQL_USER=example_user -e MYSQL_PASSWORD=example_password -p 3306:3306 mysql
```

Then export the following variable:

```sh
export INTRASTEK_DATABASE_URL=mysql://root:example_root_password@localhost:3306/example_name
```

If you changed the prisma schema please check for a need of a migration.

Then compile the backend and run it

```sh
cargo run
```

Logs can be activitied using the env var `INTRASTEK_LOG`

Finally run de frontend using

```sh
npm run dev
```
