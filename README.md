# picopik Server 

The backend for the picopik app! 

# Running 

To run the development server (with hot reloading!)

Install `postgresql-devel` and `libpq-dev`

```
cargo install diesel_cli --no-default-features --features postgres
docker-compose -f deployments/development.yml up
diesel setup 
diesel migrate run
```
To run without hot reloading, run 
```
docker-compose -f docker-compose.yml
```

move the example.env to just ".env" with your configurations to run
