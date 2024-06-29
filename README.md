# CRONO API
Rust API with axum + jwt + sqlx deployed on shuttle.

This currently provides:
- Axum API REST
- JWT authentication
- Database interaction
- Social login
- Password encryption
- Setup for a Shuttle deployment

---
## USAGE
by default, shuttle, when running on local, will try to launch a Docker container with the configured database inside of it, you must have docker installed for this to succeed.

### Local deployment:
- You must have rust installed.


```sh
cargo install cargo-shuttle
```

```sh
cargo shuttle run
```

### API
- POST `/api/user/register` - requires `name, email, password`
- POST `/api/user/login` - requires `email, password`, returns tokenPayload {`access_token, token_type, user_name`}
- POST `/api/user/social-login` - requires `name, email`, return tokenPayload

Require authorization:
- GET `/api/timers/group/:id`
- GET `/api/timers/cronograma/:id`
- GET `/api/cronograma/user` - returns all the user's cronogramas with their timers
- GET `/api/groups/user`- returns all the user's groups with their timers
- GET `/api/groups`
- POST `/api/groups/new` - requires `timer_group_id, name, timers[{timer_id, name, seconds}]` returns the new `Group{timer_group_id, is_owner, name, timers[]}`
- DELETE `/api/groups/delete` - requires `timer_group_id, name, owner, owner_name`
- POST `/api/user/add-group` - requires `group_id`, returns the group added to the user
- POST `/api/cronograma/new` - requires `cronograma_id, name`, returns the cronograma
- PUT `/api/cronograma/update` - requires `cronograma_id, name`, returns the cronograma
- DELETE `/api/cronograma/delete` - requires `cronograma_id, name`
- POST `/api/cronograma/add-timer` - requires `cronograma_id, timer: {name, seconds}` returns the timer
