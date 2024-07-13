# Elebox

![](https://i.imgur.com/fqdDK1d.png)

🌐[English](https://github.com/ziteh/elebox?tab=readme-ov-file#readme)

輕量級個人電子零件物料管理工具。

它旨在提供簡單、直覺、無須配置且開箱即用的使用體驗。嵌入式、單檔案資料庫讓資料備份更容易。

## Usage

可以從 [Releases](https://github.com/ziteh/elebox/releases) 下載可執行檔。

### GUI

圖形化程式使用 [Tauri](https://tauri.app/).

對於開發，爲 Vite 安裝 Node.js >= `20.9.0`。

```bash
cd elebox-tauri
pnpm install
pnpm tauri dev
```

### CLI

如果你是直接透過 Cargo 運行（即不是使用發行的可執行檔），請將下列指令中的 `elebox-cli` 替換成 `cargo run -p elebox-cli --`。例如使用 `cargo run -p elebox-cli -- help` 取代 `elebox-cli help`。

#### 基礎

顯示幫助訊息：

```bash
elebox-cli help
```

使用預設的資料庫路徑和檔名 `./elebox.db`：

```bash
elebox-cli <COMMAND>
```

或指定爲 `./my_box.db`:

```bash
elebox-cli my_box.db <COMMAND>
```

`<COMMAND>` 可以是 `init`、`part`、`category`、`export` 或 `import`。

#### 初始化

你需要先初始化資料庫才可以進行後續的其它操作。

建立並初始化一個新的資料庫（以預設路徑和檔名）：

```bash
elebox-cli init
```

#### 編輯零件分類

列出所有零件分類：

```bash
elebox-cli category
```

建立一個名爲 `MCU` 的新分類：

```bash
elebox-cli category new MCU
```

建立新名爲 `ARM` 和 `RISC-V` 的新分類，且將其設爲 `MCU` 的子類別。

```bash
elebox-cli category new ARM -p MCU
elebox-cli category new "RISC-V" -p MCU
```

#### 編輯零件

列出所有零件：

```bash
elebox-cli part
```

建立一個名爲 `RP2040` 的新零件庫存，且分類爲 `ARM`，數量爲 `25` 個：

```bash
elebox-cli part new RP2040 25 ARM
```

消耗或補充 10 個 `RP2040`：

```bash
elebox-cli part use RP2040 10
elebox-cli part add RP2040 10
```

將 `RP2040` 重新命名爲 `rpi-2040`

```bash
elebox-cli part update RP2040 "rpi-2040"
```

## 授權條款

根據選擇使用 [Apache-2.0](/LICENSE-APACHE) 或 [MIT license](/LICENSE-MIT) 進行授權。

Icon 來自 [Material Design Icons](https://pictogrammers.com/library/mdi/)
