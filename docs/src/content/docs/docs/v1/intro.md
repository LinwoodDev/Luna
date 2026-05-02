---
title: Introduction
---

> Welcome to Luna: Lightweight Universal Networked Assets.

## What is Luna?

Luna stands for **Lightweight Universal Networked Assets**.

It lets you keep asset metadata in a Git repository and publish it as a static JSON index plus an optional documentation website. Apps can read the generated index directly from GitHub Pages, Netlify, Vercel, or any static host without a backend.

It uses TOML files for repository, author, and asset metadata. The generated `index.json` is versioned, validated, and documented with a JSON Schema.

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

Add authors and assets:

```bash
luna create author codedoctor --display-name "Code Doctor"
luna create asset codedoctor starter-pack
```

Edit the generated TOML files, then validate the repository:

```bash
luna check
```

Generate the index file and documentation:

```bash
luna build output/docs output/index.json
```

Now you can host the `output` folder on any static hosting provider like GitHub Pages, Netlify, or Vercel.

## Repository format

```text
config.toml
content/
  codedoctor/
    config.toml
    assets/
      starter-pack/
        config.toml
```

Assets are namespaced as `{author}/{name}`. Luna validates that the stored `id` matches this namespace, that referenced authors exist, that hashes are valid, and that compatibility metadata is well formed.
