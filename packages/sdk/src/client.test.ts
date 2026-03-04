import { describe, it, expect, vi, beforeEach } from 'vitest';
import { DoubaoClient, DoubaoError, MODELS } from '../src/index.js';

// ── Mock fetch ─────────────────────────────────────────────────────────────

const mockFetch = vi.fn();
vi.stubGlobal('fetch', mockFetch);

function jsonResponse(body: unknown, status = 200) {
  return Promise.resolve({
    ok:     status < 400,
    status,
    json:   () => Promise.resolve(body),
    body:   null,
  } as unknown as Response);
}

// ── Tests ──────────────────────────────────────────────────────────────────

describe('DoubaoClient', () => {
  beforeEach(() => mockFetch.mockReset());

  it('throws on missing API key', () => {
    const prev = process.env.DOUBAO_API_KEY;
    delete process.env.DOUBAO_API_KEY;
    expect(() => new DoubaoClient({})).toThrow(DoubaoError);
    if (prev) process.env.DOUBAO_API_KEY = prev;
  });

  it('creates client with explicit key', () => {
    const client = new DoubaoClient({ apiKey: 'test-key' });
    expect(client.maskedKey()).toBe('test...key-');
  });

  it('chat() returns ChatResponse', async () => {
    const payload = {
      id: 'chatcmpl-test', object: 'chat.completion', created: 0,
      model: MODELS.PRO_32K,
      choices: [{ index: 0, message: { role: 'assistant', content: 'Hello!' }, finish_reason: 'stop' }],
      usage: { prompt_tokens: 5, completion_tokens: 3, total_tokens: 8 },
    };
    mockFetch.mockResolvedValueOnce(jsonResponse(payload));

    const client = new DoubaoClient({ apiKey: 'sk-test' });
    const resp   = await client.chat({ messages: [{ role: 'user', content: 'Hi' }] });

    expect(resp.choices[0].message.content).toBe('Hello!');
    expect(mockFetch).toHaveBeenCalledTimes(1);
    const [url, init] = mockFetch.mock.calls[0] as [string, RequestInit];
    expect(url).toContain('/chat/completions');
    expect(init.headers).toMatchObject({ 'Authorization': 'Bearer sk-test' });
  });

  it('chat() throws DoubaoError on 401', async () => {
    mockFetch.mockResolvedValueOnce(jsonResponse(
      { error: { message: 'Invalid API key' } }, 401,
    ));
    const client = new DoubaoClient({ apiKey: 'bad-key' });
    await expect(client.chat({ messages: [] })).rejects.toThrow('Invalid API key');
  });

  it('ask() returns content string', async () => {
    const payload = {
      id: 'x', object: 'chat.completion', created: 0, model: MODELS.PRO_4K,
      choices: [{ index: 0, message: { role: 'assistant', content: '42' }, finish_reason: 'stop' }],
      usage: { prompt_tokens: 1, completion_tokens: 1, total_tokens: 2 },
    };
    mockFetch.mockResolvedValueOnce(jsonResponse(payload));
    const client = new DoubaoClient({ apiKey: 'sk-test' });
    const answer = await client.ask('What is the answer?');
    expect(answer).toBe('42');
  });
});
