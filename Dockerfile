FROM golang:1.10 AS build-env

WORKDIR /go/src/github.com/hammatt/tougou_bot
COPY . .

RUN gofmt -s -w .
RUN go get -u golang.org/x/lint/golint
RUN golint ./...
RUN go get -u golang.org/x/vgo
RUN CGO_ENABLED=0 vgo test ./...
RUN CGO_ENABLED=0 GOOS=linux vgo build -ldflags "-s" -a -installsuffix cgo -o build/tougou_bot  cmd/tougou_bot/main.go

FROM alpine:latest
WORKDIR /app
RUN apk add --no-cache ca-certificates
COPY --from=build-env go/src/github.com/hammatt/tougou_bot/build/tougou_bot /app/

ENTRYPOINT ["./tougou_bot"]
