build:
	./scripts/build.sh julie-vic-wedding-api $(version)

build_in_github:
	./scripts/build_in_github.sh

postgres:
	docker run -it --rm --name julie-vic -p 5432:5432 postgres
