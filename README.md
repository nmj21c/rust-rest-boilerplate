# Rust-Rest-Boilerplate
A simple CRUD backend app using Axum, Sqlx, Validator & JWT


## 2401 추가
.env.example -> .env 로 수정
.env 에 PORT=3000 추가 (cors 랑 맞춰줌)

## postgres 실행
```
docker compose up -d
```

## rust server 실행
```
cargo run
```

## 실행
- adminer (postgres 관리)
```
http://localhost:9000
```

- rust server
```
http://localhost:3000
```