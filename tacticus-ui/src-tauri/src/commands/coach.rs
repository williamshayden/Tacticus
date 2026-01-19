use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize)]
pub struct CoachMessage {
    pub role: String,  // "gurgeh" or "user"
    pub content: String,
    pub timestamp: i64,
    pub actions: Vec<CoachAction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoachAction {
    pub action_type: String,
    pub label: String,
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoachResponse {
    pub message: CoachMessage,
    pub board_fen: Option<String>,
    pub highlights: Vec<String>,
    pub arrows: Vec<(String, String)>,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ChatMessage,
}

const GURGEH_SYSTEM_PROMPT: &str = r#"You are Gurgeh, an AI chess coach named after the legendary game player from Iain M. Banks' Culture series. You are wise, patient, and deeply knowledgeable about chess.

Your personality:
- Speak with quiet confidence and wisdom
- Use clear, concise explanations
- Reference chess concepts precisely
- Be encouraging but honest about mistakes
- Occasionally make subtle references to game theory or strategy

Your capabilities:
- Explain chess concepts (forks, pins, skewers, tactics, strategy)
- Analyze positions and suggest moves
- Review games and find improvements
- Create custom exercises
- Teach openings, endgames, and middlegame strategy

Guidelines:
- Keep responses focused and practical
- Use algebraic notation for moves (e.g., e4, Nf3, O-O)
- When explaining concepts, give concrete examples
- Adapt your explanations to the user's level
- Never use emojis in your responses

You are helping a chess student improve their game from beginner to advanced."#;

#[tauri::command]
pub fn get_coach_greeting(user_name: String, current_elo: i32, exercises_completed: i32) -> CoachResponse {
    let greeting = if exercises_completed == 0 {
        format!(
            "Welcome to Tacticus, {}. I'm Gurgeh, your chess coach - named after the legendary \
             game player from the Culture.\n\n\
             I see you're starting at {} ELO. Let's begin with some fundamentals and discover \
             where your strengths lie. Together, we'll master this ancient game.",
            user_name, current_elo
        )
    } else {
        format!(
            "Welcome back, {}. You've completed {} exercises so far. \
             Your current rating is {}. Ready to continue your training?",
            user_name, exercises_completed, current_elo
        )
    };
    
    CoachResponse {
        message: CoachMessage {
            role: "gurgeh".to_string(),
            content: greeting,
            timestamp: chrono::Utc::now().timestamp(),
            actions: vec![
                CoachAction {
                    action_type: "start_training".to_string(),
                    label: "Start Training".to_string(),
                    data: "".to_string(),
                },
                CoachAction {
                    action_type: "play_game".to_string(),
                    label: "Play a Game".to_string(),
                    data: "".to_string(),
                },
            ],
        },
        board_fen: None,
        highlights: vec![],
        arrows: vec![],
    }
}

#[tauri::command]
pub async fn chat_with_coach(
    message: String,
    context: Option<String>,
    api_key: Option<String>,
) -> Result<CoachResponse, String> {
    // Check for API key
    let key = api_key
        .or_else(|| std::env::var("OPENROUTER_API_KEY").ok())
        .or_else(|| {
            dotenv::dotenv().ok();
            std::env::var("OPENROUTER_API_KEY").ok()
        });
    
    let Some(api_key) = key else {
        return Ok(CoachResponse {
            message: CoachMessage {
                role: "gurgeh".to_string(),
                content: "I need an API key to respond. Please configure your OpenRouter API key in Settings to enable AI coaching.".to_string(),
                timestamp: chrono::Utc::now().timestamp(),
                actions: vec![
                    CoachAction {
                        action_type: "open_settings".to_string(),
                        label: "Open Settings".to_string(),
                        data: "".to_string(),
                    },
                ],
            },
            board_fen: None,
            highlights: vec![],
            arrows: vec![],
        });
    };
    
    // Build messages
    let mut messages = vec![
        ChatMessage {
            role: "system".to_string(),
            content: GURGEH_SYSTEM_PROMPT.to_string(),
        },
    ];
    
    if let Some(ctx) = context {
        messages.push(ChatMessage {
            role: "system".to_string(),
            content: format!("Current context: {}", ctx),
        });
    }
    
    messages.push(ChatMessage {
        role: "user".to_string(),
        content: message.clone(),
    });
    
    // Make API request
    let client = Client::new();
    let request = ChatRequest {
        model: "anthropic/claude-3-haiku".to_string(),
        messages,
        temperature: 0.7,
        max_tokens: 1000,
    };
    
    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .header("HTTP-Referer", "https://github.com/tacticus-chess")
        .header("X-Title", "Tacticus Chess Trainer")
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;
    
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("API error ({}): {}", status, error_text));
    }
    
    let chat_response: ChatResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;
    
    let response_content = chat_response
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .unwrap_or_else(|| "I apologize, but I couldn't generate a response. Please try again.".to_string());
    
    Ok(CoachResponse {
        message: CoachMessage {
            role: "gurgeh".to_string(),
            content: response_content,
            timestamp: chrono::Utc::now().timestamp(),
            actions: vec![],
        },
        board_fen: None,
        highlights: vec![],
        arrows: vec![],
    })
}

#[tauri::command]
pub async fn analyze_position_with_coach(
    fen: String,
    api_key: Option<String>,
) -> Result<CoachResponse, String> {
    let prompt = format!(
        "Analyze this chess position (FEN: {}).\n\n\
         Provide:\n\
         1. Who is better and why (material, position, king safety)\n\
         2. Key features of the position\n\
         3. Best plan for the side to move\n\
         4. Any tactical opportunities\n\n\
         Keep your analysis concise but thorough.",
        fen
    );
    
    chat_with_coach(prompt, Some(format!("Position FEN: {}", fen)), api_key).await
}

#[tauri::command]
pub fn get_position_feedback(
    fen: String,
    user_move: Option<String>,
    correct_move: Option<String>,
) -> CoachResponse {
    let content = if let (Some(user), Some(correct)) = (&user_move, &correct_move) {
        if user == correct {
            "Excellent move. You found the best continuation.".to_string()
        } else {
            format!(
                "Good attempt with {}. However, {} was stronger here. \
                 Let me explain why...",
                user, correct
            )
        }
    } else {
        "Let's analyze this position together.".to_string()
    };
    
    CoachResponse {
        message: CoachMessage {
            role: "gurgeh".to_string(),
            content,
            timestamp: chrono::Utc::now().timestamp(),
            actions: vec![],
        },
        board_fen: Some(fen),
        highlights: vec![],
        arrows: vec![],
    }
}

#[tauri::command]
pub fn check_api_key_configured() -> bool {
    dotenv::dotenv().ok();
    std::env::var("OPENROUTER_API_KEY").is_ok()
}
