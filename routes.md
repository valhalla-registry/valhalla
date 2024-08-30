# Routes

## Frontend /

```
GET /
GET /me
GET /search
GET /crates/:crate
GET /crates/:crate/docs
GET /crates/:crate/docs/:version

GET /account/login
GET /account/register
GET /account/manage

GET /assets
```

## /api/v1

```
POST /account/login
POST /account/register
POST PUT DELETE /account/tokens
GET /account/tokens/:name

GET /categories

GET /crates
PUT /crates/new
GET /crates/suggest
GET /crates/:name
GET PUT DELETE /crates/:name/owners
DELETE /crates/:name/:version/yank
PUT /crates/:name/:version/unyank
GET /crates/:name/:version/download

GET /index
GET /index/git
GET /index/sparse
```
