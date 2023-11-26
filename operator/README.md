# operator

## shuttle

run `cargo install cargo-shuttle` to install [shuttle](https://docs.shuttle.rs/).

## environment variables

rename `Secrets.example.toml` to `Secrets.toml` and fill :

- `DISCORD_TOKEN` : bot discord token
- `KEYPAIR` : json bytes keypair as string
- `OWNER` : oracle owner

## run locally

`cargo shuttle run`

## deploy on shuttle

run `cargo shuttle deploy` : you'll be prompted to login using discord to create an account on shuttle.
