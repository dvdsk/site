setup:
	./scripts/setup.sh

preview:
	(cd public && hugo server -D)

deploy:
	./scripts/deploy.sh


