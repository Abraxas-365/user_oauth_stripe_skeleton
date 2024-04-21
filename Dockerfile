ARG BASE_IMAGE=rust:1.75

FROM $BASE_IMAGE as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM $BASE_IMAGE as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM $BASE_IMAGE as builder
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME

ARG DATABASE_URL
RUN DATABASE_URL=$DATABASE_URL cargo build --release

FROM $BASE_IMAGE
COPY --from=builder /app/target/release/user_oauth_stripe_skeleton ./
CMD ["./user_oauth_stripe_skeleton"]
