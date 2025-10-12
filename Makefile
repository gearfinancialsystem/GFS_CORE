# Makefile

# Charger les variables d'environnement depuis .env
include .env
export LOCAL_PAT_TOKEN_GFS_GITHUB
export CRATES_IO_TOKEN_GFS_UPDATE_ONLY

# Variable pour le type de release (patch, minor, major)
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
	@echo "Bump version et création du tag (type: $(RELEASE_TYPE))..."
	cargo release --workspace $(RELEASE_TYPE) --execute --no-publish
	@echo "Push du tag sur GitHub..."
	git push origin main --tags
	@echo "Création de la release GitHub..."
	GITHUB_TOKEN=$$LOCAL_PAT_TOKEN_GFS_GITHUB gh release create $(shell git describe --tags --abbrev=0) --notes "Release $(shell git describe --tags --abbrev=0)"
	@echo "Publication sur crates.io..."
	CARGO_REGISTRY_TOKEN=$$CRATES_IO_TOKEN_GFS_UPDATE_ONLY cargo release --workspace publish --execute

.PHONY: install
install:
	@echo "Installation des outils nécessaires..."
	cargo install cargo-release
	cargo install gh
