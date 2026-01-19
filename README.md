# Tacticus - AI Chess Coach

An intelligent chess training application with **Gurgeh**, a real LLM coach (named after the legendary game player from Iain M. Banks' Culture series) that provides personalized, conversational coaching to help you improve.

## Important: API Key Required

**Tacticus requires an OpenRouter API key to enable AI coaching features.**

Get your free API key at [openrouter.ai](https://openrouter.ai) and configure it in the app's Settings.

## What Makes Tacticus Different?

### Real LLM-Powered Coaching
Unlike basic chess trainers, Tacticus uses a **sophisticated LLM agent** (via OpenRouter) that:
- Analyzes your games and provides **natural language feedback**
- Has **conversational memory** - remembers your goals, weaknesses, and progress
- Uses **tool-calling** to intelligently query your game database for relevant insights
- Provides **personalized training plans** tailored to your unique playing style
- Offers **encouragement and motivation** like a real coach

### Desktop Native App
- Built with **Tauri 2.0 + React** for a smooth, native desktop experience
- Windows XP-inspired retro "webcore" aesthetic
- Beautiful, intuitive interface for playing, training, and analyzing

### Intelligent Tool-Calling System
Instead of dumping all data into the context window or using complex RAG systems, Tacticus uses **smart tool-calling**:
- LLM coach decides what data it needs ("show me games where I blundered")
- Queries structured database with precision
- Gets exact data, not semantic similarity
- More efficient, faster, and cost-effective

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed explanation of why this approach is superior for chess training.

## Features

### Gurgeh (AI Coach) Capabilities
- **Game Analysis**: Detailed, conversational breakdown of your games
- **Playstyle Insights**: Understands if you're aggressive, tactical, positional, or solid
- **Personalized Training**: Creates custom exercises based on your specific weaknesses
- **Progress Tracking**: Remembers your improvement journey and adapts coaching
- **Natural Conversation**: Chat freely with your coach about chess concepts
- **Concept Explanations**: Click any chess term for instant explanations with examples

### Core Functionality
1. **Train**: 10 personalized exercises with adaptive difficulty and coach guidance
2. **Play**: Challenge an ELO-matched engine with calibration games
3. **Analyze**: Deep game analysis with AI-powered position evaluation
4. **Learn**: Browse comprehensive chess concept library

## Architecture

The application is built as a modular Rust workspace with a React frontend:

```
Tacticus/
├── crates/
│   ├── chess-core/        # Core chess logic, board representation
│   ├── chess-engine/      # Move evaluation and game analysis
│   ├── chess-trainer/     # Exercise generation and training sessions
│   ├── chess-ai/          # Traditional ML playstyle analysis
│   └── chess-llm-agent/   # LLM coach with tool-calling
├── tacticus-ui/           # Tauri + React frontend
│   ├── src/               # React components and stores
│   └── src-tauri/         # Tauri Rust backend
├── .env                   # OpenRouter API key (gitignored)
├── ARCHITECTURE.md        # Detailed tool-calling architecture
└── Cargo.toml             # Workspace configuration
```

**Key Innovation**: The `chess-llm-agent` crate implements a tool-calling system where the LLM coach can query your chess database with precision.

## Installation

### Prerequisites
- **Node.js 18+** (for React frontend)
- **Rust 1.75+** (for Tauri backend)
- **OpenRouter API Key** (get free credits at [openrouter.ai](https://openrouter.ai))

### Quick Start

```bash
# Clone the repository
git clone <repository-url>
cd Tacticus

# Install frontend dependencies
cd tacticus-ui
npm install

# Create environment file with your API key
echo "OPENROUTER_API_KEY=sk-or-v1-your-key-here" > .env

# Run in development mode
npm run tauri dev

# Or build for distribution
npm run tauri build
# The app will be at target/release/bundle/
```

### Configuring Your API Key

1. Get your API key from [openrouter.ai](https://openrouter.ai)
2. Either:
   - Create a `.env` file with `OPENROUTER_API_KEY=your-key-here`
   - Or configure it in the app via Settings (click the [=] button in the taskbar)

**Important**: Never commit your `.env` file! It's in `.gitignore` to protect your API key.

## Usage

### First Launch

1. **Start the app** - The onboarding wizard will greet you
2. **Enter your name** - Gurgeh will address you personally
3. **Select your skill level** - Beginner, Intermediate, Advanced, or enter a custom ELO
4. **Configure API key** - Add your OpenRouter key in Settings

### Main Views

- **Hub**: Dashboard with stats, quick access to all features, and Gurgeh's suggestions
- **Train**: 10 adaptive exercises targeting your weaknesses with real-time feedback
- **Play**: Challenge the ELO-matched engine with various time controls
- **Analyze**: Input any FEN position and get AI-powered analysis
- **Learn**: Browse the concept library (forks, pins, endgames, etc.)

### Training Workflow

The application implements a complete adaptive learning cycle:

```
1. Training Session (10 exercises)
        ↓
2. Calibration Game
        ↓
3. ELO Update & Analysis
        ↓
4. Generate Next Session Based on Performance
        ↓
   Loop back to 1
```

## Exercise Types

- **Tactics**: Forks, pins, skewers, discovered attacks, back rank mates
- **Pattern Recognition**: Find the winning move quickly
- **Calculation Ladder**: Multi-move sequences
- **Guided Discovery**: Gurgeh walks you through the solution
- **Zero Assist**: Test yourself without hints
- **Endgame Drills**: King and pawn, rook endgames, basic checkmates

## Technologies

- **Frontend**: React 18 + TypeScript + Vite
- **Desktop Framework**: Tauri 2.0
- **Backend**: Rust 2021 Edition
- **Chess Library**: `chess` crate for board representation and move generation
- **LLM Integration**: OpenRouter API (compatible with OpenAI API format)
- **State Management**: Zustand
- **Styling**: Custom CSS with XP-inspired theme

## Building from Source

```bash
# Development
cd tacticus-ui
npm run tauri dev

# Production build
npm run tauri build

# Run tests
npm test                    # Frontend tests
cd src-tauri && cargo test  # Rust tests
```

## Future Enhancements

- [ ] Integration with Stockfish for stronger engine analysis
- [ ] Opening book and repertoire training
- [ ] Spaced repetition for exercise review
- [ ] Import/export PGN files
- [ ] Puzzle rush feature
- [ ] Chess board themes and customization
- [ ] Voice interaction with Gurgeh

## Contributing

We welcome contributions! Please read our **[Contributing Guide](CONTRIBUTING.md)** before submitting a pull request.

### Quick Overview

1. **Fork** the repository
2. **Create a branch** (`feature/your-feature` or `fix/your-fix`)
3. **Make changes** following our code style
4. **Test** your changes (`npm run build` and `cargo test --workspace`)
5. **Submit a PR** - all PRs require maintainer approval

### Branch Protection

The `main` branch is protected:
- All changes must go through pull requests
- PRs require approval from code owners before merging
- Direct pushes to `main` are not allowed

### Areas for Contribution

- **Good First Issues**: Documentation, tests, new chess concepts
- **Intermediate**: New exercise types, UI enhancements
- **Advanced**: Engine improvements, LLM tool enhancements

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines, code style, and setup instructions.

## License

MIT License - see LICENSE file for details

## Acknowledgments

- Chess logic powered by the `chess` crate
- Inspired by modern chess training platforms like Chess.com and Lichess
- Named after Jernau Morat Gurgeh from Iain M. Banks' "The Player of Games"
- Built with Rust and React for performance and quality UI

---

**Start your chess improvement journey today!**

```bash
cd tacticus-ui
npm run tauri dev
```
