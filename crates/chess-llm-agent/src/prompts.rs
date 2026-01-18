use chess_ai::PlayStyle;

pub struct ChessCoachPrompts;

impl ChessCoachPrompts {
    pub fn system_prompt() -> String {
        r#"You are an expert chess coach with deep knowledge of chess strategy, tactics, and psychology. Your goal is to help players improve their chess skills through personalized guidance, encouragement, and constructive feedback.

Your coaching philosophy:
- Be encouraging and supportive, celebrating strengths while addressing weaknesses
- Provide specific, actionable advice that players can immediately apply
- Explain complex concepts in simple, relatable terms
- Adapt your teaching style to the player's level and learning pace
- Focus on understanding WHY moves are good or bad, not just WHAT to play
- Help players develop a growth mindset about chess

When analyzing games:
- Point out both tactical and strategic elements
- Explain the thought process behind strong moves
- Help players recognize patterns and themes
- Suggest specific areas to study based on their mistakes
- Provide concrete training recommendations

Remember: Your goal is to make chess learning enjoyable and to build the player's confidence while genuinely improving their skills."#.to_string()
    }

    pub fn game_analysis_prompt(
        pgn: &str,
        player_color: &str,
        move_quality_summary: &str,
        weaknesses: &[String],
    ) -> String {
        format!(
            r#"Analyze this chess game where the player played as {player_color}.

Game PGN:
{pgn}

Move Quality Summary:
{move_quality_summary}

Identified Technical Weaknesses:
{weaknesses}

Please provide a comprehensive yet friendly analysis that includes:

1. **Overall Performance**: Give an encouraging assessment of how the player performed
2. **Key Moments**: Highlight 2-3 critical positions where the game turned
3. **Strengths**: What did the player do well? (Be specific and encouraging)
4. **Areas to Improve**: Focus on 2-3 main areas (don't overwhelm them)
5. **Training Recommendations**: Suggest specific types of exercises or study material
6. **Motivational Closing**: End with an encouraging message about their potential

Keep the tone conversational, supportive, and educational. Use chess notation when referring to specific moves, but explain complex ideas in accessible language."#,
            player_color = player_color,
            pgn = pgn,
            move_quality_summary = move_quality_summary,
            weaknesses = weaknesses.join("\n")
        )
    }

    pub fn playstyle_analysis_prompt(
        style: &PlayStyle,
        aggression: f32,
        tactical: f32,
        positional: f32,
        games_analyzed: usize,
    ) -> String {
        format!(
            r#"Based on analyzing {games_analyzed} games, here are the player's style characteristics:

Primary Playing Style: {style:?}
Aggression Score: {aggression:.1}%
Tactical Awareness: {tactical:.1}%
Positional Understanding: {positional:.1}%

Please provide:

1. **Playing Style Description**: Describe their chess personality in an engaging way
2. **Strengths of This Style**: What advantages does their natural style give them?
3. **Watch Out For**: What pitfalls should they be aware of with this style?
4. **Famous Players**: Mention 1-2 famous chess players with a similar style
5. **Development Path**: How can they evolve this style to the next level?

Make it personal and motivating - help them understand and embrace their unique chess identity!"#,
            style = style,
            aggression = aggression * 100.0,
            tactical = tactical * 100.0,
            positional = positional * 100.0,
            games_analyzed = games_analyzed
        )
    }

    pub fn exercise_introduction_prompt(
        exercise_type: &str,
        difficulty: &str,
        player_weakness: &str,
    ) -> String {
        format!(
            r#"You're about to present a {difficulty} {exercise_type} exercise to address: {player_weakness}

Generate a motivating introduction that:
1. Explains WHY this exercise is important for their development
2. Connects it to their recent games (if relevant)
3. Sets a positive, achievable mindset
4. Gives a brief hint about what to look for

Keep it to 2-3 sentences, friendly and encouraging."#,
            difficulty = difficulty,
            exercise_type = exercise_type,
            player_weakness = player_weakness
        )
    }

    pub fn exercise_hint_prompt(
        position_fen: &str,
        exercise_goal: &str,
        hint_level: u32,
    ) -> String {
        let hint_guidance = match hint_level {
            1 => "Give a very subtle hint - just point them in the right direction without revealing the answer",
            2 => "Give a moderate hint - be more specific about what to look for",
            3 => "Give a strong hint - guide them very close to the solution",
            _ => "Provide the solution with a detailed explanation",
        };

        format!(
            r#"Position (FEN): {position_fen}
Exercise Goal: {exercise_goal}
Hint Level: {hint_level}

{hint_guidance}

Keep the hint encouraging and educational. If it's not hint level 4, don't give away the full answer!"#,
            position_fen = position_fen,
            exercise_goal = exercise_goal,
            hint_level = hint_level,
            hint_guidance = hint_guidance
        )
    }

    pub fn personalized_training_plan_prompt(
        current_rating: u32,
        play_style: &PlayStyle,
        top_weaknesses: &[String],
        recent_progress: &str,
    ) -> String {
        format!(
            r#"Create a personalized 2-week chess training plan for a player with:

Current Rating: {current_rating}
Playing Style: {play_style:?}
Main Weaknesses: {weaknesses}
Recent Progress: {recent_progress}

Generate a structured training plan that includes:

1. **Daily Time Commitment**: Realistic time allocation (30-60 min/day)
2. **Weekly Focus Areas**: What to prioritize each week
3. **Specific Exercises**:
   - Tactical puzzles (difficulty and quantity)
   - Opening study (which openings for their style)
   - Endgame practice (specific positions)
   - Game analysis (what to focus on)
4. **Progress Milestones**: How they'll know they're improving
5. **Motivational Tips**: How to stay consistent and enjoy the process

Make it actionable, achievable, and tailored to their unique situation!"#,
            current_rating = current_rating,
            play_style = play_style,
            weaknesses = top_weaknesses.join("\n"),
            recent_progress = recent_progress
        )
    }

    pub fn encouragement_prompt(context: &str) -> String {
        format!(
            r#"The player just: {context}

Provide a brief, genuine encouragement message (1-2 sentences) that:
- Acknowledges their effort or achievement
- Motivates them to keep going
- Is specific to their situation (not generic)

Be warm, authentic, and brief!"#,
            context = context
        )
    }
}
