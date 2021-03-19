**👍 2020-03-19, Compiles and runs on hdk 0.0.100 👍**

# Key-Value Storage

A Holochain Zome for persisting key/value pairs - a simple general "database" for Holochain apps. The API is very close to the [Web storage](https://html.spec.whatwg.org/multipage/webstorage.html#webstorage) specifications for `localStorage` and `sessionStorage`.

Multiple stores/"databases" are supported.

## Functions

| Function     |                                                                                                                          |
| ------------ | ------------------------------------------------------------------------------------------------------------------------ |
| create_store | Creates a named key/value store.                                                                                         |
| get_store    | Returns store information, or error if no store with the given name.                                                     |
| set_item     | Sets the value of the pair identified by key to value, creating a new key/value pair if none existed for key previously. |
| get_item     | Returns the current value associated with the given key, or error if the given key does not exist.                       |
| remove_item  | Removes the key/value pair with the given key, if a key/value pair with the given key exists.                            |
| length       | Returns the number of keys in the store                                                                                  |
| keys         | Returns a list with all the keys in the store                                                                            |
| key          | Returns the name of the nth key, or Error if n is greater than or equal to the number of key value pairs.                |
| clear        | Removes all key/value pairs, if there are any.                                                                           |

Sample usage:

```JavaScript
call("storage", "create_store", {
    store: "ui-state",
});

call("storage", "set_item", {
    store: "ui-state",
    key: "selected-theme",
    value: "dark",
});

call("storage", "set_item", {
    store: "ui-state",
    key: "flavor",
    value: "unicorn",
});

call("storage", "keys", {
    store: "ui-state",
});

// Returns:
{
  "keys": [
    { "key": "selected-theme" },
    { "key": "flavor" }
  ]
}

call("storage", "get_item", {
  store: "ui-state",
  key: "flavor",
});

// Returns:
{
  "itemHash": "uhCkk_...PLM",
  "item": {
    key: "flavor",
    value: "unicorn",
  }
}
```

## Installation

📝 To be written. In the mean time, have a look at the other repos in [Holochain Open Development](https://github.com/holochain-open-dev) for tips.

## Usage

🤔 Sample web app coming. Until then, run and have a look at the tests. The API is quite simple.

## Zome Developer Setup

### Prerequisites

Have [Nix Package Manager](https://developer.holochain.org/docs/install/) installed.

Launch the nix-shell to install Holochain Developer Tools. This will take a while the first time.

```bash
nix-shell
```

### Building

From within `nix-shell`, run:

```bash
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown
```

### Testing

From within `nix-shell`, run:

```bash
cd tests
npm install
npm test
```

### Docs

To generate the Zome API docs:

```bash
cargo doc --open
```

### Todo

- `list_stores`
- `delete_store`
- `update_store`
- Validations?!
- Capabilities?!
- Emit signals when changes happen.
- Support for more key value data types.

### Contributing

Yes, please! Raise an issue or post a pull request.

### License

MIT
