import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { XPWindow } from './xp/XPWindow';
import { XPButton } from './xp/XPButton';
import { XPPanel } from './xp/XPPanel';
import { XPInput } from './xp/XPInput';
import { ChessBoard } from './board/ChessBoard';
import './LearnMode.css';

interface ChessConcept {
  id: string;
  name: string;
  category: string;
  difficulty: string;
  short_description: string;
  full_explanation: string;
  example_fen: string | null;
  example_arrows: [string, string][];
  example_highlights: string[];
  related_concepts: string[];
  practice_exercises: string[];
}

interface ConceptCategory {
  name: string;
  concepts: string[];
}

interface LearnModeProps {
  onBack: () => void;
}

export const LearnMode: React.FC<LearnModeProps> = ({ onBack }) => {
  const [categories, setCategories] = useState<ConceptCategory[]>([]);
  const [concepts, setConcepts] = useState<ChessConcept[]>([]);
  const [selectedCategory, setSelectedCategory] = useState<string | null>(null);
  const [selectedConcept, setSelectedConcept] = useState<ChessConcept | null>(null);
  const [searchQuery, setSearchQuery] = useState('');
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadConcepts();
  }, []);

  const loadConcepts = async () => {
    try {
      const [cats, cons] = await Promise.all([
        invoke<ConceptCategory[]>('get_concept_categories'),
        invoke<ChessConcept[]>('get_all_concepts'),
      ]);
      setCategories(cats);
      setConcepts(cons);
    } catch (err) {
      console.error('Failed to load concepts:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleSearch = async (query: string) => {
    setSearchQuery(query);
    if (query.trim()) {
      try {
        const results = await invoke<ChessConcept[]>('search_concepts', { query });
        setConcepts(results);
        setSelectedCategory(null);
      } catch (err) {
        console.error('Search failed:', err);
      }
    } else {
      const all = await invoke<ChessConcept[]>('get_all_concepts');
      setConcepts(all);
    }
  };

  const handleCategorySelect = async (category: string) => {
    setSelectedCategory(category);
    setSelectedConcept(null);
    try {
      const categoryConcepts = await invoke<ChessConcept[]>('get_concepts_by_category', { category });
      setConcepts(categoryConcepts);
    } catch (err) {
      console.error('Failed to load category:', err);
    }
  };

  const handleConceptSelect = (concept: ChessConcept) => {
    setSelectedConcept(concept);
  };

  const filteredConcepts = selectedCategory
    ? concepts.filter(c => c.category === selectedCategory)
    : concepts;

  if (loading) {
    return (
      <div className="learn-mode-container">
        <div className="loading-message">Loading concepts...</div>
      </div>
    );
  }

  return (
    <div className="learn-mode-container">
      <div className="learn-layout">
        {/* Categories Sidebar */}
        <XPWindow title="Categories" icon="[C]" width={200} height={500}>
          <div className="category-list">
            <button
              className={`category-item ${!selectedCategory ? 'active' : ''}`}
              onClick={() => { setSelectedCategory(null); loadConcepts(); }}
            >
              [*] All Concepts
            </button>
            {categories.map((cat) => (
              <button
                key={cat.name}
                className={`category-item ${selectedCategory === cat.name ? 'active' : ''}`}
                onClick={() => handleCategorySelect(cat.name)}
              >
                {cat.name === 'Tactics' && '[T]'}
                {cat.name === 'Strategy' && '[S]'}
                {cat.name === 'Openings' && '[O]'}
                {cat.name === 'Endgames' && '[E]'}
                {' '}{cat.name}
                <span className="concept-count">({cat.concepts.length})</span>
              </button>
            ))}
          </div>
        </XPWindow>

        {/* Concept List */}
        <XPWindow title="Concepts" icon="[L]" width={300} height={500}>
          <div className="concept-search">
            <XPInput
              value={searchQuery}
              onChange={handleSearch}
              placeholder="Search concepts..."
            />
          </div>
          <div className="concept-list">
            {filteredConcepts.map((concept) => (
              <button
                key={concept.id}
                className={`concept-item ${selectedConcept?.id === concept.id ? 'active' : ''}`}
                onClick={() => handleConceptSelect(concept)}
              >
                <span className="concept-name">{concept.name}</span>
                <span className={`concept-difficulty ${concept.difficulty.toLowerCase()}`}>
                  {concept.difficulty}
                </span>
              </button>
            ))}
          </div>
        </XPWindow>

        {/* Concept Detail */}
        {selectedConcept ? (
          <div className="concept-detail-container">
            <XPWindow
              title={selectedConcept.name}
              icon="[i]"
              width={500}
              height="auto"
            >
              <div className="concept-detail">
                <div className="concept-header">
                  <span className="concept-category">{selectedConcept.category}</span>
                  <span className={`concept-difficulty ${selectedConcept.difficulty.toLowerCase()}`}>
                    {selectedConcept.difficulty}
                  </span>
                </div>

                <p className="concept-short">{selectedConcept.short_description}</p>

                {selectedConcept.example_fen && (
                  <div className="concept-example">
                    <ChessBoard
                      fen={selectedConcept.example_fen}
                      interactive={false}
                      arrows={selectedConcept.example_arrows.map(([from, to]) => ({ from, to }))}
                      highlights={selectedConcept.example_highlights.map(sq => ({ square: sq }))}
                    />
                  </div>
                )}

                <XPPanel label="Explanation" className="concept-explanation">
                  {selectedConcept.full_explanation.split('\n\n').map((para, i) => (
                    <p key={i}>{para}</p>
                  ))}
                </XPPanel>

                {selectedConcept.related_concepts.length > 0 && (
                  <div className="related-concepts">
                    <label>Related Concepts:</label>
                    <div className="related-tags">
                      {selectedConcept.related_concepts.map((id) => {
                        const related = concepts.find(c => c.id === id);
                        return related ? (
                          <button
                            key={id}
                            className="related-tag"
                            onClick={() => handleConceptSelect(related)}
                          >
                            {related.name}
                          </button>
                        ) : null;
                      })}
                    </div>
                  </div>
                )}

                <div className="concept-actions">
                  <XPButton onClick={onBack}>Back to Hub</XPButton>
                  {selectedConcept.practice_exercises.length > 0 && (
                    <XPButton primary>Practice This Concept</XPButton>
                  )}
                </div>
              </div>
            </XPWindow>
          </div>
        ) : (
          <div className="concept-placeholder">
            <XPWindow title="Learn Chess" icon="[L]" width={500} height={300}>
              <div className="placeholder-content">
                <span className="placeholder-icon">[?]</span>
                <h3>Select a Concept</h3>
                <p>Choose a concept from the list to learn about it. You can also search for specific topics.</p>
                <p>Categories include:</p>
                <ul>
                  <li><strong>Tactics</strong> - Forks, pins, skewers, and more</li>
                  <li><strong>Strategy</strong> - Positional play and planning</li>
                  <li><strong>Openings</strong> - Opening principles and common lines</li>
                  <li><strong>Endgames</strong> - Technical positions and key concepts</li>
                </ul>
              </div>
            </XPWindow>
          </div>
        )}
      </div>
    </div>
  );
};
