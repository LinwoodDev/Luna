---
title: Introduction
---

> Welcome to Luna, the Library for Universal Networking Assets

## What is Luna?

Luna is a build system written in Rust to having a git repository of assets and have them accessible via a simple file-based static web API.
It is designed to be flexible with a universal API to allow any app to use it.

It uses toml files to define the assets and their metadata and the export file is a json file to allow easy parsing.

Luna uses a namespace system with {author}/{name} to allow for easy organization of assets.

## Why Luna?

Luna is designed to be a simple and easy to use library system for assets.
Current asset systems uses a server to host the asset system which means that you need to have a server running that will be used for fetching which can also be nontransparent to the user.

Luna also doesn't use any login system to allow anonymous access to the assets. All asset changes are done inside the repository and then generated into the static files.

## How to setup Luna

It is recommended to use a git repository to host the assets.
You can use any git hosting provider like GitHub, GitLab, or Bitbucket.
Run `luna init` to create a new Luna repository.
This will create a `config.toml` file which is used to change general settings for the repository.

Generate the index file using `luna generate output` and create a documentation using `luna docs output` to generate a static website for the repository.

Now you can host the `output` folder on any static hosting provider like GitHub Pages, Netlify, or Vercel.
