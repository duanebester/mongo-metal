# Mongo Metal

Make sure you have Rust, Cargo, and Docker installed.

```bash
docker-compose up -d
```

Building:
```bash
cargo build
```

Running:
```bash
RUST_LOG=debug cargo run
```

> Note: Trys to connect to Mongo running via the Docker Compose file.

FrontEnd repo is [mongo-metal-app](https://github.com/duanebester/mongo-metal-app)