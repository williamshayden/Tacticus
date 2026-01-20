import { invoke } from '@tauri-apps/api/core';
import { GURGEH_SYSTEM_PROMPT } from './prompts';
import type { ChatMessage, PlayerStats, Game, TrainingProgress, ImprovementTrend, WeaknessEntry } from './types';

// Default model to use
const DEFAULT_MODEL = 'anthropic/claude-3-haiku';

export interface StreamCallbacks {
  onChunk: (chunk: string) => void;
  onComplete: (fullText: string) => void;
  onError: (error: Error) => void;
  onToolCall?: (toolName: string, args: unknown) => void;
  onToolResult?: (toolName: string, result: unknown) => void;
}

// Tool definitions for the LLM
const TOOL_DEFINITIONS = [
  {
    type: 'function',
    function: {
      name: 'getRecentGames',
      description: "Get the player's most recent games with analysis data",
      parameters: {
        type: 'object',
        properties: {
          count: { type: 'number', description: 'Number of recent games to retrieve (1-20)' }
        },
        required: ['count']
      }
    }
  },
  {
    type: 'function',
    function: {
      name: 'getPlayerStats',
      description: "Get comprehensive player statistics including ELO, win rate, and identified weaknesses",
      parameters: { type: 'object', properties: {} }
    }
  },
  {
    type: 'function',
    function: {
      name: 'getWeaknessHistory',
      description: "Get the player's weakness history showing exercise types where they struggle",
      parameters: {
        type: 'object',
        properties: {
          days: { type: 'number', description: 'Number of days to look back (1-365)' }
        },
        required: ['days']
      }
    }
  },
  {
    type: 'function',
    function: {
      name: 'searchGamesByOpening',
      description: "Search the player's games by opening name",
      parameters: {
        type: 'object',
        properties: {
          openingName: { type: 'string', description: 'Name of the opening to search for' }
        },
        required: ['openingName']
      }
    }
  },
  {
    type: 'function',
    function: {
      name: 'getGamesWithMistakes',
      description: "Get games where the player made significant mistakes",
      parameters: {
        type: 'object',
        properties: {
          minMistakes: { type: 'number', description: 'Minimum number of mistakes to filter by (1-10)' }
        },
        required: ['minMistakes']
      }
    }
  },
  {
    type: 'function',
    function: {
      name: 'getTrainingProgress',
      description: "Get the player's training exercise progress",
      parameters: {
        type: 'object',
        properties: {
          exerciseType: { type: 'string', description: 'Optional exercise type to filter by' }
        }
      }
    }
  },
  {
    type: 'function',
    function: {
      name: 'getImprovementTrend',
      description: "Get the player's improvement trend over a period of time",
      parameters: {
        type: 'object',
        properties: {
          days: { type: 'number', description: 'Number of days to analyze (1-365)' }
        },
        required: ['days']
      }
    }
  }
];

// Execute a tool by name
async function executeTool(name: string, args: Record<string, unknown>): Promise<unknown> {
  switch (name) {
    case 'getRecentGames': {
      const games = await invoke<Game[]>('get_recent_games', { count: args.count as number });
      return {
        success: true,
        games: games.map(g => ({
          id: g.id,
          result: g.result,
          playerColor: g.player_color,
          opponentType: g.opponent_type,
          opponentElo: g.opponent_elo,
          moves: g.moves.length,
          mistakes: g.mistakes,
          blunders: g.blunders,
          opening: g.opening_name,
          playedAt: g.created_at,
        })),
      };
    }
    case 'getPlayerStats': {
      const stats = await invoke<PlayerStats>('get_player_stats');
      return {
        success: true,
        stats: {
          currentElo: stats.current_elo,
          peakElo: stats.peak_elo,
          gamesPlayed: stats.games_played,
          wins: stats.wins,
          losses: stats.losses,
          draws: stats.draws,
          winRate: stats.win_rate.toFixed(1) + '%',
          exercisesCompleted: stats.exercises_completed,
          exerciseSuccessRate: stats.exercise_success_rate.toFixed(1) + '%',
          streak: stats.streak,
          style: stats.style,
          weaknesses: stats.weaknesses,
          strengths: stats.strengths,
        },
      };
    }
    case 'getWeaknessHistory': {
      const weaknesses = await invoke<WeaknessEntry[]>('get_weakness_history', { days: args.days as number });
      return {
        success: true,
        weaknesses: weaknesses.map(w => ({
          exerciseType: w.exercise_type,
          attempts: w.total_attempts,
          successRate: w.success_rate.toFixed(1) + '%',
          trend: w.recent_trend,
        })),
      };
    }
    case 'searchGamesByOpening': {
      const games = await invoke<Game[]>('search_games_by_opening', { openingName: args.openingName as string });
      return {
        success: true,
        totalGames: games.length,
        games: games.slice(0, 10).map(g => ({
          id: g.id,
          result: g.result,
          playerColor: g.player_color,
          opening: g.opening_name,
          mistakes: g.mistakes,
          blunders: g.blunders,
          playedAt: g.created_at,
        })),
      };
    }
    case 'getGamesWithMistakes': {
      const games = await invoke<Game[]>('get_games_with_mistakes', { minMistakes: args.minMistakes as number });
      return {
        success: true,
        totalGames: games.length,
        games: games.slice(0, 10).map(g => ({
          id: g.id,
          result: g.result,
          playerColor: g.player_color,
          opening: g.opening_name,
          mistakes: g.mistakes,
          blunders: g.blunders,
          playedAt: g.created_at,
        })),
      };
    }
    case 'getTrainingProgress': {
      const progress = await invoke<TrainingProgress>('get_training_progress', {
        exerciseType: (args.exerciseType as string) || null
      });
      return {
        success: true,
        progress: {
          totalAttempted: progress.total_attempted,
          totalSolved: progress.total_solved,
          successRate: progress.success_rate.toFixed(1) + '%',
          avgTimeSeconds: Math.round(progress.avg_time_seconds),
          avgHintsUsed: progress.avg_hints_used.toFixed(1),
        },
      };
    }
    case 'getImprovementTrend': {
      const trend = await invoke<ImprovementTrend>('get_improvement_trend', { days: args.days as number });
      return {
        success: true,
        trend: {
          eloChange: trend.elo_change,
          gamesPlayed: trend.games_in_period,
          winRate: trend.win_rate_in_period.toFixed(1) + '%',
          exercisesCompleted: trend.exercises_in_period,
          exerciseSuccessRate: trend.exercise_success_rate_in_period.toFixed(1) + '%',
        },
      };
    }
    default:
      return { success: false, error: `Unknown tool: ${name}` };
  }
}

interface OpenRouterMessage {
  role: 'system' | 'user' | 'assistant' | 'tool';
  content: string;
  tool_calls?: Array<{
    id: string;
    type: 'function';
    function: { name: string; arguments: string };
  }>;
  tool_call_id?: string;
}

interface OpenRouterChoice {
  delta?: { content?: string; tool_calls?: Array<{ id?: string; function?: { name?: string; arguments?: string } }> };
  message?: { content: string; tool_calls?: Array<{ id: string; type: string; function: { name: string; arguments: string } }> };
  finish_reason?: string;
}

// Stream a response from the AI coach with tool execution
export async function streamCoachResponse(
  apiKey: string,
  messages: ChatMessage[],
  callbacks: StreamCallbacks,
  model: string = DEFAULT_MODEL,
): Promise<void> {
  const openRouterMessages: OpenRouterMessage[] = [
    { role: 'system', content: GURGEH_SYSTEM_PROMPT },
    ...messages.map(m => ({ role: m.role as 'user' | 'assistant', content: m.content })),
  ];

  let currentToolCalls: Array<{ id: string; name: string; arguments: string }> = [];
  let maxIterations = 5;

  try {
    while (maxIterations > 0) {
      maxIterations--;

      const response = await fetch('https://openrouter.ai/api/v1/chat/completions', {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${apiKey}`,
          'Content-Type': 'application/json',
          'HTTP-Referer': 'https://github.com/tacticus-chess',
          'X-Title': 'Tacticus Chess Trainer',
        },
        body: JSON.stringify({
          model,
          messages: openRouterMessages,
          tools: TOOL_DEFINITIONS,
          stream: true,
        }),
      });

      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`API error (${response.status}): ${errorText}`);
      }

      const reader = response.body?.getReader();
      if (!reader) throw new Error('No response body');

      const decoder = new TextDecoder();
      let buffer = '';
      let fullText = '';
      currentToolCalls = [];
      let hasToolCalls = false;

      while (true) {
        const { done, value } = await reader.read();
        if (done) break;

        buffer += decoder.decode(value, { stream: true });
        const lines = buffer.split('\n');
        buffer = lines.pop() || '';

        for (const line of lines) {
          if (!line.startsWith('data: ')) continue;
          const data = line.slice(6).trim();
          if (data === '[DONE]') continue;

          try {
            const parsed = JSON.parse(data);
            const choice: OpenRouterChoice = parsed.choices?.[0];
            if (!choice) continue;

            // Handle streaming content
            if (choice.delta?.content) {
              fullText += choice.delta.content;
              callbacks.onChunk(choice.delta.content);
            }

            // Handle tool calls
            if (choice.delta?.tool_calls) {
              hasToolCalls = true;
              for (const tc of choice.delta.tool_calls) {
                const idx = tc.id ? currentToolCalls.findIndex(c => c.id === tc.id) : currentToolCalls.length - 1;
                if (tc.id && idx === -1) {
                  currentToolCalls.push({ id: tc.id, name: tc.function?.name || '', arguments: tc.function?.arguments || '' });
                } else if (idx >= 0) {
                  if (tc.function?.name) currentToolCalls[idx].name = tc.function.name;
                  if (tc.function?.arguments) currentToolCalls[idx].arguments += tc.function.arguments;
                }
              }
            }
          } catch {
            // Skip invalid JSON
          }
        }
      }

      // If no tool calls, we're done
      if (!hasToolCalls || currentToolCalls.length === 0) {
        callbacks.onComplete(fullText);
        return;
      }

      // Execute tool calls and add results to messages
      openRouterMessages.push({
        role: 'assistant',
        content: fullText,
        tool_calls: currentToolCalls.map(tc => ({
          id: tc.id,
          type: 'function' as const,
          function: { name: tc.name, arguments: tc.arguments },
        })),
      });

      for (const tc of currentToolCalls) {
        callbacks.onToolCall?.(tc.name, JSON.parse(tc.arguments || '{}'));

        try {
          const result = await executeTool(tc.name, JSON.parse(tc.arguments || '{}'));
          callbacks.onToolResult?.(tc.name, result);

          openRouterMessages.push({
            role: 'tool',
            tool_call_id: tc.id,
            content: JSON.stringify(result),
          });
        } catch (error) {
          openRouterMessages.push({
            role: 'tool',
            tool_call_id: tc.id,
            content: JSON.stringify({ success: false, error: String(error) }),
          });
        }
      }
    }

    callbacks.onComplete('');
  } catch (error) {
    callbacks.onError(error instanceof Error ? error : new Error(String(error)));
  }
}

// Generate a non-streaming response
export async function generateCoachResponse(
  apiKey: string,
  messages: ChatMessage[],
  model: string = DEFAULT_MODEL,
): Promise<string> {
  return new Promise((resolve, reject) => {
    let fullText = '';
    streamCoachResponse(apiKey, messages, {
      onChunk: (chunk) => { fullText += chunk; },
      onComplete: () => resolve(fullText),
      onError: reject,
    }, model);
  });
}

// Get a personalized greeting
export async function getPersonalizedGreeting(
  apiKey: string,
  userName: string,
  model: string = DEFAULT_MODEL,
): Promise<string> {
  return generateCoachResponse(
    apiKey,
    [{
      role: 'user',
      content: `The player "${userName}" just opened the app. Give them a brief, personalized greeting. Use the getPlayerStats tool to check their current rating and recent activity, then welcome them appropriately.`,
    }],
    model,
  );
}

// Analyze a specific position
export async function analyzePosition(
  apiKey: string,
  fen: string,
  model: string = DEFAULT_MODEL,
): Promise<string> {
  return generateCoachResponse(
    apiKey,
    [{
      role: 'user',
      content: `Analyze this chess position for me. The position in FEN notation is: ${fen}

Please provide:
1. Who is better and why (material, position, king safety)
2. Key features of the position
3. Best plan for the side to move
4. Any tactical opportunities`,
    }],
    model,
  );
}
