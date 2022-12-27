# ev_account_service

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