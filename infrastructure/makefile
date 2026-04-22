NETWORK=centralize-network

COMPOSE_DEV=docker compose \
	-f dev/docker-compose.solana.dev.yml \
	-f dev/docker-compose.dev.yml 

.PHONY: up down logs network
 
network:
	@docker network inspect $(NETWORK) >/dev/null 2>&1 || docker network create $(NETWORK)

down:
	$(COMPOSE) down

stop:
	$(COMPOSE) stop

logs: 
	$(COMPOSE) logs -f

dev: network 
	$(COMPOSE_DEV) up -d

down-dev:
	$(COMPOSE_DEV) down