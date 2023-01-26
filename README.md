# ev_account_service

EV Account service is a microservice meant to handle all the account operations such as: login, signup, account updates and more. It is build on top of the Grpc communication protocol and uses protobuf to define messages.

## Compile and run

```bash
# Script that initialises and runs the db container
# Without a running db, account service will fail to compile
./scripts/dev_db_init.sh
```


```bash
# This command build the image and generates some config files along the way
docker build --build-arg db_host=<IP_OF_DB_HOST> -f docker/Dockerfile .

# Run the service exposing the DB port
docker run -p 50051:50051 -it <IMAGE_TAG>
```

```bash
# Call the signup endpoint using the grpcurl utilitary
grpcurl -plaintext -import-path ./protos -proto endpoints.proto -d '{"username":"vladdd", "password":"test", "email":"test1@gmail.com"}' '192.168.0.164:50051' account_service_rpc.AccountService/signUp
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