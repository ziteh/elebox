
export interface CustomField {
    name: string;
    field_type: string; // TODO enum
    value: string;
}

export interface Supplier {
    name?: string;
    link?: string;
    price?: number;
    note?: string;
}

export interface PartData {
    name?: string;
    quantity?: number;
    category?: string;
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
    custom_fields?: CustomField[];
    suppliers?: Supplier[];
}

export interface Parts extends Array<PartData> { }

export interface Categories {
    [index: number]: {
        name: string;
        parent: string;
    };
}

export interface Packages {
    [index: number]: {
        name: string;
        pkg_type: string;
        alias: string;
    };
}

export interface Manufacturers {
    [index: number]: {
        name: string;
        alias: string;
        url: string;
    };
}

export interface TreeNode {
    name: string;
    children: TreeNode[];
}
