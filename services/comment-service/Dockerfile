FROM golang:1.19-bullseye

WORKDIR /app

COPY ./src/go.mod .
COPY ./src/go.sum .

RUN go get ./...

COPY ./src .

ENTRYPOINT go run .