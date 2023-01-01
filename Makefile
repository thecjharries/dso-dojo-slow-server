FILE=exercise-stack.yaml

.PHONY: all
all:

.PHONY: docker-build
docker-build:
	docker build --tag thecjharries/dso_dojo_slow_postgres:latest .

.PHONY: dev
dev:
	$(MAKE) docker-compose FILE=dev-stack.yaml

.PHONY: exercise
exercise: docker-build
	$(MAKE) docker-compose FILE=exercise-stack.yaml

.PHONY: docker-compose
docker-compose:
	if docker-compose ls | grep -q $(FILE); then \
		docker-compose --file $(FILE) down; \
	else \
		docker-compose --file $(FILE) up --detach; \
	fi
