# Contributing

Thank you for your interest in contributing to the project! Below are a few guidelines to help you get started.

## Getting Started

1. Fork the repository and create your branch from `main`:
   ```bash
   git checkout -b feat/your-feature-name
   # or for bug fixes:
   git checkout -b fix/issue-name
   ```

2. Make sure your changes compile, adhere to formatting standards, and pass lints:
   ```bash
   cargo fmt --check
   cargo clippy -- -D warnings
   cargo test
   ```

3. Commit your changes following [Conventional Commits](https://www.conventionalcommits.org/):
   ```bash
   git commit -m "feat(module): add battery status support"
   ```

4. Push your branch to your fork and submit a Pull Request targeting `main`.

## Reporting Bugs & Suggesting Features

- Check existing **Issues** to ensure the topic hasn't already been addressed.
- When opening a bug report, please provide details about your OS, environment, terminal, and steps to reproduce the issue.
