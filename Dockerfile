FROM golang:1.10

WORKDIR /go/src/github.com/hammatt/tougou_bot
COPY . .

RUN gofmt -s -w .
RUN go get -u golang.org/x/lint/golint
RUN golint ./...
RUN go get -u golang.org/x/vgo
RUN vgo test ./...
RUN vgo build -o build/tougou_bot  cmd/tougou_bot/main.go

ENTRYPOINT ["build/tougou_bot"]
