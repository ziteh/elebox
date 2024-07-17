import { invoke } from "@tauri-apps/api/tauri";
import { Package as PackageInterface } from "./interface";

export namespace DbPackage {
  export type Package = PackageInterface;

  export async function get(name: string) {
    return invoke("get_package", { name }).catch((err) =>
      console.error(`Get part, ${err}`)
    );
  }

  export async function list(): Promise<PackageInterface[]> {
    return invoke("get_packages", {});
  }

  export async function add(item: PackageInterface) {
    await invoke("add_package", { item })
      .then((msg) => console.log(`Add package, ${msg}`))
      .catch((err) => console.error(`Add package, ${err}`));
  }

  export async function update(ori_name: string, new_item: PackageInterface) {
    await invoke("update_package", { ori_name, new_item })
      .then((msg) => console.log(`Update package, ${msg}`))
      .catch((err) => console.error(`Update package, ${err}`));
  }

  export async function remove(name: string) {
    await invoke("del_package", { name })
      .then((msg) => console.log(`Delete package, ${msg}`))
      .catch((err) => console.error(`Delete package, ${err}`));
  }
}
