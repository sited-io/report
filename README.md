# Report service for sited.io

## Prerequesites

Ensure `service-apis` git submodule is initialized. If not yet done run:

```sh
git submodule update --init
```

If `service-apis` git submodule was already initialized, ensure to pull the newest changes:

```sh
git submodule update --remote
```

## Build

```sh
cargo build
```

## Run locally

Ensure environment variables are set.

```sh
export RUST_LOG=info
export RUST_BACKTRACE=0

export GH_PAT="<GitHub Personal Access Token>"
```

Then run:

```sh
cargo run
```
