# Actix Web Server with MongoDB

This project demonstrates basic implementations of the Actix Web with MongoDB.

## Prerequisites

- Rust 1.84.1 or later
- MongoDB 4.4.6 or later
- Git

## Installation

1. Clone the repository

```bash
  git clone https://github.com/geo-gkez/actix-web-mongo-db.git
  cd actix-web-mongo-db
```

2. Install dependencies

```bash
  cargo build
```

## Running the server

1. Start the MongoDB server [you can use the docker-compose](docker-compose.yml)
   - if you are using docker-compose, run the following command:
     ```bash
      docker-compose up -d`
     ```
2. Run the server

  ```bash
   cargo run
  ```

3. The server will be running on `http://localhost:8080`
