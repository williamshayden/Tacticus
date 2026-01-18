use serde::{Deserialize, Serialize};
use serde_json::Value;
use anyhow::Result;

/// Tool definition for LLM function calling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: ToolParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameters {
    #[serde(rename = "type")]
    pub param_type: String,
    pub properties: Value,
    pub required: Vec<String>,
}

/// Available chess data tools
pub struct ChessTools;

impl ChessTools {
    /// Get all available tools for the LLM
    pub fn get_all_tools() -> Vec<Tool> {
        vec![
            Self::get_recent_games_tool(),
            Self::get_player_stats_tool(),
            Self::get_weakness_history_tool(),
            Self::search_games_by_opening_tool(),
            Self::get_games_with_mistakes_tool(),
            Self::get_training_progress_tool(),
            Self::get_improvement_trend_tool(),
        ]
    }

    fn get_recent_games_tool() -> Tool {
        Tool {
            name: "get_recent_games".to_string(),
            description: "Get the player's N most recent games with move analysis, results, and key statistics".to_string(),
            parameters: ToolParameters {
                param_type: "object".to_string(),
                properties: serde_json::json!({
                    "count": {
                        "type": "integer",
                        "description": "Number of recent games to retrieve (max 20)",
                        "minimum": 1,
                        "maximum": 20
                    }
                }),
                required: vec!["count".to_string()],
            },
        }
    }

    fn get_player_stats_tool() -> Tool {
        Tool {
            name: "get_player_stats".to_string(),
            description: "Get current player statistics including rating, win rate, play style, and overall performance metrics".to_string(),
            parameters: ToolParameters {
                param_type: "object".to_string(),
                properties: serde_json::json!({}),
                required: vec![],
            },
        }
    }

    fn get_weakness_history_tool() -> Tool {
        Tool {
            name: "get_weakness_history".to_string(),
            description: "Get historical tracking of player weaknesses over time to see patterns and improvement".to_string(),
            parameters: ToolParameters {
                param_type: "object".to_string(),
                properties: serde_json::json!({
                    "time_period_days": {
                        "type": "integer",
                        "description": "Number of days to look back (default 30)",
                        "minimum": 1,
                        "maximum": 365
                    }
                }),
                required: vec![],
            },
        }
    }

    fn search_games_by_opening_tool() -> Tool {
        Tool {
            name: "search_games_by_opening".to_string(),
            description: "Find games where the player used a specific opening or opening family".to_string(),
            parameters: ToolParameters {
                param_type: "object".to_string(),
                properties: serde_json::json!({
                    "opening_name": {
                        "type": "string",
                        "description": "Name of the opening (e.g., 'Sicilian Defense', 'King's Gambit', 'e4')"
                    }
                }),
                required: vec!["opening_name".to_string()],
            },
        }
    }

    fn get_games_with_mistakes_tool() -> Tool {
        Tool {
            name: "get_games_with_mistakes".to_string(),
            description: "Get games where the player made blunders or mistakes, useful for identifying recurring tactical issues".to_string(),
            parameters: ToolParameters {
                param_type: "object".to_string(),
                properties: serde_json::json!({
                    "quality_threshold": {
                        "type": "string",
                        "description": "Minimum severity to retrieve: 'blunder', 'mistake', or 'inaccuracy'",
                        "enum": ["blunder", "mistake", "inaccuracy"]
                    },
                    "count": {
                        "type": "integer",
                        "description": "Number of games to retrieve (max 10)",
                        "minimum": 1,
                        "maximum": 10
                    }
                }),
                required: vec!["quality_threshold".to_string()],
            },
        }
    }

    fn get_training_progress_tool() -> Tool {
        Tool {
            name: "get_training_progress".to_string(),
            description: "Get training exercise completion history, success rates, and areas of improvement".to_string(),
            parameters: ToolParameters {
                param_type: "object".to_string(),
                properties: serde_json::json!({
                    "exercise_type": {
                        "type": "string",
                        "description": "Filter by exercise type (optional): 'tactics', 'endgame', 'opening', etc."
                    }
                }),
                required: vec![],
            },
        }
    }

    fn get_improvement_trend_tool() -> Tool {
        Tool {
            name: "get_improvement_trend".to_string(),
            description: "Get rating changes and performance trends over time to track improvement".to_string(),
            parameters: ToolParameters {
                param_type: "object".to_string(),
                properties: serde_json::json!({
                    "time_period_days": {
                        "type": "integer",
                        "description": "Number of days to analyze (default 90)",
                        "minimum": 7,
                        "maximum": 365
                    }
                }),
                required: vec![],
            },
        }
    }
}

/// Tool execution results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub tool_name: String,
    pub success: bool,
    pub data: Value,
    pub error: Option<String>,
}

impl ToolResult {
    pub fn success(tool_name: impl Into<String>, data: Value) -> Self {
        Self {
            tool_name: tool_name.into(),
            success: true,
            data,
            error: None,
        }
    }

    pub fn error(tool_name: impl Into<String>, error: impl Into<String>) -> Self {
        Self {
            tool_name: tool_name.into(),
            success: false,
            data: Value::Null,
            error: Some(error.into()),
        }
    }

    pub fn to_string_result(&self) -> String {
        if self.success {
            serde_json::to_string_pretty(&self.data).unwrap_or_else(|_| "{}".to_string())
        } else {
            format!("Error: {}", self.error.as_ref().unwrap_or(&"Unknown error".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tools_serialization() {
        let tools = ChessTools::get_all_tools();
        assert!(!tools.is_empty());

        for tool in tools {
            let json = serde_json::to_string(&tool).unwrap();
            assert!(json.contains(&tool.name));
        }
    }
}
