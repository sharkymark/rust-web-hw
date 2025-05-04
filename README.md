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
*   LLDB for debugging
*   VS Code extensions:
    *   Rust Analyzer
    *   LLDB Debugger
    *   GitHub Copilot
*   AI coding agents: Aider, Goose
*   GitHub Copilot VS Code extension

**Non-root User:**
The Dockerfile creates a non-root user (`vscode`) with UID 1000 and GID 1000. This user is used to run the application and perform development tasks within the container. Rust `cargo`, `rustup`, and AI coding agents Aider and Goose are configured to run as this non-root user.
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

## Debugging

> Be sure to break/end the running Rust application before starting a debug session so there isn't a port conflict preventing the debugging session from starting.

LLDB is a powerful, open-source debugger designed for debugging programs written in languages like C, C++, Rust, and others. It is included in the Dev Container setup, specifically the Dockerfile, for Rust development and debugging.

### VS Code extension

1. The `rust-analyzer` and `vadimcn.vscode-lldb` extensions are included in the Dev Container setup for Rust development and debugging.
1. A `launch.json` file is included in the `.vscode` directory to configure debugging for the Rust application. You can start a debug session by selecting the "Debug Commission Calculator" configuration from the Run and Debug panel in VS Code.
1. Set a breakpoint in your Rust code first.

### Terminal

Open a terminal in the Dev Container and run the following command to start the application in debug mode:

```bash
lldb
```

Once in the LLDB prompt, you can set breakpoints, run the application, and inspect variables. For example:

```bash
(lldb) target create "target/debug/myapp"
(lldb) breakpoint set --file src/main.rs --line <line number you want to set the breakpoint on>
(lldb) breakpoint list
(lldb) run
```
This will start the application in debug mode, and you can use the terminal to set breakpoints, inspect variables, and step through the code.
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
