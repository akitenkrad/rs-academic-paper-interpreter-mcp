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

#### カスタムコネクタ（リモート MCP サーバー接続）

Claude Desktop / claude.ai では **カスタムコネクタ** 機能でリモート MCP サーバーに接続できます。

> **要件:** Pro, Max, Team, または Enterprise プランが必要です。

**設定手順:**

1. Claude Desktop または claude.ai で **Settings > Connectors** を開く
2. **Add custom connector** をクリック
3. リモート MCP サーバーの URL を入力:
   ```
   https://your-server-domain:18080/sse
   ```
4. 必要に応じて **Advanced settings** で OAuth 認証情報を設定
5. **Add** をクリックして完了

**重要な制限事項:**

- URL は `https://` である必要があります（`http://` は不可）
- ローカルホスト (`localhost`, `127.0.0.1`) は使用できません

**HTTPS 化の方法:**

| 環境 | 推奨方法 |
|------|----------|
| ローカル / 家庭内ネットワーク | ngrok（後述） |
| 本番環境（公開ドメインあり） | nginx + Let's Encrypt（後述） |

### ngrok で HTTPS 化（ローカル環境）

公開ドメインがなくても、ngrok を使用して HTTPS URL を取得できます。

#### 1. ngrok のインストール

**Ubuntu/Debian:**
```bash
curl -s https://ngrok-agent.s3.amazonaws.com/ngrok.asc | \
  sudo tee /etc/apt/trusted.gpg.d/ngrok.asc >/dev/null
echo "deb https://ngrok-agent.s3.amazonaws.com buster main" | \
  sudo tee /etc/apt/sources.list.d/ngrok.list
sudo apt update && sudo apt install ngrok
```

**macOS:**
```bash
brew install ngrok
```

#### 2. ngrok アカウント設定

1. [ngrok.com](https://ngrok.com/) で無料アカウントを作成
2. ダッシュボードから Authtoken を取得
3. トークンを設定：

```bash
ngrok config add-authtoken <your-authtoken>
```

#### 3. MCP サーバーを公開

```bash
# MCP サーバーが起動していることを確認
sudo systemctl status academic-paper-interpreter-mcp

# ngrok でトンネルを作成
ngrok http 18080
```

出力例：
```
Session Status                online
Forwarding                    https://abc123.ngrok-free.app -> http://localhost:18080
```

#### 4. Claude Desktop に登録

1. **Settings > Connectors** を開く
2. **Add custom connector** をクリック
3. ngrok の URL + `/sse` を入力：
   ```
   https://abc123.ngrok-free.app/sse
   ```
4. **Add** をクリック

#### ngrok をバックグラウンドで実行

```bash
# バックグラウンド実行
nohup ngrok http 18080 > /dev/null 2>&1 &

# 現在の URL を確認
curl -s http://localhost:4040/api/tunnels | jq -r '.tunnels[0].public_url'
```

#### ngrok の注意点

| 項目 | 無料プラン | 有料プラン |
|------|-----------|-----------|
| URL | 起動ごとに変更 | 固定 URL 可能 |
| 接続数 | 制限あり | 制限緩和 |
| 帯域 | 制限あり | 制限緩和 |

> **Tip:** 有料プランでは固定ドメイン（例: `mcp.ngrok.io`）を設定できます。

### Cloudflare Tunnel で HTTPS 化（ローカル環境・固定 URL）

Cloudflare Tunnel を使用すると、無料で固定 URL を取得できます。

#### 1. cloudflared のインストール

**Ubuntu/Debian:**
```bash
curl -L --output cloudflared.deb \
  https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-amd64.deb
sudo dpkg -i cloudflared.deb
```

**macOS:**
```bash
brew install cloudflared
```

#### 2. Cloudflare アカウント設定

1. [Cloudflare](https://dash.cloudflare.com/) で無料アカウントを作成
2. cloudflared を認証：

```bash
cloudflared tunnel login
# ブラウザが開くので、Cloudflare にログイン
```

#### 3. トンネルの作成

```bash
# トンネルを作成
cloudflared tunnel create mcp-server

# 作成されたトンネル ID を確認
cloudflared tunnel list
```

#### 4. 設定ファイルの作成

```bash
mkdir -p ~/.cloudflared
cat > ~/.cloudflared/config.yml << 'EOF'
tunnel: <tunnel-id>
credentials-file: /home/<username>/.cloudflared/<tunnel-id>.json

ingress:
  - hostname: mcp-server.example.com
    service: http://localhost:18080
  - service: http_status:404
EOF
```

> **注意:** `<tunnel-id>` と `<username>` を実際の値に置き換えてください。

#### 5. DNS ルーティングの設定

Cloudflare に登録済みのドメインがある場合：
```bash
cloudflared tunnel route dns mcp-server mcp-server.example.com
```

ドメインがない場合は、Cloudflare の無料サブドメインを使用：
```bash
# Quick Tunnel（一時的な URL、設定不要）
cloudflared tunnel --url http://localhost:18080
```

出力例：
```
Your quick Tunnel has been created! Visit it at:
https://random-name-here.trycloudflare.com
```

#### 6. トンネルの実行

```bash
# フォアグラウンドで実行
cloudflared tunnel run mcp-server

# または Quick Tunnel（設定不要）
cloudflared tunnel --url http://localhost:18080
```

#### 7. Claude Desktop に登録

カスタムコネクタ URL：
```
https://mcp-server.example.com/sse
# または Quick Tunnel の場合
https://random-name-here.trycloudflare.com/sse
```

#### systemd でサービス化

```bash
sudo cloudflared service install
sudo systemctl enable cloudflared
sudo systemctl start cloudflared
```

#### ngrok vs Cloudflare Tunnel

| 項目 | ngrok（無料） | Cloudflare Tunnel |
|------|--------------|-------------------|
| 固定 URL | ✗ | ✓（独自ドメイン） |
| Quick Tunnel | - | ✓（一時 URL） |
| 帯域制限 | あり | なし |
| 設定の手軽さ | ◎ | ○ |
| 料金 | 無料〜 | 無料 |

> **推奨:** 固定 URL が必要な場合は Cloudflare Tunnel、手軽さ重視なら ngrok を使用してください。

### nginx + Let's Encrypt で HTTPS 化（本番環境）

公開ドメインを持つサーバーで SSL 化する方法です。

#### 前提条件

- 公開ドメイン（DNS で A レコードがサーバーに向いていること）
- ポート 80, 443 が開放されていること

#### 自動セットアップ

```bash
sudo ./scripts/setup-nginx-ssl.sh mcp.example.com admin@example.com
```

このスクリプトは以下を実行します：
1. nginx と certbot のインストール
2. Let's Encrypt から SSL 証明書を取得
3. nginx の設定（SSE 対応のリバースプロキシ）
4. 証明書の自動更新設定

#### 手動セットアップ

```bash
# 1. nginx と certbot をインストール
sudo apt-get update
sudo apt-get install -y nginx certbot python3-certbot-nginx

# 2. nginx 設定をコピー（YOUR_DOMAIN を置換）
sudo cp nginx/mcp-server.conf /etc/nginx/sites-available/
sudo sed -i 's/YOUR_DOMAIN/mcp.example.com/g' /etc/nginx/sites-available/mcp-server.conf
sudo ln -s /etc/nginx/sites-available/mcp-server.conf /etc/nginx/sites-enabled/

# 3. SSL 証明書を取得
sudo certbot --nginx -d mcp.example.com

# 4. nginx を再起動
sudo systemctl restart nginx
```

#### 設定確認

```bash
# ヘルスチェック
curl -I https://mcp.example.com/health

# SSE エンドポイント確認
curl -N https://mcp.example.com/sse
```

#### Claude Desktop での登録

カスタムコネクタ URL:
```
https://mcp.example.com/sse
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
