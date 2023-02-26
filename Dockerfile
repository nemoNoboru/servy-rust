from rust:latest as build
workdir /app
copy . /app

run cargo test --verbose
run cargo build --release



from debian:bookworm as final
workdir /
copy --from=build /app/target/release/servy /
copy build /build
EXPOSE 3000

CMD [ "./servy" ]