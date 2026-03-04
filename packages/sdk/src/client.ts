import {
  ChatOptions,
  ChatResponse,
  DoubaoClientConfig,
  DoubaoError,
  Message,
  MODELS,
  StreamChunk,
} from './types.js';

const API_BASE = 'https://ark.cn-beijing.volces.com/api/v3';

export class DoubaoClient {
  private readonly apiKey: string;
  private readonly baseUrl: string;
  private readonly defaultModel: string;
  private readonly defaultTemperature: number;
  private readonly defaultMaxTokens: number;

  constructor(config: DoubaoClientConfig = {}) {
    const key =
      config.apiKey ??
      (typeof process !== 'undefined' ? process.env.DOUBAO_API_KEY ?? '' : '');

    if (!key) {
      throw new DoubaoError(
        'Missing API key — set DOUBAO_API_KEY or pass apiKey in config.',
      );
    }

    this.apiKey             = key;
    this.baseUrl            = (config.baseUrl ?? API_BASE).replace(/\/$/, '');
    this.defaultModel       = config.model       ?? MODELS.PRO_32K;
    this.defaultTemperature = config.temperature ?? 0.7;
    this.defaultMaxTokens   = config.maxTokens   ?? 2048;
  }

  // ── Non-streaming ──────────────────────────────────────────────────────────

  async chat(opts: ChatOptions): Promise<ChatResponse> {
    const body = this.buildBody(opts, false);
    const resp = await this.post('/chat/completions', body);

    if (!resp.ok) {
      const err = await resp.json().catch(() => ({ error: { message: resp.statusText } }));
      throw new DoubaoError(err.error?.message ?? 'Unknown error', resp.status);
    }

    return resp.json() as Promise<ChatResponse>;
  }

  // ── Streaming ──────────────────────────────────────────────────────────────

  async *chatStream(opts: ChatOptions): AsyncGenerator<StreamChunk> {
    const body = this.buildBody(opts, true);
    const resp = await this.post('/chat/completions', body);

    if (!resp.ok) {
      const err = await resp.json().catch(() => ({ error: { message: resp.statusText } }));
      throw new DoubaoError(err.error?.message ?? 'Unknown error', resp.status);
    }

    if (!resp.body) throw new DoubaoError('No response body');

    const reader  = resp.body.getReader();
    const decoder = new TextDecoder();
    let   buffer  = '';

    while (true) {
      const { done, value } = await reader.read();
      if (done) break;

      buffer += decoder.decode(value, { stream: true });

      const lines = buffer.split('\n');
      buffer = lines.pop() ?? '';

      for (const line of lines) {
        const trimmed = line.trim();
        if (!trimmed || trimmed === 'data: [DONE]') continue;
        if (trimmed.startsWith('data: ')) {
          try {
            const chunk = JSON.parse(trimmed.slice(6)) as StreamChunk;
            yield chunk;
          } catch {
            // skip malformed lines
          }
        }
      }
    }
  }

  // ── Helpers ────────────────────────────────────────────────────────────────

  /** One-turn, non-streaming convenience method. */
  async ask(prompt: string, model?: string): Promise<string> {
    const resp = await this.chat({
      model:    model ?? this.defaultModel,
      messages: [{ role: 'user', content: prompt }],
    });
    return resp.choices[0]?.message.content ?? '';
  }

  /** Returns masked API key for display. */
  maskedKey(): string {
    const k = this.apiKey;
    return k.length > 8 ? `${k.slice(0, 4)}...${k.slice(-4)}` : '****';
  }

  // ── Private ────────────────────────────────────────────────────────────────

  private buildBody(opts: ChatOptions, stream: boolean): Record<string, unknown> {
    return {
      model:       opts.model       ?? this.defaultModel,
      messages:    opts.messages,
      temperature: opts.temperature ?? this.defaultTemperature,
      max_tokens:  opts.max_tokens  ?? this.defaultMaxTokens,
      top_p:       opts.top_p       ?? 1.0,
      ...(stream ? { stream: true } : {}),
    };
  }

  private post(path: string, body: unknown): Promise<Response> {
    return fetch(`${this.baseUrl}${path}`, {
      method:  'POST',
      headers: {
        'Content-Type':  'application/json',
        'Authorization': `Bearer ${this.apiKey}`,
      },
      body: JSON.stringify(body),
    });
  }
}
