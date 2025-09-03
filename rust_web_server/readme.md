# Rust HTTP Web Server

A simple, concurrent, multi-file HTTP/1.1 web server built from scratch in Rust as a learning exercise.

This project was undertaken to directly compare the experience of building a low-level network service in Rust versus its C counterpart, focusing on safety, concurrency, and developer ergonomics.

---

## Features

* **Serves Static Files:** Serves `index.html`, images, and other files from the project directory.
* **Concurrent:** Handles multiple clients simultaneously using Tokio's asynchronous, multi-threaded runtime.
* **Request Routing:** Parses the path from incoming `GET` requests to serve the correct file.
* **HTTP Responses:** Provides `200 OK` responses for found files and `404 Not Found` for missing files.

---

## The C vs. Rust Learning Journey

This server was built immediately after creating an identical one in C using `fork()` and the raw sockets API. The goal was to experience the practical differences between the two languages for this classic systems programming task.

The key takeaways were profound:

* **Memory Safety:** Rust's compile-time ownership and borrow checking completely eliminated the risk of buffer overflows and memory leaks. Vulnerabilities that required careful manual prevention in C were simply not possible in safe Rust.
* **Concurrency Model:** Tokio's `async/await` and lightweight tasks provided a vastly more scalable and efficient concurrency model compared to the traditional `fork()`-per-client approach. A single Rust process can handle thousands of connections with minimal overhead.
* **Error Handling:** The `Result` enum and `?` operator forced robust, explicit error handling, making the final code far more reliable than the C version's manual checking of return codes and `errno`.
* **Developer Experience:** High-level abstractions like `tokio::fs::read` and powerful tools like `cargo` made the development process faster and more focused on the core logic rather than low-level boilerplate.

---

## Getting Started

### Prerequisites

Ensure you have the Rust toolchain installed. You can get it from the official site: [rustup.rs](https://rustup.rs/).

### Building and Running

1.  **Clone the repository:**
    ```bash
    # Replace with your actual repo URL
    git clone [https://github.com/yakitoritrash/rust_web_server.git](https://github.com/yakitoritrash/rust_web_server.git)
    cd rust_web_server
    ```

2.  **Place your content:**
    Make sure you have an `index.html` file and any other files you want to serve (like `image.jpg`) in the root of the project directory. A `404.html` file is also recommended.

3.  **Build the project in release mode:**
    ```bash
    cargo build --release
    ```

4.  **Run the server:**
    ```bash
    ./target/release/rust_web_server
    ```

The server will now be listening on `http://localhost:8080`.

---

## License

This project is licensed under the MIT License.
