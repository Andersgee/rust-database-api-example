# rust-database-api-example

Example of setting up a mysql database with a rust api.

This is mostly just [Prisma Client Rust getting-started](https://prisma.brendonovich.dev/getting-started/installation) but with an actix server and docker stuff.

### dev

```sh
#1. start db service
sudo docker compose up db

#2. push or pull prisma/schema.prisma
cargo run -p prisma-cli -- db push
#cargo run -p prisma-cli -- db pull

#3. generate src/db.rs (from prisma/schema.prisma)
cargo run -p prisma-cli -- generate

#4 run api (debug mode)
cargo run
```

### run on a another machine

first edit `docker-compose.yml - services.api.image` to `your-hub-user/repo-name`

1. run `docker compose build`
2. upload to docker hub
   `docker push your-hub-user/repo-name`
3. copy these files `.env`, `docker-compose.yml`, `my.cnf`
4. run `docker compose up`

## notes

- clear database with `docker compose down -v`
- [actix extractors](https://actix.rs/docs/extractors/)
- [prisma-client queries](https://prisma.brendonovich.dev/reading-data/find)
