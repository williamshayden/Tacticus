# Tacticus Architecture

## Overview

Tacticus is an AI-powered chess training application that uses LLM agents with tool-calling to provide personalized coaching. Instead of using RAG/vector databases, we use a simpler, more effective **tool-calling architecture** where the LLM agent queries structured chess data as needed.

## Why Tool-Calling Over RAG?

### Chess Data is Highly Structured
- Games have clear metadata (date, result, opening, rating change)
- Moves are sequential and have precise evaluations
- Queries are predictable: "recent games", "games with blunders", "improvement trends"

### Benefits of Tool-Calling
1. **Precision**: Get exact data, not semantic similarity
2. **Simplicity**: No embeddings, no vector math, just SQL queries
3. **Efficiency**: Only fetch data the LLM actually needs
4. **Maintainability**: Easy to add new tools as needed
5. **Cost-Effective**: Smaller context windows, faster responses

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Desktop GUI (egui)                       │
│  ┌────────────┐ ┌────────────┐ ┌────────────┐ ┌──────────┐│
│  │   Play     │ │   Train    │ │  Analyze   │ │ Profile  ││
│  │   Game     │ │ Exercises  │ │   Games    │ │  Stats   ││
│  └────────────┘ └────────────┘ └────────────┘ └──────────┘│
└─────────────────────────────────────────────────────────────┘
                           ↕
┌─────────────────────────────────────────────────────────────┐
│                    LLM Chess Coach Agent                     │
│  ┌────────────────────────────────────────────────────────┐ │
│  │  Coaching Session (Conversation Manager)               │ │
│  │  • System Prompt: Expert chess coach personality       │ │
│  │  • Context: Recent games, player stats, current focus │ │
│  │  • Tool Access: Can query chess database via tools    │ │
│  └────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                           ↕
┌─────────────────────────────────────────────────────────────┐
│                  Tool-Calling System                         │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  Available Tools:                                     │  │
│  │  • get_recent_games(n) → Last N games with analysis  │  │
│  │  • get_player_stats() → Rating, win rate, style      │  │
│  │  • get_weakness_history() → Tracked weaknesses       │  │
│  │  • search_games_by_opening(name) → Specific opening  │  │
│  │  • get_games_with_mistakes(quality) → Error games    │  │
│  │  • get_training_progress() → Exercise completion     │  │
│  │  • get_improvement_trend() → Rating over time        │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                           ↕
┌─────────────────────────────────────────────────────────────┐
│                 PostgreSQL Database                          │
│  ┌────────────┐ ┌────────────┐ ┌────────────┐ ┌──────────┐│
│  │  Games     │ │  Profiles  │ │ Exercises  │ │Sessions  ││
│  │  • Moves   │ │  • Rating  │ │ • Results  │ │• History ││
│  │  • Analysis│ │  • Style   │ │ • Progress │ │• Context ││
│  └────────────┘ └────────────┘ └────────────┘ └──────────┘│
└─────────────────────────────────────────────────────────────┘
```

## Data Flow Example

### User asks: "How can I improve my opening play?"

1. **GUI** → Sends message to **LLM Coach Agent**

2. **LLM Coach** thinks:
   - "I need to see their recent games to check opening performance"
   - **Calls tool**: `get_recent_games(10)`
   - **Gets back**: Structured game data with opening names, move quality

3. **LLM Coach** analyzes:
   - "I see they play the Sicilian Defense but make early inaccuracies"
   - **Calls tool**: `search_games_by_opening("Sicilian Defense")`
   - **Gets back**: All Sicilian games with detailed analysis

4. **LLM Coach** provides personalized response:
   ```
   Looking at your recent games, I notice you favor the Sicilian Defense
   (great choice for your aggressive style!), but you're struggling in
   moves 4-7 where theory is critical.

   Specifically, in your last 3 Sicilian games, you lost ~150 centipawns
   in the opening phase. Let's work on:

   1. The Najdorf variation - matches your tactical style
   2. Key pawn breaks and piece development
   3. Recognizing when to deviate from theory

   I've prepared 5 Sicilian Defense exercises focused on the early
   middlegame. Shall we start?
   ```

5. **GUI** → Displays response with exercises

## Tool Implementation

### Tool Definition
```rust
pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: ToolParameters, // JSON Schema
}
```

### Tool Execution
```rust
// LLM decides to call a tool
let tool_call = "get_recent_games";
let params = json!({"count": 5});

// Executor queries database
let games = game_repository.get_user_games(user_id, 5).await?;

// Format as structured data
let result = serde_json::to_value(games)?;

// Return to LLM
tool_result = ToolResult::success(tool_call, result);

// LLM uses this data in its response
```

## Crate Organization

### `chess-llm-agent`
- **OpenRouter Client**: HTTP client for LLM API calls
- **Chess Coach**: High-level coaching interface
- **Prompts**: Carefully crafted prompts for different scenarios
- **Conversation Manager**: Maintains chat history with context trimming
- **Tools**: Tool definitions and execution framework

### `chess-storage`
- PostgreSQL repositories for games, profiles, exercises
- Tool executors that translate tool calls → database queries
- Efficient data serialization for LLM consumption

### `chess-gui`
- Desktop app using egui (native Rust GUI)
- Views: Home, Play, Train, Analyze, Profile
- Async integration with LLM coach
- Real-time chess board visualization

## Key Design Decisions

### 1. No Vector Database
Chess data doesn't benefit from semantic search. We need precise queries like "games where I blundered in the opening" not "games similar to this position".

### 2. Tool-Calling > Context Stuffing
Instead of putting all game history in the context window:
- Tools fetch only what's needed
- Reduces token costs
- Faster responses
- Can handle unlimited history

### 3. Structured Data Format
Tools return JSON with clear structure:
```json
{
  "games": [
    {
      "id": 123,
      "date": "2026-01-18",
      "result": "win",
      "opening": "Sicilian Defense",
      "blunders": 1,
      "mistakes": 3,
      "average_centipawn_loss": 45
    }
  ]
}
```

### 4. Stateless Tool Execution
Each tool call is independent - makes it easy to:
- Cache results
- Parallelize queries
- Add new tools without breaking existing ones

## Future Enhancements

1. **Batch Tool Calls**: LLM can request multiple tools in one turn
2. **Tool Composition**: One tool can call others (e.g., weakness_trend calls get_games_with_mistakes over time)
3. **Smart Caching**: Cache frequently accessed data (player stats, recent games)
4. **Streaming Responses**: Show LLM's thinking process as it queries tools
5. **Tool Learning**: Track which tools are most useful, optimize availability

## Running the Application

```bash
# Set up environment
cp .env.example .env
# Add your OpenRouter API key to .env

# Build and run
cargo run --bin tacticus

# The GUI will launch with:
# - Chess board for playing games
# - AI coach chat interface
# - Training exercises view
# - Game analysis tools
# - Player profile and stats
```

## API Key Security

The OpenRouter API key is:
- Stored in `.env` (gitignored)
- Never hardcoded in source
- Loaded at runtime via `dotenv`
- Never sent to frontend (desktop app keeps it secure)

## Summary

Tacticus uses **tool-calling with structured data** instead of RAG because:
- Chess data is inherently structured
- Queries are predictable and precise
- Much simpler to implement and maintain
- More cost-effective and faster
- Easier to debug and extend

The LLM coach becomes a smart query orchestrator that knows what data to fetch and how to use it to provide personalized coaching.
