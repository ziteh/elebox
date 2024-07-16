import { invoke } from "@tauri-apps/api/tauri";
import { Manufacturer as ManufacturerInterface } from "./interface";

export namespace DbManufacturer {
  export type Manufacturer = ManufacturerInterface;

  export async function get(name: string) {
    return invoke("get_mfr", { name }).catch((err) =>
      console.error(`Get part, ${err}`)
    );
  }

  export async function list(): Promise<ManufacturerInterface[]> {
    return invoke("get_mfrs", {});
  }

  export async function add(item: ManufacturerInterface) {
    await invoke("add_mfr", { item })
      .then((msg) => console.log(`Add manufacturer, ${msg}`))
      .catch((err) => console.error(`Add manufacturer, ${err}`));
  }

  export async function update(
    ori_name: string,
    new_item: ManufacturerInterface
  ) {
    await invoke("update_mfr", { ori_name, new_item })
      .then((msg) => console.log(`Update manufacturer, ${msg}`))
      .catch((err) => console.error(`Update manufacturer, ${err}`));
  }

  export async function remove(name: string) {
    await invoke("del_mfr", { name })
      .then((msg) => console.log(`Delete manufacturer, ${msg}`))
      .catch((err) => console.error(`Delete manufacturer, ${err}`));
  }
}
