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

### local deployment:

```cargo shuttle run```

### API
- POST `/api/user/register`
- POST `/api/user/login`
- GET `/api/timers/group/:id`
- GET `/api/timers/cronograma/:id`
- GET `/api/cronograma/user/:id`

