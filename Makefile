# Set a default FILE to be safe
# principle of least surprise
FILE=exercise-stack.yaml

# Do nothing if make is called w/o targets
.PHONY: all
all:

# Build the web server
.PHONY: docker-build
docker-build:
	docker build --tag thecjharries/dso_dojo_slow_postgres:latest .

# Run the dev environment
.PHONY: dev
dev:
	$(MAKE) docker-compose FILE=dev-stack.yaml

# Run the exercise environment
.PHONY: exercise
exercise: docker-build
	$(MAKE) docker-compose FILE=exercise-stack.yaml

# Convenience target to toggle specific envs
.PHONY: docker-compose
docker-compose:
	if docker-compose ls | grep -q $(FILE); then \
		docker-compose --file $(FILE) down; \
	else \
		docker-compose --file $(FILE) up --detach; \
	fi
