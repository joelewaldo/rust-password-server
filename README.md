<div align="left" style="position: relative;">
<h1>RUST-PASSWORD-SERVER</h1>
<p align="left">
	<em>Unlocking security with Rust's power</em>
</p>
<p align="left">
	<img src="https://img.shields.io/github/license/joelewaldo/rust-password-server?style=default&logo=opensourceinitiative&logoColor=white&color=0080ff" alt="license">
	<img src="https://img.shields.io/github/last-commit/joelewaldo/rust-password-server?style=default&logo=git&logoColor=white&color=0080ff" alt="last-commit">
	<img src="https://img.shields.io/github/languages/top/joelewaldo/rust-password-server?style=default&color=0080ff" alt="repo-top-language">
	<img src="https://img.shields.io/github/languages/count/joelewaldo/rust-password-server?style=default&color=0080ff" alt="repo-language-count">
</p>
<p align="left"><!-- default option, no dependency badges. -->
</p>
<p align="left">
	<!-- default option, no dependency badges. -->
</p>
</div>
<br clear="right">

## 🔗 Table of Contents

- [📍 Overview](#-overview)
- [👾 Features](#-features)
- [🚀 Getting Started](#-getting-started)
  - [☑️ Prerequisites](#-prerequisites)
  - [⚙️ Installation](#-installation)
  - [🤖 Usage](#🤖-usage)
  - [🧪 Testing](#🧪-testing)

---

## 📍 Overview

The rust-password-server project is a secure Rust-based solution that simplifies password management by encrypting and storing passwords in a PostgreSQL database. It offers features like HTTP routing, encryption, and asynchronous tasks. Ideal for developers seeking a robust, open-source password server for enhanced data security and efficient password operations.

---

## 👾 Features

|     |      Feature      | Summary                                                                                                                                                                                                                                                                                                    |
| :-- | :---------------: | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ⚙️  | **Architecture**  | <ul><li>Implements a Rust password server using various dependencies like `aes-gcm`, `axum`, `sqlx`, and `tokio`</li><li>Manages encryption, HTTP routing, database interactions, and asynchronous tasks</li><li>Organized into bounded context modules for clear boundaries and maintainability</li></ul> |
| 🔩  | **Code Quality**  | <ul><li>Well-structured codebase with clear separation of concerns</li><li>Utilizes Rust's strong type system for robustness</li><li>Follows best practices for error handling and async programming</li></ul>                                                                                             |
| 📄  | **Documentation** | <ul><li>Comprehensive documentation in primary language `Rust`</li><li>Includes detailed explanations of configuration settings and environment variables</li><li>Provides usage and test commands for easy setup and testing</li></ul>                                                                    |
| 🔌  | **Integrations**  | <ul><li>Integrates with `PostgreSQL` for database operations</li><li>Utilizes `Axum` for HTTP routing and handling</li><li>Includes `Docker` configurations for easy deployment and testing</li></ul>                                                                                                      |
| 🧩  |  **Modularity**   | <ul><li>Encapsulates domain logic within bounded context modules</li><li>Clear separation of infrastructure, domain, utility, and application components</li><li>Facilitates easier navigation and understanding of the codebase</li></ul>                                                                 |
| 🧪  |    **Testing**    | <ul><li>Automated testing setup using `GitHub Actions` on push to main branch</li><li>Ensures test environment setup with required secrets and configurations</li><li>Utilizes Rust toolchain and caches dependencies for efficient testing</li></ul>                                                      |
| ⚡️ |  **Performance**  | <ul><li>Utilizes `tokio` for asynchronous task handling</li><li>Efficient encryption and decryption using `aes-gcm` algorithm</li><li>Optimized HTTP routing with `Axum` framework</li></ul>                                                                                                               |
| 🛡️  |   **Security**    | <ul><li>Implements secure data handling with `AES-GCM` encryption</li><li>Manages database connections securely for password storage</li><li>Defines environment variables for secure configuration</li></ul>                                                                                              |
| 📦  | **Dependencies**  | <ul><li>Includes essential dependencies like `serde_json`, `uuid`, `sqlx`, and more</li><li>Manages dependencies using `Cargo.toml` and `Cargo.lock`</li><li>Utilizes `dotenv` for environment variable management</li></ul>                                                                               |

---

## 🚀 Getting Started

### ☑️ Prerequisites

Before getting started with rust-password-server, ensure your runtime environment meets the following requirements:

- **Programming Language:** Rust
- **Package Manager:** Cargo
- **Container Runtime:** Docker

### ⚙️ Installation

Install rust-password-server using one of the following methods:

**Build from source:**

1. Clone the rust-password-server repository:

```sh
❯ git clone https://github.com/joelewaldo/rust-password-server
```

2. Navigate to the project directory:

```sh
❯ cd rust-password-server
```

3. Install the project dependencies:

**Using `cargo`** &nbsp; [<img align="center" src="https://img.shields.io/badge/Rust-000000.svg?style={badge_style}&logo=rust&logoColor=white" />](https://www.rust-lang.org/)

```sh
❯ cargo build
```

**Using `docker`** &nbsp; [<img align="center" src="https://img.shields.io/badge/Docker-2CA5E0.svg?style={badge_style}&logo=docker&logoColor=white" />](https://www.docker.com/)

```sh
❯ docker build -t joelewaldo/rust-password-server .
```

### 🤖 Usage

Run rust-password-server using the following command:
**Using `cargo`** &nbsp; [<img align="center" src="https://img.shields.io/badge/Rust-000000.svg?style={badge_style}&logo=rust&logoColor=white" />](https://www.rust-lang.org/)

1. Start the database using Docker Compose:

```sh
❯ cd docker
```

```sh
❯ docker-compose up -d
```

2. Run the server with Cargo:

```sh
❯ cargo run
```

**Using `docker`** &nbsp; [<img align="center" src="https://img.shields.io/badge/Docker-2CA5E0.svg?style={badge_style}&logo=docker&logoColor=white" />](https://www.docker.com/)

1. Navigate to the Docker directory:

```sh
❯ cd docker
```

2. Start the server:

```sh
❯ docker-compose up -d
```

The server will now be running on `localhost:8000`.

### 🧪 Testing

Run the test suite using the following command:
**Using `cargo`** &nbsp; [<img align="center" src="https://img.shields.io/badge/Rust-000000.svg?style={badge_style}&logo=rust&logoColor=white" />](https://www.rust-lang.org/)

```sh
❯ cargo test
```
