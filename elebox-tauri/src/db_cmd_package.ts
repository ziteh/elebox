import { invoke } from "@tauri-apps/api/tauri";
import { Package as PackageInterface } from "./interface";

export namespace DbPackage {
  export type Package = PackageInterface;

  export async function get(name: string) {
    return invoke("get_package", { name }).catch((err) =>
      console.warn(`Get part, ${err}`)
    );
  }

  export async function list(): Promise<PackageInterface[]> {
    return invoke("get_packages", {});
  }

  export async function add(item: PackageInterface) {
    try {
      const msg = await invoke("add_package", { item });
      console.log(`Add package, ${msg}`);
      return msg;
    } catch (err) {
      console.warn(`Add package, ${err}`);
      throw err;
    }
  }

  export async function update(ori_name: string, new_item: PackageInterface) {
    try {
      const msg = await invoke("update_package", { ori_name, new_item });
      console.log(`Update package, ${msg}`);
      return msg;
    } catch (err) {
      console.warn(`Update package, ${err}`);
      throw err;
    }
  }

  export async function remove(name: string) {
    await invoke("del_package", { name })
      .then((msg) => console.log(`Delete package, ${msg}`))
      .catch((err) => console.warn(`Delete package, ${err}`));
  }
}
