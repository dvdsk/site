.DEFAULT_GOAL := preview

copy_cv:
	cp ../Jobs/CV/public/David\ Kleingeld\ CV.pdf public/public/about/cv.pdf
	cp ../Jobs/CV/body.tex scripts/about_page/body.tex

public/content/about.md: scripts/about_page/body.tex scripts/about_page/src/main.rs
	(cd scripts/about_page && cargo r)
	mv scripts/about_page/output.md public/content/about.md

setup:
	./scripts/setup.sh

preview: public/content/about.md
	(cd public && hugo server -D)

deploy: public/content/about.md
	./scripts/deploy_optimized.sh

.PHONY: copy_cv
