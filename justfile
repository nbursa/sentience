default:
  just run

run:
  go run ./cmd/main.go

test:
  go test ./...

fmt:
  go fmt ./...
