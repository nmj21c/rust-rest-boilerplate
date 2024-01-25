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

## test
- signon (post http://localhost:3000/api/v1/users/signon)
```
Header
  Content-Type : application/json
Body
  {
    "email: "test@naver.com",
    "name": "test",
    "password": "123123"
  }
```

- signin (post http://localhost:3000/api/v1/users/signin)
```
Header
  Content-Type : application/json
Body
  {
    "email: "test@naver.com",
    "password": "123123"
  }
```

- whoami (get http://localhost:3000/api/v1/users/whoami)
```
Header
  Content-Type : application/json
  Authorization : Bearer {signin res > access_token}
```

- catagories list (get http://localhost:3000/api/v1/categories)
```
Header
  Content-Type : application/json
  Authorization : Bearer {signin res > access_token}
```

- catagories insert (get http://localhost:3000/api/v1/categories)
```
Header
  Content-Type : application/json
  Authorization : Bearer {signin res > access_token}
Body
  {
    "name": "test",
    "cat_type": "NonEssential"
  }
```