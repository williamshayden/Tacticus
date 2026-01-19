# Contributing to Tacticus

Thank you for your interest in contributing to Tacticus! This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for everyone.

## Getting Started

### Prerequisites

- **Node.js 18+** - For the React frontend
- **Rust 1.75+** - For the Tauri backend and chess crates
- **Git** - For version control

### Development Setup

1. **Fork the repository** on GitHub

2. **Clone your fork:**
   ```bash
   git clone https://github.com/YOUR_USERNAME/Tacticus.git
   cd Tacticus
   ```

3. **Install dependencies:**
   ```bash
   cd tacticus-ui
   npm install
   ```

4. **Run in development mode:**
   ```bash
   npm run tauri dev
   ```

### Project Structure

```
Tacticus/
├── crates/                    # Rust chess engine and logic
│   ├── chess-core/           # Core chess logic
│   ├── chess-engine/         # Move evaluation
│   ├── chess-trainer/        # Exercise generation
│   ├── chess-ai/             # Playstyle analysis
│   └── chess-llm-agent/      # LLM integration
├── tacticus-ui/              # Tauri + React frontend
│   ├── src/                  # React components
│   │   ├── components/       # UI components
│   │   ├── stores/           # Zustand state
│   │   └── styles/           # CSS
│   └── src-tauri/            # Tauri Rust backend
│       └── src/commands/     # API commands
└── docs/                     # Documentation
```

## How to Contribute

### Reporting Bugs

1. **Check existing issues** to avoid duplicates
2. **Create a new issue** with:
   - Clear, descriptive title
   - Steps to reproduce
   - Expected vs actual behavior
   - System information (OS, versions)
   - Screenshots if applicable

### Suggesting Features

1. **Check existing issues/discussions** for similar ideas
2. **Create a feature request** with:
   - Clear description of the feature
   - Use case / why it's valuable
   - Possible implementation approach

### Submitting Code

#### Branch Naming

Use descriptive branch names:
- `feature/add-puzzle-mode`
- `fix/pawn-promotion-dialog`
- `docs/update-readme`
- `refactor/cleanup-stores`

#### Commit Messages

Follow conventional commits:
```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting (no code change)
- `refactor`: Code restructuring
- `test`: Adding tests
- `chore`: Maintenance

Examples:
```
feat(training): add spaced repetition for exercises
fix(board): handle en passant capture correctly
docs(readme): add API key setup instructions
```

#### Pull Request Process

1. **Create a feature branch** from `main`
2. **Make your changes** following the code style
3. **Test your changes:**
   ```bash
   # Frontend
   cd tacticus-ui
   npm run build
   
   # Backend
   cargo test --workspace
   ```
4. **Push to your fork** and create a PR
5. **Fill out the PR template** with:
   - Description of changes
   - Related issue (if any)
   - Screenshots for UI changes
   - Testing done
6. **Wait for review** - maintainer approval required

#### Branch Protection

The `main` branch is protected with the following rules:

- **No direct pushes** - All changes must go through pull requests
- **Required reviews** - PRs require approval from code owners
- **Stale review dismissal** - Approvals are dismissed when new commits are pushed
- **Conversation resolution** - All review comments must be resolved

This ensures code quality and prevents accidental changes to the main branch.

### Code Style

#### TypeScript/React

- Use TypeScript for all new code
- Functional components with hooks
- Descriptive variable/function names
- No emojis in code or UI text (use ASCII like `[G]` `[!]`)

#### Rust

- Follow `rustfmt` defaults
- Use `clippy` for linting
- Descriptive error messages
- Document public APIs

#### CSS

- Use CSS variables from `xp-theme.css`
- Follow the XP aesthetic (gradients, bevels, shadows)
- Mobile-responsive when possible

## Areas for Contribution

### Good First Issues

- Adding new chess concepts to the library
- Writing tests (E2E or unit)
- Documentation improvements
- Accessibility improvements

### Intermediate

- New exercise types
- UI/UX enhancements
- Performance optimizations
- Additional language support

### Advanced

- Engine improvements
- LLM tool enhancements
- New training algorithms
- Platform-specific features

## Testing

### Frontend Tests

```bash
cd tacticus-ui
npm test                    # Unit tests
npx playwright test         # E2E tests
```

### Backend Tests

```bash
cargo test --workspace      # All Rust tests
cargo test -p chess-engine  # Specific crate
```

## Questions?

- **GitHub Issues** - For bugs and features
- **GitHub Discussions** - For questions and ideas

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

## For Maintainers

### Setting Up Branch Protection

To configure branch protection on GitHub:

1. Go to **Settings** → **Branches** in your repository

2. Click **Add branch protection rule**

3. Set **Branch name pattern** to `main`

4. Enable the following options:
   - [x] **Require a pull request before merging**
     - [x] Require approvals (set to 1)
     - [x] Require review from Code Owners
     - [x] Dismiss stale pull request approvals when new commits are pushed
   - [x] **Require conversation resolution before merging**
   - [x] **Do not allow bypassing the above settings** (optional, applies rules to admins too)

5. Click **Create** to save the rule

### CODEOWNERS

The `.github/CODEOWNERS` file specifies who must review PRs:

```
# All files require review from the owner
* @williamshayden
```

To add more reviewers, edit this file:
```
# Frontend changes
/tacticus-ui/ @williamshayden @frontend-reviewer

# Rust changes  
/crates/ @williamshayden @rust-reviewer
```

---

Thank you for helping make Tacticus better!
