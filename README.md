# Rust Project with SurrealDB

This README provides instructions for setting up and running your Rust project, which utilizes SurrealDB for data storage. Ensure you have Rust and Cargo installed; if not, follow the instructions in the official Rust documentation: [Rust Installation](https://www.rust-lang.org/tools/install).

## Getting Started

### Clone the Repository

Begin by cloning this repository to your local machine:

```bash
git clone https://github.com/kris007iron/movies-rust.git
```

### Build the Project

Navigate to the project directory and use Cargo to build the project:

```bash
cd movies-rust
cargo build
```

### Set Up SurrealDB

Before running your project, make sure SurrealDB is correctly set up. You can use the following command to start SurrealDB with the desired options:

```bash
'C:\Program Files\SurrealDB\surreal.exe' start --log trace --user root --pass root file:mydatabase.db
```

Replace `mydatabase.db` with the path to your database file.

### Running the Project

You can run the project using Cargo:

```bash
cargo run
```

## Project Structure

Explain the project structure, including important directories and files. For example:

- `src/`: Contains the Rust source code.
- `Cargo.toml`: Defines project dependencies.

## Usage

- `api/v1/movies` returns all movies
- `api/v1/movies/{id}` returns movie with specific id
- `api/v1/reviews/` is the post method that creates the review

  Frontend for this project will be on my profile soon [here](https://github.com/kris007iron/movies-client)

## License

This project is open-source and available under the MIT License. For details, please see the [LICENSE.md](LICENSE.md) file.

## Contributing

If you'd like to contribute to this project, please follow the guidelines in the [CONTRIBUTING.md](CONTRIBUTING.md) file.

## Acknowledgments

FreeCodeCamp.org project - movie api with spring-boot and react

## Contact

For questions or support, please reach out to [me](kris007.iron@gmail.com).
