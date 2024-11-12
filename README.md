# apub-lite

## Install Tools

Install [Just](https://github.com/casey/just):

```bash
cargo install just
```

Install other tools:

```bash
just install-tools
```

## Start Server

By default, a domain assigned by [serveo](https://serveo.net/) is used. 
If you want to use a specific domain, set `APUB_LITE_URL` in your `.env` file as follows:

```env
APUB_LITE_URL="https://example.com"
```

Then, start the server in development mode:

```bash
just dev
```

## Check

After completing your work, you need to verify it using the following commands:

- `just format` (alias `just f`): Formats Rust files using [rustfmt](https://github.com/rust-lang/rustfmt).
- `just lint` (alias `just l`): Checks if the code adheres to the rules using [clippy](https://github.com/rust-lang/rust-clippy).

You can also perform both actions together using `just ready` (alias `just r`).

