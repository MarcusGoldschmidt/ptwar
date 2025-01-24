GO ?= go
GOPATH ?= $(shell go env GOPATH)

VERSION=0.0.1
V_COMMIT := $(shell git rev-parse HEAD)
V_BUILT_BY := $(shell git config user.email)
V_BUILT_AT := $(shell date)

V_LDFLAGS_COMMON := -s \
					-X "github.com/MarcusGoldschmidt/ptwar/pkg.Version=${VERSION}" \
					-X "github.com/MarcusGoldschmidt/ptwar/pkg.Commit=${V_COMMIT}" \
					-X "github.com/MarcusGoldschmidt/ptwar/pkg.BuiltBy=${V_BUILT_BY}" \
					-X "github.com/MarcusGoldschmidt/ptwar/pkg.BuiltAt=${V_BUILT_AT}"

.PHONY: install-deps
install-deps:
	go install golang.org/x/tools/cmd/stringer@latest

.PHONY: generate
generate:
	go generate ./...

.PHONY: build
build: generate
	CGO_ENABLED=0 $(GO) build -v -ldflags '$(V_LDFLAGS_COMMON)' ./cmd/server

.PHONY: update-deps
update-deps:
	go get -u ./...
	go mod tidy -v
	go mod vendor

.PHONY: clean
lint:
	golangci-lint run

.PHONY: test
test: generate
	go test -v ./...

.PHONY: run
run: generate
	$(GO) run -v -ldflags '$(V_LDFLAGS_COMMON)' ./cmd/server
