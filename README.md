# GongZuo Infra

### 準備

```bash
cargo install cargo-make
cp .env.example .env
cargo make dotenv
# admin userのpasswordを聞かれる
cargo make make_user
```

### 開発環境での DB の接続

```bash
psql -U root -h 127.0.0.1 -p 11468 -d gongzuo
```

### web_backend に curl でリクエストする

```bash
# cargo install cargo-watch
cargo watch -x run
```

```bash
curl http://localhost:3000
```
