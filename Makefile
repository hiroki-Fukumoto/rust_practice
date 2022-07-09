watch:
	cargo watch -x run

M=
migrate-generate:
	diesel migration generate $(M)

migrate-run:
	diesel migration run
