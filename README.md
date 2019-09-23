# Isotope Server 

The backend for the isotope app! 

# Running 

```
cargo install diesel_cli --no-default-features --features postgres
docker-compose -f deployments/development.yml up  
diesel setup 
diesel migrate run
```

move the example.env to just ".env" with your configurations to run
