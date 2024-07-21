export interface CustomField {
  name: string;
  field_type: string; // TODO enum
  value: string;
}

export interface Supplier {
  name: string;
  link: string;
  price?: number;
  note: string;
}

export interface Part {
  name: string;
  quantity: number;
  category: string;
  package?: string;
  package_detail?: string;
  mfr?: string;
  location?: string;
  alias?: string;
  description?: string;
  mfr_no?: string;
  datasheet_link?: string;
  product_link?: string;
  image_link?: string;
  custom_fields: CustomField[];
  suppliers: Supplier[];
  starred: boolean;
}

export interface Category {
  name: string;
  parent?: string;
  alias?: string;
}

export enum PkgType {
  Smt = "Smt",
  Tht = "Tht",
  Others = "Others",
}

export interface Package {
  name: string;
  pkg_type: PkgType;
  alias?: string;
}

export interface Manufacturer {
  name: string;
  alias?: string;
  url?: string;
}

export interface TreeNode {
  name: string;
  children: TreeNode[];
}

export interface Config {
  language?: string;
  database?: string;
}
