// ── Models ──────────────────────────────────────────────────────────────────

export const MODELS = {
  PRO_32K:   'doubao-pro-32k',
  PRO_4K:    'doubao-pro-4k',
  LITE_32K:  'doubao-lite-32k',
  LITE_4K:   'doubao-lite-4k',
  PRO_128K:  'doubao-pro-128k',
} as const;

export type ModelId = (typeof MODELS)[keyof typeof MODELS] | (string & {});

// ── Messages ─────────────────────────────────────────────────────────────────

export type Role = 'system' | 'user' | 'assistant';

export interface Message {
  role:    Role;
  content: string;
}

// ── Requests ──────────────────────────────────────────────────────────────────

export interface ChatOptions {
  model?:       ModelId;
  messages:     Message[];
  temperature?: number;
  max_tokens?:  number;
  top_p?:       number;
  stream?:      boolean;
}

// ── Responses ─────────────────────────────────────────────────────────────────

export interface Usage {
  prompt_tokens:     number;
  completion_tokens: number;
  total_tokens:      number;
}

export interface Choice {
  index:         number;
  message:       Message;
  finish_reason: string | null;
}

export interface ChatResponse {
  id:      string;
  object:  string;
  created: number;
  model:   string;
  choices: Choice[];
  usage:   Usage;
}

export interface Delta {
  role?:    Role;
  content?: string;
}

export interface StreamChoice {
  index:         number;
  delta:         Delta;
  finish_reason: string | null;
}

export interface StreamChunk {
  id:      string;
  object:  string;
  created: number;
  model:   string;
  choices: StreamChoice[];
}

// ── Client config ─────────────────────────────────────────────────────────────

export interface DoubaoClientConfig {
  /** Your Volcengine API key. Falls back to DOUBAO_API_KEY env var. */
  apiKey?:     string;
  /** Override the API base URL (useful for proxies / testing). */
  baseUrl?:    string;
  /** Default model for all requests. */
  model?:      ModelId;
  /** Default temperature (0–1). */
  temperature?: number;
  /** Default max_tokens. */
  maxTokens?:  number;
}

// ── Errors ────────────────────────────────────────────────────────────────────

export class DoubaoError extends Error {
  constructor(
    message: string,
    public readonly status?: number,
    public readonly code?: string,
  ) {
    super(message);
    this.name = 'DoubaoError';
  }
}
