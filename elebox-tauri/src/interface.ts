
export interface PartData {
    alias?: string;
    category?: string;
    cost?: number;
    datasheet_link?: string;
    description?: string;
    digikey_no?: string;
    image_link?: string;
    location?: string;
    mfr?: string;
    mfr_no?: string;
    mouser_no?: string;
    name?: string;
    package?: string;
    product_link?: string;
    quantity?: number;
    suppliers?: string;
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
