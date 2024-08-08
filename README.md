# Just Clean Up (Work in progress)
Simple API written in Rust with the usage of Axum framework. Ultimately, this is to be the backend of an application used to manage schedules and cleaning duties.
___

## Usage
### Pre-requirements
To run application it is necessary to specify a few environment variables:
- required:
  - `PORT` - port on which the API should listening
  - `BASE_PATH` - the base path of the API
  - `DATABASE_URL` - URL to connect to the database (at the moment only MongoDB is supported)
  - `JWT_SECRET` - some secret used to encrypting and decrypting JWT tokens (in a future it will be replaced with a pair of RSA keys)
- optional:
  - `RUST_LOG=just_clean_up=<logging_level>` - level of the logging
  - `DEV_MODE` - bool specifying if the API should run in development mode

These environment variables could be placed in `.env` file in the root project directory. If so, then it will be loaded on the application startup.
___

### Running
#### Production purposes
To build and run server execute command below:
```bash
cargo build --release && target/just-clean-up
```
___

#### Development purposes
For development purposes it is recommended to use command:
```bash
cargo run
```
___

### Testing
#### Unit tests
TODO
___
#### Integration tests
TODO