# Elebox

![](https://i.imgur.com/fqdDK1d.png)

Lightweight personal electronic parts inventory management tool.

It aims to remain simple, intuitive, zero configuration and works out-of-the-box. The embedded, single-file database is easy to backup.

## Usage

Download executable from [Releases](./releases). (🚧WIP)

### GUI

GUI powered by [Tauri](https://tauri.app/).

For development, install Node.js >= `20.9.0` for Vite.

```bash
cd elebox-tauri
pnpm install
pnpm tauri dev
```

### CLI

If you're run via Cargo (i.e. not using the released executable), replace `elebox-cli` with `cargo run -p elebox-cli --` in the following command. For example, use `cargo run -p elebox-cli -- help` instead of `elebox-cli help`.

#### Basic

Print help message:

```bash
elebox-cli help
```

Use and operate on the default database path `./elebox.db`:

```bash
elebox-cli <COMMAND>
```

or a specified path `./mybox.db`:

```bash
elebox-cli mybox.db <COMMAND>
```

where `<COMMAND>` can be `init`, `part` or `category`.

#### Init

You need to initialize a database before you can proceed with the next steps.

Create and initialize a new database with the default path `./elebox.db`:

```bash
elebox-cli init
```

#### Edit Part category

List part categorys:

```bash
elebox-cli category
```

Create a new part category named `SMD`:

```bash
elebox-cli category new SMD
```

Create new part category named `SOT-23` and `SMD 0603`, and designate them as a subcategory of `SMD`:

```bash
elebox-cli category new SOT-23 -p SMD
elebox-cli category new "SMD 0603" -p SMD
```

#### Edit Part

List parts:

```bash
elebox-cli part 
```

Create a new part named `AMS1117` with a part category of `SMD`, and a quantity of `100`:

```bash
elebox-cli part new AMS1117 100 SMD
```

Consume or restock 10 `AMS1117`:

```bash
elebox-cli part use AMS1117 10
elebox-cli part add AMS1117 10
```

Rename `AMS1117` to `ams1117-3.3v`:

```bash
elebox-cli part update AMS1117 ams1117-3.3v
```

## License

Licensed under either of [Apache-2.0](/LICENSE-APACHE) or [MIT license](/LICENSE-MIT) at your option.

Icons from [Material Design Icons](https://pictogrammers.com/library/mdi/)
