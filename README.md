# Academic Paper Interpreter MCP Server

MCP (Model Context Protocol) サーバーとして動作する学術論文解析ツール。論文の検索、取得、LLMによる解析機能を提供します。

## 機能

- **論文検索**: キーワード、著者、カテゴリによる学術論文検索
- **論文取得**: arXiv ID または URL から論文メタデータとPDFコンテンツを取得
- **論文解析**: LLM (OpenAI / Anthropic / Ollama) による要約・洞察生成
- **統合ツール**: 取得から解析までを一括実行

## インストール

### ビルド

```bash
cargo build --release
```

バイナリは `target/release/academic-paper-interpreter-mcp` に生成されます。

### 依存関係

- Rust 2024 edition
- OpenCV (PDF処理用)

## 設定

### 環境変数

| 変数名 | 説明 | デフォルト |
|--------|------|-----------|
| `LLM_PROVIDER` | 使用するLLMプロバイダー | `openai` |
| `OPENAI_API_KEY` | OpenAI APIキー | - |
| `OPENAI_MODEL` | OpenAIモデル名 | `gpt-5.2-2025-12-11` |
| `ANTHROPIC_API_KEY` | Anthropic APIキー | - |
| `ANTHROPIC_MODEL` | Anthropicモデル名 | `claude-sonnet-4-20250514` |
| `OLLAMA_BASE_URL` | Ollama サーバーURL | `http://localhost:11434` |
| `OLLAMA_MODEL` | Ollamaモデル名 | `llama3.2` |

### 設定例

```bash
# OpenAI を使用する場合
export LLM_PROVIDER=openai
export OPENAI_API_KEY=sk-xxxxx

# Anthropic を使用する場合
export LLM_PROVIDER=anthropic
export ANTHROPIC_API_KEY=sk-ant-xxxxx

# Ollama を使用する場合
export LLM_PROVIDER=ollama
export OLLAMA_BASE_URL=http://localhost:11434
export OLLAMA_MODEL=llama3.2
```

## 使い方

### 起動

```bash
# stdio モード（デフォルト）- Claude Desktop 等のMCPクライアント向け
academic-paper-interpreter-mcp

# SSE モード - Web クライアント向け
academic-paper-interpreter-mcp --transport sse --port 18080

# ログレベル指定
academic-paper-interpreter-mcp --log-level debug
```

### コマンドラインオプション

| オプション | 短縮形 | 説明 | デフォルト |
|-----------|--------|------|-----------|
| `--transport` | `-t` | トランスポート種別 (`stdio`, `sse`) | `stdio` |
| `--port` | `-p` | SSEモード時のポート番号 | `18080` |
| `--log-level` | - | ログレベル | `info` |

### Claude Desktop での設定

`claude_desktop_config.json` に以下を追加:

#### stdio モード（ローカル実行）

Claude Desktop がプロセスを直接起動し、stdin/stdout で通信します。

```json
{
  "mcpServers": {
    "academic-paper-interpreter": {
      "command": "/path/to/academic-paper-interpreter-mcp",
      "args": [],
      "env": {
        "LLM_PROVIDER": "openai",
        "OPENAI_API_KEY": "sk-xxxxx"
      }
    }
  }
}
```

#### SSE モード（リモート/サービス接続）

別途起動したサーバーに SSE で接続します。サーバーは事前に起動しておく必要があります。

```json
{
  "mcpServers": {
    "academic-paper-interpreter": {
      "url": "http://localhost:18080/sse"
    }
  }
}
```

### システムサービスとして登録 (Ubuntu/systemd)

SSE モードでバックグラウンドサービスとして常時起動する場合の設定方法です。

#### 自動インストール

```bash
# インストールスクリプトを実行
sudo ./scripts/install-service.sh

# APIキーを設定
sudo vim /etc/academic-paper-interpreter-mcp/env

# サービスを起動
sudo systemctl start academic-paper-interpreter-mcp
```

#### 手動インストール

```bash
# 1. リリースビルド
cargo build --release

# 2. 環境設定ディレクトリを作成
sudo mkdir -p /etc/academic-paper-interpreter-mcp
sudo cp academic-paper-interpreter-mcp.env /etc/academic-paper-interpreter-mcp/env
sudo chmod 600 /etc/academic-paper-interpreter-mcp/env

# 3. APIキーを設定
sudo vim /etc/academic-paper-interpreter-mcp/env

# 4. サービスファイルをコピー
sudo cp academic-paper-interpreter-mcp.service /etc/systemd/system/
# EnvironmentFile 行のコメントを解除
sudo sed -i 's|# EnvironmentFile=/etc/academic-paper-interpreter-mcp/env|EnvironmentFile=/etc/academic-paper-interpreter-mcp/env|' \
    /etc/systemd/system/academic-paper-interpreter-mcp.service

# 5. サービスを有効化・起動
sudo systemctl daemon-reload
sudo systemctl enable academic-paper-interpreter-mcp
sudo systemctl start academic-paper-interpreter-mcp
```

#### サービス管理コマンド

```bash
# 状態確認
sudo systemctl status academic-paper-interpreter-mcp

# ログ確認 (リアルタイム)
sudo journalctl -u academic-paper-interpreter-mcp -f

# 再起動
sudo systemctl restart academic-paper-interpreter-mcp

# 停止
sudo systemctl stop academic-paper-interpreter-mcp

# 無効化
sudo systemctl disable academic-paper-interpreter-mcp
```

#### アンインストール

```bash
sudo ./scripts/uninstall-service.sh
```

#### サービス設定のカスタマイズ

`/etc/systemd/system/academic-paper-interpreter-mcp.service` を編集してカスタマイズできます:

| 設定項目 | 説明 | デフォルト |
|----------|------|-----------|
| `ExecStart` | 起動コマンド (ポート番号等) | `--port 18080` |
| `User` / `Group` | 実行ユーザー | `kucho` |
| `MemoryMax` | メモリ上限 | `2G` |
| `CPUQuota` | CPU使用率上限 | `200%` |
| `Restart` | 再起動ポリシー | `on-failure` |

変更後は以下を実行:
```bash
sudo systemctl daemon-reload
sudo systemctl restart academic-paper-interpreter-mcp
```

## MCP ツール仕様

### 1. search_papers

キーワード、著者、カテゴリで論文を検索します。

**パラメータ:**

| 名前 | 型 | 必須 | 説明 |
|------|-----|------|------|
| `query` | string | Yes | 検索キーワード |
| `author` | string | No | 著者名フィルタ |
| `category` | string | No | arXivカテゴリ (例: `cs.CL`, `cs.AI`) |
| `max_results` | number | No | 最大件数 (デフォルト: 10) |

**使用例:**

```json
{
  "query": "transformer attention mechanism",
  "category": "cs.CL",
  "max_results": 5
}
```

**レスポンス:**

```json
{
  "papers": [
    {
      "title": "Attention Is All You Need",
      "authors": ["Ashish Vaswani", "..."],
      "abstract": "The dominant sequence transduction models...",
      "arxiv_id": "1706.03762",
      "published_date": "2017-06-12T00:00:00Z",
      "pdf_url": "https://arxiv.org/pdf/1706.03762.pdf"
    }
  ],
  "total_count": 1
}
```

### 2. fetch_paper

arXiv ID または URL から論文データを取得します。

**パラメータ:**

| 名前 | 型 | 必須 | 説明 |
|------|-----|------|------|
| `arxiv_id` | string | No* | arXiv ID (例: `2301.00001`) |
| `url` | string | No* | 論文URL |
| `include_pdf_content` | boolean | No | PDFコンテンツを含める (デフォルト: true) |

*`arxiv_id` または `url` のいずれか必須

**使用例:**

```json
{
  "arxiv_id": "1706.03762",
  "include_pdf_content": true
}
```

**レスポンス:**

```json
{
  "paper": {
    "title": "Attention Is All You Need",
    "authors": ["Ashish Vaswani", "..."],
    "abstract": "The dominant sequence transduction models...",
    "arxiv_id": "1706.03762",
    "ss_id": "12345678",
    "categories": ["cs.CL", "cs.LG"],
    "published_date": "2017-06-12T00:00:00Z",
    "pdf_url": "https://arxiv.org/pdf/1706.03762.pdf",
    "content": "Full paper text extracted from PDF..."
  }
}
```

### 3. analyze_paper

LLMを使用して論文を解析し、要約と洞察を生成します。

**パラメータ:**

| 名前 | 型 | 必須 | 説明 |
|------|-----|------|------|
| `paper` | Paper | Yes | fetch_paper で取得した論文データ |
| `llm_config` | LlmConfig | No | LLM設定 (環境変数のデフォルト使用可) |
| `analysis_type` | string | No | 解析タイプ: `summary`, `detailed`, `comparison` |

**LlmConfig:**

| 名前 | 型 | 説明 |
|------|-----|------|
| `provider` | string | `openai`, `anthropic`, `ollama` |
| `model` | string | モデル名 (省略時は環境変数から) |

**使用例:**

```json
{
  "paper": { "...": "fetch_paper の結果" },
  "llm_config": {
    "provider": "anthropic",
    "model": "claude-sonnet-4-20250514"
  },
  "analysis_type": "summary"
}
```

**レスポンス:**

```json
{
  "analysis": {
    "summary": "This paper introduces the Transformer architecture...",
    "key_contributions": [
      "Novel self-attention mechanism",
      "Parallelizable architecture",
      "State-of-the-art results on translation tasks"
    ],
    "methodology": "The authors propose a model architecture based entirely on attention mechanisms...",
    "limitations": [
      "Quadratic complexity with sequence length"
    ],
    "related_work": []
  }
}
```

### 4. interpret_paper

論文の取得と解析を一括で実行する統合ツールです。

**パラメータ:**

| 名前 | 型 | 必須 | 説明 |
|------|-----|------|------|
| `query.title` | string | No* | タイトルで検索 |
| `query.url` | string | No* | 論文URL |
| `query.pdf_url` | string | No* | PDF URL |
| `query.arxiv_id` | string | No* | arXiv ID |
| `llm_config` | LlmConfig | No | LLM設定 |

*`query` 内のいずれか1つ以上必須

**使用例:**

```json
{
  "query": {
    "arxiv_id": "1706.03762"
  },
  "llm_config": {
    "provider": "openai"
  }
}
```

**レスポンス:**

```json
{
  "paper": {
    "title": "Attention Is All You Need",
    "...": "..."
  },
  "analysis": {
    "summary": "...",
    "key_contributions": ["..."],
    "...": "..."
  }
}
```

## エラーハンドリング

### エラーコード

| コード | 説明 |
|--------|------|
| `PAPER_NOT_FOUND` | 指定された論文が見つからない |
| `INVALID_ARXIV_ID` | 無効なarXiv ID形式 |
| `PDF_FETCH_FAILED` | PDFの取得に失敗 |
| `LLM_ERROR` | LLM処理中のエラー |
| `LLM_CONFIG_ERROR` | LLM設定エラー (APIキー未設定等) |
| `RATE_LIMIT_EXCEEDED` | APIレート制限超過 |
| `NETWORK_ERROR` | ネットワーク接続エラー |
| `INVALID_REQUEST` | 無効なリクエストパラメータ |

### エラーレスポンス例

```json
{
  "error": {
    "code": "LLM_CONFIG_ERROR",
    "message": "API key not found for provider: openai",
    "details": null
  }
}
```

## アーキテクチャ

```
src/
├── bin/app.rs           # CLIエントリーポイント
├── lib.rs               # ライブラリルート
├── models/              # データモデル
│   ├── paper.rs         # Paper, PaperSummary
│   ├── analysis.rs      # PaperAnalysis, AnalysisType
│   ├── llm_config.rs    # LlmConfig, LlmProvider
│   ├── request.rs       # リクエスト型
│   └── response.rs      # レスポンス型
├── llm/                 # LLMプロバイダー管理
│   ├── config.rs        # LlmConfigResolver
│   └── provider.rs      # AnalyzerType, create_analyzer
└── server/              # MCPサーバー
    ├── handler.rs       # PaperInterpreterService
    ├── tools/           # MCPツール実装
    │   ├── search.rs
    │   ├── fetch.rs
    │   ├── analyze.rs
    │   └── interpret.rs
    └── transport/       # トランスポート層
        ├── stdio.rs
        └── sse.rs
```

## ライセンス

MIT
