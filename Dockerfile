ARG BASE_IMAGE=rust:1.70
# stage 1 generate a recipe for dependencies
FROM $BASE_IMAGE as planner

# 设置 RUST_BACKTRACE=1 环境变量
ENV RUST_BACKTRACE=1

ADD ./.cargo $CARGO_HOME/
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# stage 2 build our dependencies
FROM $BASE_IMAGE as cacher
ADD ./.cargo $CARGO_HOME/
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# stage 3 use the main official rust docker image
FROM $BASE_IMAGE as builder
ADD ./.cargo $CARGO_HOME/
COPY . /app
WORKDIR /app

# COPY dependencies
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME

RUN cargo build --release

FROM $BASE_IMAGE

COPY --from=builder /app/target/release/rust-init-demo /app/rust-init-demo
COPY configs /app/configs

WORKDIR /app

# ENV ROCKET_ADDRESS=0.0.0.0
# EXPOSE 8000

CMD ["./rust-init-demo"]