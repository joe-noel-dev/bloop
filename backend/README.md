# Backend

PocketBase backend for storing projects and samples.

## Start

```sh
./start.sh
```

This will:
- Build the Docker image
- Start the PocketBase server on port 8080
- Restore from backup if no existing data found

## Access

- Admin UI: http://localhost:8080/_/admin
- API endpoint: http://localhost:8080

## Management

View logs:
```sh
docker logs -f bloop-backend-dev
```

Stop:
```sh
docker stop bloop-backend-dev
```
