# CRONO API
Rust API with axum + jwt + sqlx deployed on shuttle.

This currently provides:
- Axum API REST
- JWT authentication
- Database interaction
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
- POST `/api/user/login` - requires `email, password`, returns tokenPayload {`access_token, token_type, user_name, user_id`}
- GET `/api/timers/group/:id`
- GET `/api/timers/cronograma/:id`
- GET `/api/cronograma/user/:id`
- GET `/api/groups/user/:id`
- GET `/api/groups`
- POST `/api/user/add-group` - requires `user_id, group_id`, returns the group added to the user

