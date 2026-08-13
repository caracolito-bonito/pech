# pech 🔥

Учебная serverless-платформа (аля AWS Lambda) на Rust: функции на [Rhai](https://rhai.rs), API на Axum.

## План

### Этап 0 — Монолит-MVP ⬅️

- [x] `GET /health` — пинг-сервер на axum
- [x] CRUD функций в памяти (`HashMap` за `Arc<RwLock>`)
- [ ] `POST /functions/{id}/run` — исполнение через `rhai::Engine`
- [ ] Авторизация по захардкоженному токену

### Этап 1 — Живучесть

- [ ] SQLite (`sqlx`) + миграции
- [ ] Юзеры и токены в базе
- [ ] Auth-middleware

### Этап 2 — Укрощение рантайма

- [ ] Таймауты исполнения
- [ ] Лимиты Rhai (операции, глубина, память)
- [ ] `spawn_blocking` для синхронного Rhai
- [ ] Кэш скомпилированных AST (warm start)

### Этап 3 — Control plane и воркеры

- [ ] Workspace: бинарь API-ноды + бинарь воркера
- [ ] HTTP-протокол между ними
- [ ] Round-robin по воркерам

### Этап 4 — Redis

- [ ] Pub/sub для обновлений функций
- [ ] Кэш функций на воркерах + инвалидация
- [ ] Метрики cold/warm start

### Этап 5 — Умный роутинг

- [ ] Мапа «какие функции в памяти каких нод»
- [ ] Sticky routing / consistent hashing
- [ ] Вытеснение холодных функций

### Этап 6 — WASM

- [ ] Трейт `Runtime`: Rhai + `wasmtime`
- [ ] Тип рантайма — свойство функции

### Этап 7 — Для агентов

- [ ] MCP-сервер поверх API

## Запуск

```sh
cargo run
curl localhost:3000/health
```
