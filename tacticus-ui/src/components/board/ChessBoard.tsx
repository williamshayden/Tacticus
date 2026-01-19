import React, { useState } from 'react';
import { Chessboard } from 'react-chessboard';
import './ChessBoard.css';

interface ChessBoardProps {
  fen?: string;
  orientation?: 'white' | 'black';
  onMove?: (from: string, to: string, promotion?: string) => boolean;
  selectedSquare?: string | null;
  legalMoves?: string[];
  lastMove?: { from: string; to: string } | null;
  arrows?: { from: string; to: string; color?: string }[];
  highlights?: { square: string; color?: string }[];
  interactive?: boolean;
}

// Check if a move is a pawn promotion
const isPromotionMove = (fen: string, from: string, to: string): boolean => {
  const fromRank = from[1];
  const toRank = to[1];
  const fromFile = from[0];
  
  // Get the piece on the from square from FEN
  const fenParts = fen.split(' ');
  const position = fenParts[0];
  const rows = position.split('/');
  
  // Convert algebraic to array index
  const fileIndex = fromFile.charCodeAt(0) - 'a'.charCodeAt(0);
  const rankIndex = 8 - parseInt(fromRank);
  
  // Expand FEN row to get piece
  let expandedRow = '';
  for (const char of rows[rankIndex]) {
    if (char >= '1' && char <= '8') {
      expandedRow += '.'.repeat(parseInt(char));
    } else {
      expandedRow += char;
    }
  }
  
  const piece = expandedRow[fileIndex];
  
  // Check if it's a pawn moving to the last rank
  if (piece === 'P' && toRank === '8') return true;  // White pawn
  if (piece === 'p' && toRank === '1') return true;  // Black pawn
  
  return false;
};

export const ChessBoard: React.FC<ChessBoardProps> = ({
  fen = 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1',
  orientation = 'white',
  onMove,
  selectedSquare,
  legalMoves = [],
  lastMove,
  arrows = [],
  highlights = [],
  interactive = true,
}) => {
  const [showPromotion, setShowPromotion] = useState(false);
  const [pendingMove, setPendingMove] = useState<{ from: string; to: string } | null>(null);

  const squareStyles: Record<string, React.CSSProperties> = {};

  // Highlight selected square
  if (selectedSquare) {
    squareStyles[selectedSquare] = {
      backgroundColor: 'rgba(100, 150, 255, 0.6)',
    };
  }

  // Highlight legal move destinations
  legalMoves.forEach((square) => {
    squareStyles[square] = {
      ...squareStyles[square],
      background: 'radial-gradient(circle, rgba(0, 100, 0, 0.4) 25%, transparent 25%)',
    };
  });

  // Highlight last move
  if (lastMove) {
    squareStyles[lastMove.from] = {
      ...squareStyles[lastMove.from],
      backgroundColor: 'rgba(255, 255, 130, 0.5)',
    };
    squareStyles[lastMove.to] = {
      ...squareStyles[lastMove.to],
      backgroundColor: 'rgba(255, 255, 130, 0.5)',
    };
  }

  // Custom highlights
  highlights.forEach((h) => {
    squareStyles[h.square] = {
      ...squareStyles[h.square],
      backgroundColor: h.color || 'rgba(255, 0, 0, 0.4)',
    };
  });

  const arrowsData = arrows.map((a) => ({
    startSquare: a.from,
    endSquare: a.to,
    color: a.color || 'rgba(0, 128, 0, 0.7)',
  }));

  const handlePieceDrop = (args: { piece: unknown; sourceSquare: string; targetSquare: string | null }): boolean => {
    if (!interactive || !onMove || !args.targetSquare) return false;
    
    // Check if this is a promotion move
    if (isPromotionMove(fen, args.sourceSquare, args.targetSquare)) {
      setPendingMove({ from: args.sourceSquare, to: args.targetSquare });
      setShowPromotion(true);
      return false; // Don't complete the move yet
    }
    
    return onMove(args.sourceSquare, args.targetSquare);
  };

  const handlePromotion = (piece: string) => {
    if (pendingMove && onMove) {
      onMove(pendingMove.from, pendingMove.to, piece);
    }
    setShowPromotion(false);
    setPendingMove(null);
  };

  const cancelPromotion = () => {
    setShowPromotion(false);
    setPendingMove(null);
  };

  return (
    <div className="chess-board-container">
      <div className="chess-board-frame">
        <Chessboard
          options={{
            position: fen,
            boardOrientation: orientation,
            onPieceDrop: handlePieceDrop,
            squareStyles: squareStyles,
            arrows: arrowsData,
            allowDragging: interactive,
            boardStyle: {
              borderRadius: '4px',
              boxShadow: 'inset 0 0 4px rgba(0,0,0,0.3)',
            },
            darkSquareStyle: { backgroundColor: '#b58863' },
            lightSquareStyle: { backgroundColor: '#f0d9b5' },
          }}
        />
      </div>
      
      {/* Promotion Dialog */}
      {showPromotion && (
        <div className="promotion-overlay" onClick={cancelPromotion}>
          <div className="promotion-dialog" onClick={(e) => e.stopPropagation()}>
            <div className="promotion-title">Promote pawn to:</div>
            <div className="promotion-pieces">
              <button className="promotion-piece" onClick={() => handlePromotion('q')} title="Queen">
                {orientation === 'white' || fen.includes(' b ') ? 'Q' : 'q'}
              </button>
              <button className="promotion-piece" onClick={() => handlePromotion('r')} title="Rook">
                {orientation === 'white' || fen.includes(' b ') ? 'R' : 'r'}
              </button>
              <button className="promotion-piece" onClick={() => handlePromotion('b')} title="Bishop">
                {orientation === 'white' || fen.includes(' b ') ? 'B' : 'b'}
              </button>
              <button className="promotion-piece" onClick={() => handlePromotion('n')} title="Knight">
                {orientation === 'white' || fen.includes(' b ') ? 'N' : 'n'}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
