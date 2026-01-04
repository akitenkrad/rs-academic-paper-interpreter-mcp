# 設計書: Academic Paper Interpreter MCP Server

## 概要

論文の情報を収集し内容を解析するMCPサーバ。`academic-paper-interpreter`ライブラリをラップし、MCPプロトコル経由でAIアシスタントに論文解析機能を提供する。

## アーキテクチャ

```
┌─────────────────────────────────────────────────────────────┐
│                     MCP Client                              │
│              (Claude Desktop, etc.)                         │
└─────────────────────────┬───────────────────────────────────┘
                          │ MCP Protocol (JSON-RPC 2.0)
                          │ Transport: stdio
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                   MCP Server (rmcp)                         │
│  ┌────────────────────────────────────────────────────────┐ │
│  │                    Tool Router                         │ │
│  │  ┌─────────────┐ ┌─────────────┐ ┌──────────────────┐  │ │
│  │  │search_papers│ │ fetch_paper │ │  analyze_paper   │  │ │
│  │  └─────────────┘ └─────────────┘ └──────────────────┘  │ │
│  │  ┌──────────────────────────────────────────────────┐  │ │
│  │  │              interpret_paper (統合)              │  │ │
│  │  └──────────────────────────────────────────────────┘  │ │
│  └────────────────────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────────────────────┐ │
│  │                  LLM Provider Manager                  │ │
│  │     OpenAI  │  Anthropic  │  Ollama (Local)            │ │
│  └────────────────────────────────────────────────────────┘ │
└─────────────────────────┬───────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│              academic-paper-interpreter                     │
│  ┌─────────────┐ ┌─────────────┐ ┌──────────────────────┐   │
│  │ PaperClient │ │PaperAnalyzer│ │    LlmProvider       │   │
│  │ (検索/取得) │ │  (LLM解析)  │ │ (OpenAI/Anthropic/   │   │
│  │             │ │             │ │       Ollama)        │   │
│  └─────────────┘ └─────────────┘ └──────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                    External Services                        │
│     arXiv API  │  Semantic Scholar API  │  LLM APIs         │
└─────────────────────────────────────────────────────────────┘
```

## MCP Tools

### 1. interpret_paper (統合ツール)

論文の検索・取得・解析を一括で実行する。

**Parameters:**
```json
{
  "query": {
    "type": "object",
    "properties": {
      "title": { "type": "string", "description": "論文タイトル（部分一致検索）" },
      "url": { "type": "string", "description": "論文のURL" },
      "pdf_url": { "type": "string", "description": "論文PDFのURL" },
      "arxiv_id": { "type": "string", "description": "arXiv ID (例: 2301.00001)" }
    },
    "description": "少なくとも1つの識別子が必要"
  },
  "llm_config": {
    "type": "object",
    "properties": {
      "provider": { "type": "string", "enum": ["openai", "anthropic", "ollama"] },
      "model": { "type": "string" }
    },
    "description": "省略時は環境変数のデフォルト設定を使用"
  }
}
```

**Response:**
```json
{
  "paper": {
    "title": "...",
    "authors": ["..."],
    "abstract": "...",
    "arxiv_id": "...",
    "categories": ["..."],
    "published_date": "...",
    "pdf_url": "..."
  },
  "analysis": {
    "summary": "...",
    "key_contributions": ["..."],
    "methodology": "...",
    "limitations": ["..."]
  }
}
```

### 2. search_papers

論文を検索する。

**Parameters:**
```json
{
  "query": { "type": "string", "description": "検索キーワード" },
  "author": { "type": "string", "description": "著者名（オプション）" },
  "category": { "type": "string", "description": "arXivカテゴリ (例: cs.CL)（オプション）" },
  "max_results": { "type": "integer", "default": 10, "description": "最大取得件数" }
}
```

**Response:**
```json
{
  "papers": [
    {
      "title": "...",
      "authors": ["..."],
      "abstract": "...",
      "arxiv_id": "...",
      "published_date": "...",
      "pdf_url": "..."
    }
  ],
  "total_count": 42
}
```

### 3. fetch_paper

論文のメタデータとPDFコンテンツを取得する。

**Parameters:**
```json
{
  "arxiv_id": { "type": "string", "description": "arXiv ID" },
  "url": { "type": "string", "description": "論文URL（arxiv_idがない場合）" },
  "include_pdf_content": { "type": "boolean", "default": true, "description": "PDF本文を取得するか" }
}
```

**Response:**
```json
{
  "paper": {
    "title": "...",
    "authors": ["..."],
    "abstract": "...",
    "arxiv_id": "...",
    "categories": ["..."],
    "published_date": "...",
    "pdf_url": "...",
    "content": "..."
  }
}
```

### 4. analyze_paper

取得済みの論文コンテンツをLLMで解析する。

**Parameters:**
```json
{
  "paper": {
    "type": "object",
    "description": "fetch_paperで取得した論文データ"
  },
  "llm_config": {
    "type": "object",
    "properties": {
      "provider": { "type": "string", "enum": ["openai", "anthropic", "ollama"] },
      "model": { "type": "string" }
    }
  },
  "analysis_type": {
    "type": "string",
    "enum": ["summary", "detailed", "comparison"],
    "default": "summary"
  }
}
```

**Response:**
```json
{
  "analysis": {
    "summary": "...",
    "key_contributions": ["..."],
    "methodology": "...",
    "limitations": ["..."],
    "related_work": ["..."]
  }
}
```

## 環境変数

| 変数名 | 説明 | デフォルト |
|--------|------|-----------|
| `LLM_PROVIDER` | デフォルトLLMプロバイダー | `openai` |
| `OPENAI_API_KEY` | OpenAI APIキー | - |
| `OPENAI_MODEL` | OpenAIモデル | `gpt-4o` |
| `ANTHROPIC_API_KEY` | Anthropic APIキー | - |
| `ANTHROPIC_MODEL` | Anthropicモデル | `claude-sonnet-4-20250514` |
| `OLLAMA_BASE_URL` | OllamaサーバーURL | `http://localhost:11434` |
| `OLLAMA_MODEL` | Ollamaモデル | `llama3.2` |
| `SEMANTIC_SCHOLAR_API_KEY` | Semantic Scholar APIキー（オプション） | - |
| `LOG_LEVEL` | ログレベル | `info` |

## Transport

### stdio (標準入出力)

Claude Desktop等のCLIツールとの統合用。

```bash
# 起動例
./academic-paper-interpreter-mcp
```

## モジュール構成

```
src/
├── bin/
│   └── app.rs              # エントリーポイント、CLI引数処理
├── server/
│   ├── mod.rs              # MCPサーバー本体
│   ├── tools/
│   │   ├── mod.rs
│   │   ├── interpret.rs    # interpret_paper ツール
│   │   ├── search.rs       # search_papers ツール
│   │   ├── fetch.rs        # fetch_paper ツール
│   │   └── analyze.rs      # analyze_paper ツール
│   └── transport/
│       ├── mod.rs
│       └── stdio.rs        # stdio transport
├── llm/
│   ├── mod.rs              # LLMプロバイダー管理
│   └── config.rs           # 設定解決ロジック
└── models/
    ├── mod.rs
    ├── paper.rs            # 論文データ構造
    ├── analysis.rs         # 解析結果データ構造
    └── request.rs          # ツールリクエスト構造

shared/
├── src/
│   ├── lib.rs
│   ├── errors.rs           # エラー型定義
│   ├── logger.rs           # ロギング設定
│   └── utils.rs            # ユーティリティ
```

## エラーハンドリング

### エラー種別

| エラーコード | 説明 |
|--------------|------|
| `PAPER_NOT_FOUND` | 指定された論文が見つからない |
| `INVALID_ARXIV_ID` | arXiv IDの形式が不正 |
| `PDF_FETCH_FAILED` | PDF取得に失敗 |
| `LLM_ERROR` | LLM API呼び出しエラー |
| `LLM_CONFIG_ERROR` | LLM設定エラー（APIキー未設定等） |
| `RATE_LIMIT` | APIレート制限 |
| `NETWORK_ERROR` | ネットワークエラー |

### エラーレスポンス形式

```json
{
  "error": {
    "code": "PAPER_NOT_FOUND",
    "message": "Paper with arXiv ID '2301.00001' not found",
    "details": {
      "arxiv_id": "2301.00001",
      "searched_sources": ["arxiv", "semantic_scholar"]
    }
  }
}
```

## 依存クレート

```toml
[dependencies]
# MCP
rmcp = { version = "0.9", features = ["server", "transport-io"] }

# 論文解析
academic-paper-interpreter = { git = "..." }

# シリアライゼーション
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
schemars = "0.8"  # JSON Schema生成

# 非同期
tokio = { version = "1", features = ["full"] }

# エラー処理
thiserror = "2.0"
anyhow = "1.0"

# ロギング
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }

# CLI
clap = { version = "4", features = ["derive"] }
```

## 設計原則

1. **ツールの冪等性**: 同じ入力に対して同じ結果を返す
2. **グレースフルデグラデーション**: LLMエラー時もメタデータは返す
3. **設定の柔軟性**: 環境変数 < ツールパラメータの優先順位
4. **ストリーミング対応**: 長時間処理の進捗通知（将来拡張）
