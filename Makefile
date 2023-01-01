.PHONY: all
all:

.PHONY: dev-up
dev-up:
	docker-compose --file dev-stack.yaml up --detach

.PHONY: dev-down
dev-down:
	docker-compose --file dev-stack.yaml down

.PHONY: docker-build
docker-build:
	docker build --tag thecjharries/dso_dojo_slow_postgres:latest .

.PHONY: exercise-up
exercise-up:
	docker-compose --file exercise-stack.yaml up --detach

.PHONY: exercise-down
exercise-down:
	docker-compose --file exercise-stack.yaml down
