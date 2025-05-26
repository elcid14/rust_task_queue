Rust Task Queue

A sample project to understand the inner workings of task queues and provide a Celery-like experience in Rust.

Overview

This project aims to explore and demonstrate the implementation of a task queue system in Rust, drawing inspiration from Python's Celery. It serves as an educational tool to delve into concepts such as task scheduling, worker management, and asynchronous processing in Rust.
Features

    Basic task queue implementation

    Worker process management

    Asynchronous task execution

    Makefile integration for streamlined development
    GitHub

Getting Started
Prerequisites

    Rust (latest stable version)

    Cargo (Rust's package manager)

    Rabbitmq running locally(I used docker)

Installation

    Clone the repository:

    git clone https://github.com/elcid14/rust_task_queue.git
    cd rust_task_queue

    Build the project:

    cargo build

    Run the application:

    cargo run

Project Structure

    src/: Contains the main source code for the task queue implementation.

    Cargo.toml: Defines project metadata and dependencies.

    Makefile: Provides build and run commands for convenience.

Usage

To use the task queue:

    Define your tasks in the src/tasks.rs file.

    Register tasks with the queue system.

    Start worker processes to process tasks.

    Enqueue tasks for execution via rabbitmq.

Note: Detailed examples and usage instructions will be provided in future updates.
Contributing

Contributions are welcome! If you'd like to contribute, please fork the repository and submit a pull request.
License

This project is licensed under the MIT License. See the LICENSE file for details.
Acknowledgments

Inspired by Celery and the desire to implement similar functionality in Rust.
