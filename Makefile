.PHONY: all
all:

.PHONY: dev-up
dev-up:
	docker-compose --file dev-stack.yaml up --detach

.PHONY: dev-down
dev-down:
	docker-compose --file dev-stack.yaml down
