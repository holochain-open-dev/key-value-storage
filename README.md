# Key-Value Storage

A Holochain Zome for persisting key/value pairs - a simple general "database" for Holochain apps. The API is very close to the [Web storage](https://html.spec.whatwg.org/multipage/webstorage.html#webstorage) specifications for `localStorage` and `sessionStorage`.

## Installation

📝 To be written. In the mean time, have a look at the other repos in [Holochain Open Development](https://github.com/holochain-open-dev) for tips.

## Usage

🤔 Sample web app coming. Until then, run and have a look at the tests. The API is quite simple.

## Zome Developer Setup

### Requirements

- Having run through [holochain RSM installation](https://github.com/holochain/holochain-dna-build-tutorial).
- Run all the steps described in this README.md inside the `nix-shell` of the `holochain` core repository.
- Have [`holochain-run-dna`](https://www.npmjs.com/package/@holochain-open-dev/holochain-run-dna) installed globally, and the `lair-keystore` described in its README as well.

### Building

```bash
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown
dna-util -c storage.dna.workdir/
```

### Testing

After having built the DNA:

```bash
cd tests
npm install
npm test
```

### Running

After having built the DNA:

```bash
holochain-run-dna storage.dna.gz
```

Now `holochain` will be listening at port `8888`;

Restart the command if it fails (flaky holochain start).

## Todo

- `list_stores`
- `delete_store`
- `update_store`?
- Validation?!
- Emit signals when changes happen.
- Support more data types.
