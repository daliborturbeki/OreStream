# OreStream

A production-grade Rust backend that streams live Solana Ore mining data in real time.

Built to learn and demonstrate: async Rust, real-time data pipelines, REST + WebSocket APIs, Docker, and Kubernetes.

## What it does

Connects to the Solana Ore mining program via [Arete](https://arete.run), processes typed on-chain events, and serves them through a REST API and live WebSocket feed — backed by Postgres (historical storage) and Redis (hot cache + pub/sub).

## Stack

| Layer | Technology |
|---|---|
| Language | Rust (async, Tokio) |
| HTTP / WebSocket | Axum 0.8 |
| Solana streaming | Arete |
| Database | PostgreSQL + SQLx |
| Cache | Redis |
| Observability | Prometheus + tracing |
| DevOps | Docker Compose + Kubernetes |

## Architecture

```
Solana (Ore Program)
    └─> Arete (typed stream)
            ├─> Redis  (hot cache + pub/sub)
            └─> Postgres (historical data)
                    └─> Axum (REST + WebSocket)
                                └─> Clients
```

## Phases

1. **Spike** — prove the stream (Arete events printing to stdout)
2. **Core API** — Axum server, `/api/rounds/current`, `/ws/live`
3. **Data Layer** — Postgres + Redis
4. **Workers** — background aggregation
5. **Docker** — multi-stage build + Compose
6. **Kubernetes** — full manifest set
7. **Polish** — CI, Prometheus, README, live demo

## Running locally

```bash
docker compose up
```

## Status

Active development.
