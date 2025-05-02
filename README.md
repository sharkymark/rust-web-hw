# Commission Calculator Web App

This is a Rust web application built with the Rocket framework. It provides a simple interface to calculate sales commissions based on quota, variable compensation, and deal revenue. The application uses Diesel ORM to interact with an SQLite database for storing commission records.

## Features

*   Calculate sales commission based on user inputs.
*   Display historical commission records.
*   Delete selected commission records.
*   Uses Rocket for web framework.
*   Uses Diesel ORM with SQLite for data persistence.
*   Uses Handlebars for HTML templating.

## Development Environment

This project is configured to run within a Dev Container. The setup includes:

*   Rust (latest)
*   SQLite3 and necessary development libraries
*   Diesel CLI (with SQLite support)
*   Other development tools (Git, curl, etc.)
*   GitHub CLI for authentication
*   AI coding agents: Aider, Goose
*   GitHub Copilot VS Code extension

**Setup:**

1.  Ensure you have Docker and the VS Code Dev Containers extension installed.
2.  Open the project folder in VS Code.
3.  When prompted, choose "Reopen in Container".

VS Code will build the Docker image defined in `.devcontainer/Dockerfile` and start the container. The `postCreateCommand` and `postStartCommand` in `.devcontainer/devcontainer.json` will handle:
    *   Authenticating with GitHub CLI.
    *   Running the `init-project.sh` script which:
        *   Initializes the Rust project (`cargo init`) if needed.
        *   Builds the project (`cargo build`).
        *   Runs database migrations (`diesel migration run`) if the database doesn't exist.
        *   Updates dependencies (`cargo update`).
        *   Starts the application (`cargo run`).

Once the container is running and the post-start script completes, the application will be accessible (typically at `http://localhost:8000` if running locally, or via the forwarded port in Codespaces/Dev Containers).

## License

MIT License

Copyright (c) 2025 Mark Milligan

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
