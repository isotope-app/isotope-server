# picopik Server 

The backend for the picopik app! 

# Running The Server

To run the development server (with hot reloading!)

Install `postgresql-devel` and `libpq-dev`

move the example.env to just ".env" with your configurations 

```
cargo install diesel_cli --no-default-features --features postgres
docker-compose -f deployments/dev.yml up
diesel setup 
diesel migrate run
```
To run without hot reloading, run 

```
docker-compose -f docker-compose.yml
```
# Running The CLI 
To develop in the CLI run 

```
docker-compose -f deployments/cli.yml up
docker exec -it deployments_picopik-cli_1 bash

```