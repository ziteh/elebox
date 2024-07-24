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
