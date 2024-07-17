import { invoke } from "@tauri-apps/api/tauri";
import {
  Category as CategoryInterface,
  TreeNode as TreeNodeInterface,
} from "./interface";

export namespace DbCategory {
  export type Category = CategoryInterface;
  export type TreeNode = TreeNodeInterface;

  export async function get(name: string) {
    return invoke("get_category", { name }).catch((err) =>
      console.error(`Get part, ${err}`)
    );
  }

  export async function list(): Promise<CategoryInterface[]> {
    return invoke("get_categories", {});
  }

  export async function add(item: CategoryInterface) {
    await invoke("add_category", { item })
      .then((msg) => console.log(`Add category, ${msg}`))
      .catch((err) => console.error(`Add category, ${err}`));
  }

  export async function update(ori_name: string, new_item: CategoryInterface) {
    await invoke("update_category", { ori_name, new_item })
      .then((msg) => console.log(`Update category, ${msg}`))
      .catch((err) => console.error(`Update category, ${err}`));
  }

  export async function remove(name: string) {
    await invoke("del_category", { name })
      .then((msg) => console.log(`Delete category, ${msg}`))
      .catch((err) => console.error(`Delete category, ${err}`));
  }

  export async function getTree(): Promise<TreeNodeInterface[]> {
    return invoke("get_tree", {});
  }
}
