use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChessConcept {
    pub id: String,
    pub name: String,
    pub category: String,
    pub difficulty: String,
    pub short_description: String,
    pub full_explanation: String,
    pub example_fen: Option<String>,
    pub example_arrows: Vec<(String, String)>,
    pub example_highlights: Vec<String>,
    pub related_concepts: Vec<String>,
    pub practice_exercises: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptCategory {
    pub name: String,
    pub concepts: Vec<String>,
}

// Built-in concept library
fn get_concept_library() -> Vec<ChessConcept> {
    vec![
        // TACTICS
        ChessConcept {
            id: "fork".to_string(),
            name: "Fork".to_string(),
            category: "Tactics".to_string(),
            difficulty: "Beginner".to_string(),
            short_description: "A single piece attacks two or more enemy pieces simultaneously.".to_string(),
            full_explanation: "A fork is one of the most common tactical motifs in chess. It occurs when \
                one piece attacks two or more enemy pieces at the same time. The opponent can usually only \
                save one piece, resulting in material gain for the attacking side.\n\n\
                Knights are especially good at forking because of their unique L-shaped movement pattern, \
                which makes it hard to see the attack coming. A 'royal fork' attacks both the king and queen.".to_string(),
            example_fen: Some("r1bqkb1r/pppp1ppp/2n2n2/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR w KQkq - 4 4".to_string()),
            example_arrows: vec![("h5".to_string(), "f7".to_string()), ("h5".to_string(), "e5".to_string())],
            example_highlights: vec!["h5".to_string(), "f7".to_string()],
            related_concepts: vec!["knight_fork".to_string(), "double_attack".to_string()],
            practice_exercises: vec!["fork_1".to_string(), "fork_2".to_string()],
        },
        ChessConcept {
            id: "knight_fork".to_string(),
            name: "Knight Fork".to_string(),
            category: "Tactics".to_string(),
            difficulty: "Beginner".to_string(),
            short_description: "A knight attacks two or more pieces at once using its unique movement.".to_string(),
            full_explanation: "The knight fork is one of the most powerful tactical weapons in chess. \
                Because knights move in an L-shape and can jump over pieces, knight forks are often \
                difficult to see coming.\n\n\
                The most devastating knight fork is when it attacks the king and queen simultaneously, \
                forcing the king to move and allowing the knight to capture the queen.".to_string(),
            example_fen: Some("r1bqkb1r/pppp1ppp/2n5/4p3/2B1n3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 5".to_string()),
            example_arrows: vec![],
            example_highlights: vec!["e4".to_string()],
            related_concepts: vec!["fork".to_string(), "family_fork".to_string()],
            practice_exercises: vec!["knight_fork_1".to_string()],
        },
        ChessConcept {
            id: "pin".to_string(),
            name: "Pin".to_string(),
            category: "Tactics".to_string(),
            difficulty: "Beginner".to_string(),
            short_description: "A piece cannot move because it would expose a more valuable piece behind it.".to_string(),
            full_explanation: "A pin occurs when a piece cannot move (or shouldn't move) because doing so \
                would expose a more valuable piece behind it to capture.\n\n\
                There are two types of pins:\n\
                • Absolute pin: The piece behind is the king, making it illegal to move the pinned piece.\n\
                • Relative pin: The piece behind is valuable (like a queen), but moving is still legal.\n\n\
                Bishops and rooks are the typical pinning pieces because they attack in straight lines.".to_string(),
            example_fen: Some("r1bqkbnr/pppp1ppp/2n5/4p3/2B5/5N2/PPPP1PPP/RNBQK2R b KQkq - 3 3".to_string()),
            example_arrows: vec![("c4".to_string(), "f7".to_string())],
            example_highlights: vec!["c4".to_string(), "f7".to_string(), "e8".to_string()],
            related_concepts: vec!["skewer".to_string(), "absolute_pin".to_string(), "relative_pin".to_string()],
            practice_exercises: vec!["pin_1".to_string(), "pin_2".to_string()],
        },
        ChessConcept {
            id: "skewer".to_string(),
            name: "Skewer".to_string(),
            category: "Tactics".to_string(),
            difficulty: "Beginner".to_string(),
            short_description: "An attack on a valuable piece that, when it moves, exposes a piece behind it.".to_string(),
            full_explanation: "A skewer is like a reverse pin. Instead of a less valuable piece being in front, \
                the MORE valuable piece is in front. When it moves out of the way, the piece behind can be captured.\n\n\
                For example, if a bishop attacks a king with a rook behind it, the king must move, \
                and then the rook can be captured.".to_string(),
            example_fen: Some("4k3/8/8/8/8/8/4R3/4K3 w - - 0 1".to_string()),
            example_arrows: vec![("e2".to_string(), "e8".to_string())],
            example_highlights: vec!["e2".to_string(), "e8".to_string()],
            related_concepts: vec!["pin".to_string(), "x_ray".to_string()],
            practice_exercises: vec!["skewer_1".to_string()],
        },
        ChessConcept {
            id: "back_rank_mate".to_string(),
            name: "Back Rank Mate".to_string(),
            category: "Tactics".to_string(),
            difficulty: "Beginner".to_string(),
            short_description: "Checkmate delivered on the back rank when the king is trapped by its own pawns.".to_string(),
            full_explanation: "A back rank mate occurs when a rook or queen delivers checkmate on the opponent's \
                back rank (1st rank for White, 8th rank for Black). This typically happens when the king is \
                trapped behind its own pawns with no escape squares.\n\n\
                To prevent back rank mates, you can:\n\
                • Create 'luft' (breathing room) by moving a pawn\n\
                • Keep a piece defending the back rank\n\
                • Trade off the attacking pieces".to_string(),
            example_fen: Some("6k1/5ppp/8/8/8/8/8/R3K3 w - - 0 1".to_string()),
            example_arrows: vec![("a1".to_string(), "a8".to_string())],
            example_highlights: vec!["a1".to_string(), "g8".to_string()],
            related_concepts: vec!["checkmate_patterns".to_string(), "luft".to_string()],
            practice_exercises: vec!["back_rank_1".to_string()],
        },
        ChessConcept {
            id: "discovered_attack".to_string(),
            name: "Discovered Attack".to_string(),
            category: "Tactics".to_string(),
            difficulty: "Intermediate".to_string(),
            short_description: "Moving one piece reveals an attack from another piece behind it.".to_string(),
            full_explanation: "A discovered attack happens when you move a piece and uncover an attack \
                from another piece that was behind it. If the piece you move also creates a threat, \
                you're effectively attacking with two pieces at once.\n\n\
                A discovered CHECK is especially powerful - the opponent must deal with the check, \
                allowing your moved piece to capture or create other threats freely.".to_string(),
            example_fen: Some("r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4".to_string()),
            example_arrows: vec![],
            example_highlights: vec![],
            related_concepts: vec!["double_check".to_string(), "windmill".to_string()],
            practice_exercises: vec!["discovered_1".to_string()],
        },
        // STRATEGY
        ChessConcept {
            id: "piece_activity".to_string(),
            name: "Piece Activity".to_string(),
            category: "Strategy".to_string(),
            difficulty: "Beginner".to_string(),
            short_description: "Active pieces control more squares and create more threats.".to_string(),
            full_explanation: "Piece activity is one of the most important strategic concepts in chess. \
                An active piece is one that controls many squares, creates threats, and participates in the game.\n\n\
                Key principles:\n\
                • Develop pieces toward the center where they control more squares\n\
                • Avoid moving the same piece twice in the opening\n\
                • Connect your rooks by castling and developing all minor pieces\n\
                • Don't leave pieces on the back rank where they're passive".to_string(),
            example_fen: None,
            example_arrows: vec![],
            example_highlights: vec![],
            related_concepts: vec!["development".to_string(), "centralization".to_string()],
            practice_exercises: vec![],
        },
        ChessConcept {
            id: "pawn_structure".to_string(),
            name: "Pawn Structure".to_string(),
            category: "Strategy".to_string(),
            difficulty: "Intermediate".to_string(),
            short_description: "The arrangement of pawns determines the character of the position.".to_string(),
            full_explanation: "Pawn structure is the backbone of chess strategy. Since pawns cannot move \
                backward, every pawn move permanently changes the position.\n\n\
                Key pawn structures to know:\n\
                • Isolated pawn: No friendly pawns on adjacent files (weak but can be active)\n\
                • Doubled pawns: Two pawns on the same file (usually weak)\n\
                • Passed pawn: No enemy pawns can block or capture it (very strong)\n\
                • Pawn chain: Diagonal line of pawns protecting each other\n\
                • Backward pawn: Cannot be protected by other pawns (often a target)".to_string(),
            example_fen: None,
            example_arrows: vec![],
            example_highlights: vec![],
            related_concepts: vec!["isolated_pawn".to_string(), "passed_pawn".to_string(), "doubled_pawns".to_string()],
            practice_exercises: vec![],
        },
        // OPENINGS
        ChessConcept {
            id: "opening_principles".to_string(),
            name: "Opening Principles".to_string(),
            category: "Openings".to_string(),
            difficulty: "Beginner".to_string(),
            short_description: "The fundamental guidelines for playing the opening phase well.".to_string(),
            full_explanation: "The opening principles help you get a good position without memorizing specific moves:\n\n\
                1. Control the center: Place pawns on e4/d4 (or e5/d5 for Black)\n\
                2. Develop your pieces: Knights before bishops, toward the center\n\
                3. Castle early: Usually within the first 10 moves for king safety\n\
                4. Connect your rooks: Clear the back rank so rooks protect each other\n\
                5. Don't move the same piece twice: Unless there's a good reason\n\
                6. Don't bring the queen out too early: It can be chased with tempo\n\
                7. Don't make too many pawn moves: Develop pieces instead".to_string(),
            example_fen: Some("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()),
            example_arrows: vec![("e2".to_string(), "e4".to_string()), ("d2".to_string(), "d4".to_string())],
            example_highlights: vec!["e4".to_string(), "d4".to_string(), "e5".to_string(), "d5".to_string()],
            related_concepts: vec!["development".to_string(), "center_control".to_string(), "castling".to_string()],
            practice_exercises: vec![],
        },
        ChessConcept {
            id: "castling".to_string(),
            name: "Castling".to_string(),
            category: "Openings".to_string(),
            difficulty: "Beginner".to_string(),
            short_description: "A special move that tucks your king to safety and activates a rook.".to_string(),
            full_explanation: "Castling is a special move involving the king and a rook. It's the only move \
                where two pieces move at once.\n\n\
                Requirements to castle:\n\
                • Neither the king nor the rook has moved before\n\
                • No pieces between the king and rook\n\
                • The king is not currently in check\n\
                • The king doesn't pass through or land on an attacked square\n\n\
                Kingside castling (O-O): King goes to g1/g8, rook to f1/f8\n\
                Queenside castling (O-O-O): King goes to c1/c8, rook to d1/d8".to_string(),
            example_fen: Some("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1".to_string()),
            example_arrows: vec![("e1".to_string(), "g1".to_string()), ("h1".to_string(), "f1".to_string())],
            example_highlights: vec!["e1".to_string(), "g1".to_string(), "h1".to_string(), "f1".to_string()],
            related_concepts: vec!["king_safety".to_string(), "opening_principles".to_string()],
            practice_exercises: vec![],
        },
        // ENDGAMES
        ChessConcept {
            id: "opposition".to_string(),
            name: "Opposition".to_string(),
            category: "Endgames".to_string(),
            difficulty: "Intermediate".to_string(),
            short_description: "A key endgame concept where kings face each other with one square between.".to_string(),
            full_explanation: "Opposition is a crucial concept in king and pawn endgames. When two kings \
                stand on the same file or rank with one square between them, the player NOT to move \
                has the opposition - a significant advantage.\n\n\
                Why it matters:\n\
                • The side with opposition can force the enemy king to give way\n\
                • This allows your king to advance or protect a passed pawn\n\
                • It's often the difference between winning and drawing a pawn endgame\n\n\
                Types: Direct opposition (same file/rank), diagonal opposition, distant opposition.".to_string(),
            example_fen: Some("8/8/8/3k4/8/3K4/8/8 w - - 0 1".to_string()),
            example_arrows: vec![],
            example_highlights: vec!["d3".to_string(), "d5".to_string()],
            related_concepts: vec!["king_and_pawn".to_string(), "zugzwang".to_string()],
            practice_exercises: vec!["opposition_1".to_string()],
        },
        ChessConcept {
            id: "zugzwang".to_string(),
            name: "Zugzwang".to_string(),
            category: "Endgames".to_string(),
            difficulty: "Advanced".to_string(),
            short_description: "A position where any move worsens your position - being forced to move is a disadvantage.".to_string(),
            full_explanation: "Zugzwang is a German word meaning 'compulsion to move.' It describes positions \
                where having to move is a disadvantage - if you could pass, you'd be fine, but every \
                legal move makes your position worse.\n\n\
                Zugzwang is most common in endgames, especially king and pawn endings. It's often the \
                key to converting a winning position or finding a draw in a losing one.\n\n\
                Mutual zugzwang occurs when whoever moves loses - this is where opposition becomes crucial.".to_string(),
            example_fen: Some("8/8/8/8/8/1k6/2p5/1K6 w - - 0 1".to_string()),
            example_arrows: vec![],
            example_highlights: vec!["b1".to_string()],
            related_concepts: vec!["opposition".to_string(), "triangulation".to_string()],
            practice_exercises: vec![],
        },
    ]
}

#[tauri::command]
pub fn get_all_concepts() -> Vec<ChessConcept> {
    get_concept_library()
}

#[tauri::command]
pub fn get_concept(concept_id: String) -> Option<ChessConcept> {
    get_concept_library()
        .into_iter()
        .find(|c| c.id == concept_id)
}

#[tauri::command]
pub fn get_concepts_by_category(category: String) -> Vec<ChessConcept> {
    get_concept_library()
        .into_iter()
        .filter(|c| c.category.to_lowercase() == category.to_lowercase())
        .collect()
}

#[tauri::command]
pub fn search_concepts(query: String) -> Vec<ChessConcept> {
    let query_lower = query.to_lowercase();
    get_concept_library()
        .into_iter()
        .filter(|c| {
            c.name.to_lowercase().contains(&query_lower) ||
            c.short_description.to_lowercase().contains(&query_lower) ||
            c.full_explanation.to_lowercase().contains(&query_lower)
        })
        .collect()
}

#[tauri::command]
pub fn get_concept_categories() -> Vec<ConceptCategory> {
    let concepts = get_concept_library();
    let mut categories: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    
    for concept in concepts {
        categories
            .entry(concept.category.clone())
            .or_insert_with(Vec::new)
            .push(concept.id);
    }
    
    categories
        .into_iter()
        .map(|(name, concepts)| ConceptCategory { name, concepts })
        .collect()
}

#[tauri::command]
pub fn define_term(term: String) -> Option<String> {
    let term_lower = term.to_lowercase();
    
    // Quick definitions for common terms
    let definitions: std::collections::HashMap<&str, &str> = [
        ("check", "An attack on the king. The king must get out of check on the next move."),
        ("checkmate", "A check that cannot be escaped. The game ends."),
        ("stalemate", "When a player has no legal moves but is not in check. The game is a draw."),
        ("en passant", "A special pawn capture that can occur when a pawn advances two squares and lands beside an enemy pawn."),
        ("promotion", "When a pawn reaches the opposite end of the board, it must become a queen, rook, bishop, or knight."),
        ("tempo", "A unit of time in chess, essentially one move. Gaining tempo means making your opponent waste a move."),
        ("fianchetto", "Developing a bishop to g2/b2 (or g7/b7) after moving the knight pawn one square."),
        ("gambit", "An opening where material (usually a pawn) is sacrificed for positional compensation."),
        ("blunder", "A serious mistake that dramatically worsens your position."),
        ("brilliancy", "An exceptionally creative or beautiful move, often involving a sacrifice."),
        ("elo", "A rating system used to calculate the relative skill levels of players."),
        ("material", "The pieces and pawns. Having more material generally means having an advantage."),
        ("initiative", "Having control of the game, making threats and forcing your opponent to respond."),
        ("compensation", "Non-material advantages (like activity or attack) that balance material losses."),
    ].into_iter().collect();
    
    definitions.get(term_lower.as_str()).map(|s| s.to_string())
}

#[tauri::command]
pub fn get_related_concepts(concept_id: String) -> Vec<ChessConcept> {
    let concepts = get_concept_library();
    
    // First find the related concept IDs
    let related_ids: Vec<String> = concepts
        .iter()
        .find(|c| c.id == concept_id)
        .map(|c| c.related_concepts.clone())
        .unwrap_or_default();
    
    // Then filter concepts by those IDs
    concepts
        .into_iter()
        .filter(|c| related_ids.contains(&c.id))
        .collect()
}
