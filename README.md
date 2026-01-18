# Tacticus - AI Chess Coach

An intelligent chess training application with a **real LLM coach** that learns your playing style and provides personalized, conversational coaching to help you improve.

## 🌟 What Makes Tacticus Different?

### Real LLM-Powered Coaching
Unlike basic chess trainers, Tacticus uses a **sophisticated LLM agent** (via OpenRouter) that:
- Analyzes your games and provides **natural language feedback**
- Has **conversational memory** - remembers your goals, weaknesses, and progress
- Uses **tool-calling** to intelligently query your game database for relevant insights
- Provides **personalized training plans** tailored to your unique playing style
- Offers **encouragement and motivation** like a real coach

### Desktop Native GUI
- Built with **egui** for a smooth, native desktop experience
- No web browser required - true desktop performance
- Beautiful, intuitive interface for playing, training, and analyzing

### Intelligent Tool-Calling System
Instead of dumping all data into the context window or using complex RAG systems, Tacticus uses **smart tool-calling**:
- LLM coach decides what data it needs ("show me games where I blundered")
- Queries structured database with precision
- Gets exact data, not semantic similarity
- More efficient, faster, and cost-effective

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed explanation of why this approach is superior for chess training.

## 🎯 Features

### LLM Coach Capabilities
- **Game Analysis**: Detailed, conversational breakdown of your games
- **Playstyle Insights**: Understands if you're aggressive, tactical, positional, or solid
- **Personalized Training**: Creates custom exercises based on your specific weaknesses
- **Progress Tracking**: Remembers your improvement journey and adapts coaching
- **Natural Conversation**: Chat freely with your coach about chess concepts
- **Motivational Support**: Provides encouragement and celebrates your improvements

### Core Functionality
1. **Play**: Interactive chess board to play against the engine
2. **Train**: 5-10 personalized exercises with LLM coach hints
3. **Analyze**: Deep game analysis with natural language feedback
4. **Profile**: Track rating, style, stats, and improvement trends

## Architecture

The application is built as a modular Rust workspace:

```
Tacticus/
├── crates/
│   ├── chess-core/        # Core chess logic, board representation
│   ├── chess-engine/      # Move evaluation and game analysis
│   ├── chess-trainer/     # Exercise generation and training sessions
│   ├── chess-ai/          # Traditional ML playstyle analysis
│   ├── chess-llm-agent/   # LLM coach with tool-calling
│   ├── chess-storage/     # SQLite persistence + tool executors
│   └── chess-gui/         # Desktop app with egui
├── .env                   # OpenRouter API key (gitignored)
├── ARCHITECTURE.md        # Detailed tool-calling architecture
└── Cargo.toml             # Workspace configuration
```

**Key Innovation**: The `chess-llm-agent` crate implements a tool-calling system where the LLM coach can query your chess database with precision instead of using RAG or context stuffing.

## Installation

### Prerequisites
- **Rust 1.75+** (for latest egui features)
- **OpenRouter API Key** (get free credits at [openrouter.ai](https://openrouter.ai))
- **SQLite 3**

### Setup

```bash
# Clone the repository
git clone <repository-url>
cd Tacticus

# Set up your OpenRouter API key
echo "OPENROUTER_API_KEY=sk-or-v1-your-key-here" > .env
echo "OPENROUTER_BASE_URL=https://openrouter.ai/api/v1" >> .env

# Build and run the desktop app
cargo run --release

# Or build for distribution
cargo build --release
# The binary will be at target/release/tacticus
```

**Important**: Never commit your `.env` file! It's in `.gitignore` to protect your API key.

## Usage

Run the native desktop app:
```bash
cargo run
```

The GUI provides five main views:

- 🏠 **Home**: Chat with your AI coach - ask questions, get insights, discuss strategy
- ♟️ **Play**: Play games on an interactive chess board
- 📚 **Train**: Complete 5-10 personalized exercises tailored to your weaknesses
- 🔍 **Analyze**: Review your games with AI-powered move-by-move analysis
- 👤 **Profile**: Track your rating, progress, style, and improvement trends

### How It Works

The AI coach uses sophisticated tool-calling to analyze your games:

1. **Play games** - The coach observes your moves without assistance
2. **Get analysis** - LLM coach queries your game database for specific insights
3. **Receive training** - Personalized exercises based on identified weaknesses
4. **Track progress** - Monitor your improvement over time with detailed stats

## Training Workflow

The application implements a complete adaptive learning cycle:

```
1. Play Game → 2. AI Analysis → 3. Identify Weaknesses → 4. Generate Exercises
     ↑                                                              ↓
     └──────────────────── 5. Update Profile ←──────────────────────
```

### Example Session

1. **Launch Tacticus** - `cargo run`

2. **Navigate to Play** - Play a game on the interactive board

3. **Get AI Analysis** - After the game, the LLM coach analyzes your moves:
   - "I notice you favor tactical play - great for creating threats!"
   - "However, you made early inaccuracies in the opening phase"
   - "You lost about 150 centipawns in moves 4-7"

4. **Start Training** - Navigate to the Train view for personalized exercises:
   - Each exercise targets your specific weaknesses
   - Get hints from your AI coach when stuck
   - Receive detailed explanations after solving
   - Progress automatically tracked

5. **Monitor Progress** - Check the Profile view to see:
   - Rating trends over time
   - Playing style analysis
   - Strengths and improvement areas
   - Training completion statistics

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
- **GUI Framework**: `egui` for native desktop UI
- **LLM Integration**: OpenRouter API with tool-calling
- **Database**: SQLite with `sqlx` for async operations
- **Async Runtime**: `tokio`
- **Serialization**: `serde` and `serde_json`

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
- [ ] Multiplayer support with peer-to-peer connections
- [ ] Import/export PGN files
- [ ] Tournament mode
- [ ] Puzzle rush feature
- [ ] Video lessons integration
- [ ] Chess board themes and customization
- [ ] Voice interaction with AI coach

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
cargo run --release
```
