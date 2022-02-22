.DEFAULT_GOAL := preview

setup:
	./scripts/setup.sh

preview:
	(cd public && hugo server -D)

deploy:
	./scripts/deploy_optimized.sh
