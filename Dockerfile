FROM rust:1.62.1-bullseye as builder
# rust part
COPY . /app
WORKDIR /app
RUN cargo install --path .
# vue part
RUN curl -sL https://deb.nodesource.com/setup_18.x | bash -
RUN apt install -y nodejs
RUN npm install -g pnpm
RUN mkdir /vue
WORKDIR /vue
RUN git clone https://github.com/quiode/ela-website.git .
RUN pnpm install
RUN pnpm run build
RUN mv dist/* /app/static/
WORKDIR /app

FROM debian:bullseye-slim as runner
RUN mkdir -p /app
COPY --from=builder /usr/local/cargo/bin/ela-api /usr/local/bin/ela-api
COPY --from=builder /app/static /app/static
COPY Rocket.toml /app/
WORKDIR /app
ENV ROCKET_ADDRESS=0.0.0.0
# database
RUN mkdir /var/lib/ela
COPY template.sqlite /var/lib/ela/db.sqlite
VOLUME "/var/lib/ela/"
# metadata
LABEL description="API for ELA (easy logging application)"
LABEL org.opencontainers.image.authors="mail@dominik-schwaiger.ch"
# metadata
EXPOSE 8000
CMD ["ela-api"]