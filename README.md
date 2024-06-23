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
- GET `/api/cronograma/user`
- GET `/api/groups/user`
- GET `/api/groups`
- POST `/api/groups/new` - requires `timer_group_id, name, timers[{timer_id, name, seconds}]` returns the new `Group{timer_group_id, is_owner, name, timers[]}`
- POST `/api/user/add-group` - requires `group_id`, returns the group added to the user

