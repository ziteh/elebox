import { invoke } from "@tauri-apps/api/tauri";
import { Part as PartInterface } from "@/types/part";

export namespace DbPart {
  export type Part = PartInterface;

  export async function get(name: string) {
    return invoke("get_part", { name }).catch((err) =>
      console.warn(`Get part, ${err}`)
    );
  }

  export async function list(): Promise<PartInterface[]> {
    return invoke("get_parts", {});
  }

  export async function add(item: PartInterface) {
    try {
      const msg = await invoke("add_part", { item });
      console.log(`Add part, ${msg}`);
      return msg;
    } catch (err) {
      console.warn(`Add part, ${err}`);
      throw err;
    }
  }

  export async function update(ori_name: string, new_item: PartInterface) {
    try {
      const msg = await invoke("update_part", { ori_name, new_item });
      console.log(`Update part, ${msg}`);
      return msg;
    } catch (err) {
      console.warn(`Update part, ${err}`);
      throw err;
    }
  }

  export async function remove(name: string) {
    await invoke("del_part", { name })
      .then((msg) => console.log(`Delete part, ${msg}`))
      .catch((err) => console.warn(`Delete part, ${err}`));
  }

  export async function modifyQty(name: string, increment: number) {
    await invoke("increment_part", { name, increment })
      .then((msg) => console.log(`Modify qty part, ${msg}`))
      .catch((err) => console.warn(`Modify qty part, ${err}`));
  }
}
