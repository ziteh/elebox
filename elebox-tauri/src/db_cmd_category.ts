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
      console.warn(`Get part, ${err}`)
    );
  }

  export async function list(): Promise<CategoryInterface[]> {
    return invoke("get_categories", {});
  }

  export async function add(item: CategoryInterface) {
    try {
      const msg = await invoke("add_category", { item });
      console.log(`Add category, ${msg}`);
      return msg;
    } catch (err) {
      console.warn(`Add category, ${err}`);
      throw err;
    }
  }

  export async function update(ori_name: string, new_item: CategoryInterface) {
    try {
      const msg = await invoke("update_category", { ori_name, new_item });
      console.log(`Update category, ${msg}`);
      return msg;
    } catch (err) {
      console.warn(`Update category, ${err}`);
      throw err;
    }
  }

  export async function remove(name: string) {
    await invoke("del_category", { name })
      .then((msg) => console.log(`Delete category, ${msg}`))
      .catch((err) => console.warn(`Delete category, ${err}`));
  }

  export async function getTree(): Promise<TreeNodeInterface[]> {
    return invoke("get_tree", {});
  }
}
