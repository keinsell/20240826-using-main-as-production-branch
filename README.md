# `neuronek-cli`

Main repository is available under [`keinsell/neuronek`](https://github.com/keinsell/neuronek). This repository is more
experimental than offical as the official one I would say have trouble with data maintainment over the various
applications and offline-first approach.

This repository is a simplified version of the application which will implement basic commands and will be available
cross-platform comparing to cli application in main repository.

If the experiments in this repository will be successful code will become upstream and will be available as git
submodule in the main repository.

## Introduction

### Building Application

Building production release was desinged and persisted in `.justfile` which contains build setups and configurations
for different plaforms, `just release` will execute all of them.

```bash
just release
```

For `x86_64-unknown-linux-gnu` we create self-extracting archive which is available under `dist/neuronek.run`
