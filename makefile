# Makefile para quantbr

.PHONY: default build test clippy fmt check dev

default: dev

build:
	cargo build

test:
	cargo test

clippy:
	cargo clippy --all-targets -- -D warnings

fmt:
	cargo fmt --all

check:
	cargo check

dev:
	cargo check --all-targets
	cargo test --workspace
	cargo clippy --all-targets -- -D warnings
	

.PHONY: build_site build_docs combine_outputs deploy

build_site:
	cd site && bundle exec jekyll build

build_docs:
	rm -rf target/doc
	cargo doc --no-deps -p quant-tools
	mdbook build docs

combine_outputs:
	mkdir -p site/_site/api
	mkdir -p site/_site/book
	cp -r target/doc/. site/_site/api/
	cp -r docs/book/. site/_site/book/

deploy:
	# Garante que estamos na gh-pages
	git checkout gh-pages
	rm -rf *
	cp -r site/_site/* .
	git add .
	git commit -m "Deploy site + book + api"
	git push
	git checkout master
