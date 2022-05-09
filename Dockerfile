FROM rust as build

WORKDIR /usr/src/tv-quotes-api
COPY . .

RUN SQLX_OFFLINE=1 cargo install --path .


FROM gcr.io/distroless/cc

COPY --from=build /usr/local/cargo/bin/tv-quotes-api /usr/local/bin/tv-quotes-api

EXPOSE 8080

CMD ["tv-quotes-api"]