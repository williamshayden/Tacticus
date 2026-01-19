import React from 'react';
import { Chessboard } from 'react-chessboard';
import './ChessBoard.css';

interface ChessBoardProps {
  fen?: string;
  orientation?: 'white' | 'black';
  onMove?: (from: string, to: string) => boolean;
  selectedSquare?: string | null;
  legalMoves?: string[];
  lastMove?: { from: string; to: string } | null;
  arrows?: { from: string; to: string; color?: string }[];
  highlights?: { square: string; color?: string }[];
  interactive?: boolean;
}

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
    return onMove(args.sourceSquare, args.targetSquare);
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
    </div>
  );
};
