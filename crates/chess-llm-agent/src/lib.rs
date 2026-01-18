pub mod openrouter;
pub mod chess_coach;
pub mod prompts;
pub mod conversation;
pub mod tools;

pub use openrouter::{OpenRouterClient, ChatMessage, ChatRequest, ChatResponse};
pub use chess_coach::{ChessCoach, CoachingSession, CoachFeedback, GameSummary, PlayerStats, SessionContext};
pub use conversation::{ConversationManager, Message};
pub use tools::{ChessTools, Tool, ToolResult};
