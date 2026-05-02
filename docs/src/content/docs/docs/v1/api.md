---
title: API Reference
---

Luna generates a versioned `index.json` file for consumers. The current format is `file_version = 1`.

```json
{
  "file_version": 1,
  "name": "Linwood Pack Registry",
  "authors": [],
  "assets": []
}
```

Assets use the `{author}/{name}` namespace. Consumers should treat `id`, `author`, `name`, `categories`, and version metadata as the primary fields for discovery and installation.

```json
{
  "id": "codedoctor/starter-pack",
  "author": "codedoctor",
  "name": "starter-pack",
  "categories": ["texture-pack"]
}
```

Use `luna schema` to print the JSON Schema generated from the Rust API structs, or `luna schema docs/public/schemas/index-v1.schema.json` to write it to a file. The published docs site exposes the generated file at `/schemas/index-v1.schema.json`.

Use `luna check` before publishing to catch duplicate IDs, missing authors, drifting asset IDs, invalid URLs, and invalid checksums.

The Rust API crate can be found [in the `api` folder of the git repository](https://github.com/LinwoodDev/Luna/tree/main/api).
