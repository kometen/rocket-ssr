FROM rust

WORKDIR /usr/src/rocket
COPY . .

RUN cargo install --path .
EXPOSE 8008

CMD ["rocket-ssr"]
