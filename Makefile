.PHONY: docker-build docker-run docker-run-pulled docker-push docker-pull docker-down

docker-build:
	DOCKER_BUILDKIT=1 docker build -t uwrss:latest .

docker-run:
	docker run -d --restart unless-stopped --env-file .env --dns 8.8.8.8 uwrss

docker-run-pulled:
	docker run -d --restart unless-stopped --env-file .env --dns 8.8.8.8 jiztastamablastamarang/uwrss:latest

docker-push:
	docker login
	docker tag uwrss:latest jiztastamablastamarang/uwrss:latest
	docker push jiztastamablastamarang/uwrss:latest

docker-pull:
	docker login
	docker pull jiztastamablastamarang/uwrss:latest

docker-down:
	docker stop $$(docker ps -a -q --filter name=uwrss)
	docker rm $$(docker ps -a -q --filter name=uwrss)