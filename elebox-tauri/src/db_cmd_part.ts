import { invoke } from "@tauri-apps/api/tauri";
import { Part as PartInterface } from "./interface";

export namespace DbPart {
  export type Part = PartInterface;

  export async function get(name: string) {
    return invoke("get_part", { name }).catch((err) =>
      console.error(`Get part, ${err}`)
    );
  }

  export async function list(): Promise<PartInterface[]> {
    return invoke("get_parts", {});
  }

  export async function add(item: PartInterface) {
    await invoke("add_part", { item })
      .then((msg) => console.log(`Add part, ${msg}`))
      .catch((err) => console.error(`Add part, ${err}`));
  }

  export async function update(ori_name: string, new_item: PartInterface) {
    await invoke("update_part", { ori_name, new_item })
      .then((msg) => console.log(`Update part, ${msg}`))
      .catch((err) => console.error(`Update part, ${err}`));
  }

  export async function remove(name: string) {
    await invoke("del_part", { name })
      .then((msg) => console.log(`Delete part, ${msg}`))
      .catch((err) => console.error(`Delete part, ${err}`));
  }

  export async function modifyQty(name: string, increment: number) {
    await invoke("increment_part", { name, increment })
      .then((msg) => console.log(`Modify qty part, ${msg}`))
      .catch((err) => console.error(`Modify qty part, ${err}`));
  }
}
