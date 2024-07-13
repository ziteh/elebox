# Elebox

![](https://i.imgur.com/fqdDK1d.png)

🌐[中文](https://github.com/ziteh/elebox/blob/main/README-zh.md)

Lightweight personal electronic parts inventory management tool.

It aims to remain simple, intuitive, zero configuration and works out-of-the-box. The embedded, single-file database is easy to backup.

> 🚧 In development

## Features

- Category tree.
    ![](https://i.imgur.com/RvYJg8k.png)
- Export as CSV, import from CSV.

## Usage

Download executable from [Releases](https://github.com/ziteh/elebox/releases).

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

Use and operate on the default database path and filename `./elebox.db`:

```bash
elebox-cli <COMMAND>
```

or a specified `./my_box.db`:

```bash
elebox-cli my_box.db <COMMAND>
```

where `<COMMAND>` can be `init`, `part`, `category`, `export` or `import`.

#### Init

You need to initialize a database before you can proceed with the next steps.

Create and initialize a new database with the default path:

```bash
elebox-cli init
```

#### Edit Part category

List part categories:

```bash
elebox-cli category
```

Create a new part category named `MCU`:

```bash
elebox-cli category new MCU
```

Create new part category named `ARM` and `RISC-V`, and designate them as a subcategory of `MCU`:

```bash
elebox-cli category new ARM -p MCU
elebox-cli category new "RISC-V" -p MCU
```

#### Edit Part

List parts:

```bash
elebox-cli part
```

Create a new part named `RP2040` with a part category of `ARM`, and a quantity of `25`:

```bash
elebox-cli part new RP2040 25 ARM
```

Consume or restock 10 `RP2040`:

```bash
elebox-cli part use RP2040 10
elebox-cli part add RP2040 10
```

Rename `RP2040` to `rpi-RP2040`:

```bash
elebox-cli part update RP2040 "rpi-RP2040"
```

## License

Licensed under either of [Apache-2.0](/LICENSE-APACHE) or [MIT license](/LICENSE-MIT) at your option.

Icons from [Material Design Icons](https://pictogrammers.com/library/mdi/)
