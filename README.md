# Elebox

Lightweight personal electronic parts inventory management tool.

## Usage

### Tauri GUI

Install Node.js >= `20.9.0` for Vite.

```
cd elebox
pnpm install
pnpm tauri dev
```

### CLI

If you're run via Cargo (i.e. not using the released executable), replace `elebox-cli` with `cargo run -p elebox-cli --` in the following command. For example, use `cargo run -p elebox-cli -- help` instead of `elebox-cli help`.

#### Basic

Print help message:
```
elebox-cli help
```

Use and operate on the default database filename `elebox.db`:
```
elebox-cli <COMMAND>
```
or a specified filename `mybox.db`:
```
elebox-cli mybox.db <COMMAND>
```

where `<COMMAND>` can be `init`, `part` or `type`.

#### Init

Create and initialize a new database with the default filename `elebox.db`:
```
elebox-cli init
```

#### Edit Part Type

List part types:
```
elebox-cli type
```

Create a new part type named `SMD`:
```
elebox-cli type new SMD
```

Create new part type named `SOT-23` and `SMD 0603`, and designate them as a subtype of `SMD`:
```
elebox-cli type new SOT-23 -p SMD
elebox-cli type new "SMD 0603" -p SMD
```

#### Edit Part

List parts:
```
elebox-cli part 
```

Create a new part named `AMS1117` with a part type of `SMD`, and a quantity of `100`:
```
elebox-cli part new AMS1117 100 SMD
```

Consume or restock 10 `AMS1117`:
```
elebox-cli part use AMS1117 10
elebox-cli part add AMS1117 10
```

Rename `AMS1117` to `ams1117-3.3v`:
```
elebox-cli part update AMS1117 ams1117-3.3v
```

## License

Licensed under either of [Apache-2.0](/LICENSE-APACHE) or [MIT license](/LICENSE-MIT) at your option.