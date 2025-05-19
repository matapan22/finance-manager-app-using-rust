# Contributing to My Finance App

Thank you for considering contributing to **My Finance App**! 🚀 Your help is highly appreciated. Below are the guidelines to ensure a smooth collaboration.

## How to Contribute

### 1. Fork the Repository
- Click the **Fork** button on the repository's GitHub page.
- Clone your forked repository to your local machine:
  ```bash
  git clone https://gitlab.rlp.net/mathew.siby/finance-manager-app-using-rust
  cd my-finance-app
  ```

### 2. Set Up the Development Environment
- Install Rust and Cargo:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- Install required dependencies:
  ```bash
  cargo install
  ```

### 3. Create a New Branch
- Always work on a new branch before making changes:
  ```bash
  git checkout -b feature-your-feature-name
  ```

### 4. Make Your Changes
- Follow coding best practices.
- Keep commits **small and meaningful**.
- Ensure all changes pass linting and formatting checks:
  ```bash
  cargo fmt
  cargo clippy
  ```

### 5. Run Tests
- Before submitting changes, run tests to ensure stability:
  ```bash
  cargo test
  ```

### 6. Commit Your Changes
- Use clear and meaningful commit messages:
  ```bash
  git add .
  git commit -m "feat: add new budget feature"
  ```

### 7. Push and Open a Pull Request
- Push your changes to your forked repository:
  ```bash
  git push origin feature-your-feature-name
  ```
- Open a **Pull Request (PR)** on GitHub and describe your changes.

---

## Contribution Guidelines

- Follow the **Rust coding conventions**.
- Write meaningful **commit messages**.
- **Document** any new functionality in the README if necessary.
- Ensure your changes don’t break existing functionality.
- Be **respectful and constructive** in discussions.

---

## Issues and Feature Requests
- Found a bug? Open an **Issue** [here](https://gitlab.rlp.net/mathew.siby/finance-manager-app-using-rust/issues).
- Have an idea? Feel free to suggest it in the **Discussions** section!

---

## Thank You!
We appreciate your contribution. Together, let's build an amazing finance management app! 

