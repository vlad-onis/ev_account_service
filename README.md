# ev_account_service

## Compile and run
```bash
# This command build the image and generates some config files along the way
docker build --build-arg db_host=<IP_OF_DB_HOST> -f docker/Dockerfile .

# Run the service exposing the DB port
docker run -p 50051:50051 -it <IMAGE_TAG>
```

## Run tests
```bash
# Run only integration tests
./scripts/dev_db_init.sh
cargo test --test '*'

# Run only unit tests
cargo test --lib

# Run all tests available
cargo test
```