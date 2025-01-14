GOOS = $(shell go env GOOS)
GOARCH = $(shell go env GOARCH)
BUILD_DIR = dist/${GOOS}_${GOARCH}

build:
	go build ./cmd/server

update-deps:
	go get -d -u ./...
	go mod tidy -v
	go mod vendor

lint:
	golangci-lint run
