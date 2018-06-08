FROM golang:1.10

WORKDIR /go/src/github.com/hammatt/tougou_bot
COPY . .

RUN gofmt -s -w .
RUN go build -o build/tougou_bot  cmd/tougou_bot/main.go

CMD ["main"]
