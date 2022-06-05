dev: dev-up

dev-up:
	docker-compose -f docker-compose.yml up --build -d

dev-logs:
	docker-compose -f docker-compose.yml logs -f -t

dev-down:
	docker-compose -f docker-compose.yml down --remove-orphans
