IMAGE := ghcr.io/aodihis/websocket-playground
TAG   ?= latest

.PHONY: build push release

build:
	docker build -t $(IMAGE):$(TAG) .

push:
	docker push $(IMAGE):$(TAG)

release: build push
	@echo "Released $(IMAGE):$(TAG)"
