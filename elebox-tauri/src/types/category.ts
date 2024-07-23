export interface Category {
  name: string;
  parent?: string;
  alias?: string;
}

export interface TreeNode {
  name: string;
  children: TreeNode[];
}
