import { invoke } from "@tauri-apps/api/tauri";
import { Manufacturer as ManufacturerInterface } from "./interface";

export namespace DbManufacturer {
  export type Manufacturer = ManufacturerInterface;

  export async function get(name: string) {
    return invoke("get_mfr", { name }).catch((err) =>
      console.warn(`Get part, ${err}`)
    );
  }

  export async function list(): Promise<ManufacturerInterface[]> {
    return invoke("get_mfrs", {});
  }

  export async function add(item: ManufacturerInterface) {
    try {
      const msg = await invoke("add_mfr", { item });
      console.log(`Add manufacturer, ${msg}`);
      return msg;
    } catch (err) {
      console.warn(`Add manufacturer, ${err}`);
      throw err;
    }
  }

  export async function update(
    ori_name: string,
    new_item: ManufacturerInterface
  ) {
    try {
      const msg = await invoke("update_mfr", { ori_name, new_item });
      console.log(`Update manufacturer, ${msg}`);
      return msg;
    } catch (err) {
      console.warn(`Update manufacturer, ${err}`);
      throw err;
    }
  }

  export async function remove(name: string) {
    await invoke("del_mfr", { name })
      .then((msg) => console.log(`Delete manufacturer, ${msg}`))
      .catch((err) => console.warn(`Delete manufacturer, ${err}`));
  }
}
