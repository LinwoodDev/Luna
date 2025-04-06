# Contribution guide

There are many ways to contribute to this project.
Thanks to everyone who has contributed!
If you have any questions, please contact me at [contact@linwood.dev](mailto:contact@linwood.dev).

There is a matrix server for this project [here](https://linwood.dev/matrix) and a discord server [here](https://discord.linwood.dev).

Please follow the [Code of Conduct](https://luna.linwood.dev/code-of-conduct).

## Give feedback

The best way to give feedback is to open an issue on GitHub.
Please report any bug you find [here](https://github.com/LinwoodDev/Luna/issues/new?assignees=CodeDoctorDE&labels=bug%2Ctriage&template=bug_report.yml&title=%5BBug%5D%3A+).

If you have a feature that you would like to see added, please open an issue [here](https://github.com/LinwoodDev/Luna/issues/new?assignees=CodeDoctorDE&labels=enhancement%2Ctriage&template=feature_request.yml&title=%5BFeature+request%5D%3A+).

## Write documentation

Documentation is important for users to understand the program and its features.
The documentation is written in markdown, a simple markup language. It can be found in the `docs` folder.

To start, please install [pnpm](https://pnpm.io/installation).

Get started by running:

```bash
cd docs
pnpm install
pnpm start
```

All stable documentation can be found in the `versioned_docs` folder.

Fork the project and create a pull request to add your documentation to the `develop` branch.

## Code

This project is written in [Rust](https://www.rust-lang.org/).
The cli source code can be found in the `cli` folder.

To get started, run:

```bash
cd cli
cargo build
```

Fork the project and create a pull request to add your code to the `develop` branch.
