# Makefile
include .env
export LOCAL_PAT_TOKEN_GFS_GITHUB
export CRATES_IO_TOKEN_GFS_UPDATE_ONLY

RELEASE_TYPE ?= patch

.PHONY: release-patch
release-patch:
	@echo "Release de type PATCH..."
	$(MAKE) release RELEASE_TYPE=patch

.PHONY: release-minor
release-minor:
	@echo "Release de type MINOR..."
	$(MAKE) release RELEASE_TYPE=minor

.PHONY: release-major
release-major:
	@echo "Release de type MAJOR..."
	$(MAKE) release RELEASE_TYPE=major

.PHONY: release
release:
	@echo "Configuration de l'URL Git avec le token..."
	git remote set-url origin https://$$LOCAL_PAT_TOKEN_GFS_GITHUB@github.com/gearfinancialsystem/GFS_CORE.git

	@echo "Vérification des changements non commités..."
	@if [ -n "$(shell git status --porcelain)" ]; then \
		echo "Des changements non commités ont été détectés. Commit automatique..."; \
		git add .; \
		git commit -m "Pre-release commit: $(shell date +'%Y-%m-%d %H:%M:%S')"; \
	fi

	@echo "Bump version, création du tag global, push et GitHub Release..."
	CARGO_REGISTRY_TOKEN=$$CRATES_IO_TOKEN_GFS_UPDATE_ONLY cargo release --workspace $(RELEASE_TYPE) --execute

.PHONY: install
install:
	@echo "Installation des outils nécessaires..."
	cargo install cargo-release
	cargo install gh
