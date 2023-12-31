PROJECT_NAME=$(notdir $(CURDIR))
CONTAINER_PATH=$(CURDIR)/build/docker-compose.yml

workspace: exit
	docker-compose -f $(CONTAINER_PATH) up -d workspace
	docker-compose -f $(CONTAINER_PATH) exec workspace bash

exit:
	docker-compose -f $(CONTAINER_PATH) down --remove-orphans --volumes
	docker rmi -f workspace