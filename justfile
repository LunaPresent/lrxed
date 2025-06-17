set dotenv-required
release_plz := require("release-plz")
git_token := env("GIT_TOKEN")

help:
	just --list

release:
	{{ release_plz }} release
