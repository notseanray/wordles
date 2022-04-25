golang:
	cd go && go build main.go
	mv go/main ./wordle
rust:
	cd rs && cargo build --release
	mv rs/target/release/wordle-rs ./wordle
