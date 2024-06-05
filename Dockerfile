from rust
workdir /app
copy . .
run cargo build --release
