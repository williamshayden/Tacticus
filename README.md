# Tacticus - Adaptive Chess Training AI

An intelligent chess training application built in Rust that learns your playing style and provides personalized exercises to improve your chess skills.

## Features

### 🎯 Adaptive Learning
- **AI-Powered Analysis**: Advanced algorithms analyze your games to identify strengths and weaknesses
- **Personalized Training**: Generates custom exercise sets (5-10 exercises) based on your specific needs
- **Play Style Recognition**: Identifies whether you're aggressive, tactical, positional, solid, or balanced
- **Progressive Difficulty**: Automatically adjusts challenge level based on your performance

### 🎮 Core Functionality
1. **Training Sessions**: Complete personalized exercise sets with hints and explanations
2. **Practice Games**: Play against the computer with real-time analysis
3. **Game Analysis**: Detailed move-by-move breakdown of your games
4. **Progress Tracking**: Persistent storage of your games, exercises, and improvement over time

### 🧠 Learning Agent Capabilities
- Analyzes your move quality (Brilliant, Great, Good, Inaccuracy, Mistake, Blunder)
- Identifies tactical patterns and strategic themes
- Tracks centipawn loss to measure accuracy
- Provides actionable feedback and focus areas
- Adapts training based on game phase weaknesses (opening, middlegame, endgame)

## Architecture

The application is built as a modular Rust workspace with six crates:

```
chess-trainer-app/
├── crates/
│   ├── chess-core/       # Core chess logic, board representation
│   ├── chess-engine/     # Move evaluation and game analysis
│   ├── chess-trainer/    # Exercise generation and training sessions
│   ├── chess-ai/         # AI agent for playstyle analysis
│   ├── chess-storage/    # SQLite persistence layer
│   └── chess-cli/        # Command-line interface
└── Cargo.toml            # Workspace configuration
```

## Installation

### Prerequisites
- Rust 1.70 or higher
- SQLite 3

### Build from Source

```bash
# Clone the repository
git clone <repository-url>
cd Tacticus

# Build the project
cargo build --release

# The binary will be at target/release/chess-cli
```

## Usage

### Initialize the Database

First time setup:
```bash
chess-cli init
```

### Start a Training Session

```bash
chess-cli train
```

This will:
1. Load or create your player profile
2. Analyze your weaknesses from previous games
3. Generate 5-10 personalized exercises
4. Guide you through each exercise with hints
5. Update your profile based on performance

### Play a Practice Game

```bash
# Play as white (default)
chess-cli play

# Play as black
chess-cli play --color black
```

After the game:
- Your moves are analyzed for quality
- The AI identifies your strengths and weaknesses
- You receive personalized recommendations
- The game is saved to your history

### View Your Profile

```bash
chess-cli profile
```

Displays:
- Skill level and estimated rating
- Play style characteristics
- Games played and exercises completed
- Current strengths and weaknesses
- Style breakdown (aggression, tactical, positional, etc.)

### Analyze Your Games

```bash
chess-cli analyze
```

Provides:
- Move-by-move analysis of your most recent game
- Quality annotations for each move
- Best move alternatives
- Centipawn loss calculations
- Statistical summary

## Training Workflow

The application implements a complete adaptive learning cycle:

```
1. Play Game → 2. AI Analysis → 3. Identify Weaknesses → 4. Generate Exercises
     ↑                                                              ↓
     └──────────────────── 5. Update Profile ←──────────────────────
```

### Example Session

```bash
# Initialize database (first time only)
$ chess-cli init

# Play a game to establish baseline
$ chess-cli play

# After the game, you'll see analysis:
# "Your play style is: Tactical
#  Strengths: Strong move selection, High accuracy
#  Areas for improvement: Weak opening play - study opening principles
#
#  Recommendations:
#  • Complete 7 exercises
#  • Focus on: Opening Principles"

# Start personalized training
$ chess-cli train

# Complete exercises tailored to your weaknesses
# Each exercise provides:
# - Clear objective
# - Position to analyze
# - Hints if needed
# - Detailed explanation

# Check your progress
$ chess-cli profile
```

## Exercise Types

The system includes multiple exercise categories:

- **Tactics**: Forks, pins, skewers, discovered attacks
- **Opening**: Control center, development, castle early
- **Endgame**: King and pawn, rook endgames, pawn promotion
- **Positional**: Pawn structure, piece activity, weak squares
- **Strategy**: Long-term planning, positional advantages
- **Calculation**: Visualizing variations, calculating deeply

## Technologies

- **Language**: Rust 2021 Edition
- **Chess Library**: `chess` crate for board representation and move generation
- **Database**: SQLite with `sqlx` for async operations
- **CLI Framework**: `clap` for command-line parsing
- **UI**: `colored` for terminal formatting
- **Async Runtime**: `tokio`

## Scalability

The application is designed to scale infinitely:

- **Modular Architecture**: Easy to add new exercise types, strategies, and analysis methods
- **Extensible Storage**: SQLite for local use, easily adaptable to PostgreSQL for multi-user
- **Stateless Analysis**: Each game analysis is independent and can be parallelized
- **Efficient Caching**: Move evaluations can be cached for faster analysis
- **Low Memory Footprint**: Streaming analysis for large game collections

## Future Enhancements

Potential improvements:
- [ ] Integration with Stockfish for stronger engine analysis
- [ ] Opening book and repertoire training
- [ ] Spaced repetition for exercise review
- [ ] Multiplayer support
- [ ] Web interface
- [ ] Import/export PGN files
- [ ] Tournament mode
- [ ] Puzzle rush feature
- [ ] Video lessons integration

## Contributing

Contributions are welcome! Areas for contribution:
- Additional exercise content
- New tactical patterns
- Enhanced evaluation functions
- UI/UX improvements
- Documentation
- Tests

## License

MIT License - see LICENSE file for details

## Acknowledgments

- Chess logic powered by the `chess` crate
- Inspired by modern chess training platforms like Chess.com and Lichess
- Built with Rust for performance and reliability

---

**Start your chess improvement journey today!**

```bash
cargo build --release
./target/release/chess-cli init
./target/release/chess-cli play
```
