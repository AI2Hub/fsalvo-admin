FROM debian:buster-slim

ENV TZ Asia/Shanghai

WORKDIR /app

COPY ./src/config/log4rs.yaml /app/src/config/log4rs.yaml
COPY ./config.toml /app/config.toml
COPY ./target/release/salvo-admin /app/

CMD ["./salvo-admin"]