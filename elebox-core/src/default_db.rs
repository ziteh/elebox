use crate::{category::*, db::*, manufacturer::*, package::*, Part, PartManager};
use std::path::Path;

pub fn create_default_db(path: &str) {
    let exists = Path::new(&path).exists();

    let db = JammDatabase::new(path);
    db.init();

    if exists {
        return;
    }

    let cats: Vec<Category> = vec![
        Category {
            name: "Resistors".to_string(),
            parent: None,
            alias: None,
        },
        Category {
            name: "Variable Resistors".to_string(),
            parent: Some("Resistors".to_string()),
            alias: None,
        },
        Category {
            name: "Capacitors".to_string(),
            parent: None,
            alias: None,
        },
        Category {
            name: "Electrolytic Caps".to_string(),
            parent: Some("Capacitors".to_string()),
            alias: None,
        },
        Category {
            name: "Tantalum Caps".to_string(),
            parent: Some("Capacitors".to_string()),
            alias: None,
        },
        Category {
            name: "Ceramic Caps".to_string(),
            parent: Some("Capacitors".to_string()),
            alias: None,
        },
        Category {
            name: "Inductors".to_string(),
            parent: None,
            alias: None,
        },
        Category {
            name: "Crystals".to_string(),
            parent: None,
            alias: None,
        },
        Category {
            name: "Oscillators".to_string(),
            parent: None,
            alias: None,
        },
        Category {
            name: "Diodes".to_string(),
            parent: None,
            alias: None,
        },
        Category {
            name: "LED".to_string(),
            parent: Some("Diodes".to_string()),
            alias: None,
        },
        Category {
            name: "TVS".to_string(),
            parent: Some("Diodes".to_string()),
            alias: None,
        },
        Category {
            name: "MCU".to_string(),
            parent: None,
            alias: Some("Microcontroller".to_string()),
        },
        Category {
            name: "PMIC".to_string(),
            parent: None,
            alias: Some("Power Management IC".to_string()),
        },
        Category {
            name: "LDO Regulators".to_string(),
            parent: Some("PMIC".to_string()),
            alias: None,
        },
        Category {
            name: "DC-DC Regulators".to_string(),
            parent: Some("PMIC".to_string()),
            alias: None,
        },
        Category {
            name: "Buck Converters".to_string(),
            parent: Some("DC-DC Regulators".to_string()),
            alias: None,
        },
        Category {
            name: "Boost Converters".to_string(),
            parent: Some("DC-DC Regulators".to_string()),
            alias: None,
        },
        Category {
            name: "Connectors".to_string(),
            parent: None,
            alias: None,
        },
        Category {
            name: "Buttons".to_string(),
            parent: None,
            alias: None,
        },
        Category {
            name: "Transistors".to_string(),
            parent: None,
            alias: None,
        },
        Category {
            name: "N-MOSFETs".to_string(),
            parent: Some("Transistors".to_string()),
            alias: None,
        },
        Category {
            name: "P-MOSFETs".to_string(),
            parent: Some("Transistors".to_string()),
            alias: None,
        },
        Category {
            name: "Modules".to_string(),
            parent: None,
            alias: None,
        },
    ];

    let cat_mgr = CategoryManager::new(&db);
    for c in cats {
        let _ = cat_mgr.add(&c);
    }

    let pkgs: Vec<Package> = vec![
        Package {
            name: "SMD 0201".to_string(),
            pkg_type: PackageType::Smt,
            alias: None,
        },
        Package {
            name: "SMD 0402".to_string(),
            pkg_type: PackageType::Smt,
            alias: None,
        },
        Package {
            name: "SMD 0603".to_string(),
            pkg_type: PackageType::Smt,
            alias: None,
        },
        Package {
            name: "SMD 0805".to_string(),
            pkg_type: PackageType::Smt,
            alias: None,
        },
        Package {
            name: "SMD 1206".to_string(),
            pkg_type: PackageType::Smt,
            alias: None,
        },
        Package {
            name: "SOT-23".to_string(),
            pkg_type: PackageType::Smt,
            alias: None,
        },
        Package {
            name: "SOT-23-5".to_string(),
            pkg_type: PackageType::Smt,
            alias: None,
        },
        Package {
            name: "SOIC-8".to_string(),
            pkg_type: PackageType::Smt,
            alias: None,
        },
        Package {
            name: "QFN-56".to_string(),
            pkg_type: PackageType::Smt,
            alias: None,
        },
        Package {
            name: "TO-92".to_string(),
            pkg_type: PackageType::Tht,
            alias: None,
        },
        Package {
            name: "TO-220".to_string(),
            pkg_type: PackageType::Tht,
            alias: None,
        },
    ];

    let pkg_mgr = PackageManager::new(&db);
    for p in pkgs {
        let _ = pkg_mgr.add(&p);
    }

    let mfrs: Vec<Manufacturer> = vec![
        Manufacturer {
            name: "Raspberry Pi".to_string(),
            alias: None,
            url: Some("https://www.raspberrypi.com/".to_string()),
        },
        Manufacturer {
            name: "Texas Instruments".to_string(),
            alias: Some("TI".to_string()),
            url: Some("https://www.ti.com/".to_string()),
        },
        Manufacturer {
            name: "Analog Devices".to_string(),
            alias: Some("ADI".to_string()),
            url: Some("https://www.analog.com/en/index.html".to_string()),
        },
        Manufacturer {
            name: "STMicroelectronics".to_string(),
            alias: Some("ST".to_string()),
            url: Some("https://www.st.com/content/st_com/en.html".to_string()),
        },
        Manufacturer {
            name: "Microchip".to_string(),
            alias: None,
            url: Some("https://www.microchip.com/".to_string()),
        },
        Manufacturer {
            name: "Infineon".to_string(),
            alias: None,
            url: Some("https://www.infineon.com/".to_string()),
        },
        Manufacturer {
            name: "ON Semiconductor".to_string(),
            alias: Some("Onsemi".to_string()),
            url: Some("https://www.onsemi.com/".to_string()),
        },
        Manufacturer {
            name: "ROHM".to_string(),
            alias: None,
            url: Some("https://www.rohm.com/".to_string()),
        },
    ];

    let mfr_mgr = ManufacturerManager::new(&db);
    for m in mfrs {
        let _ = mfr_mgr.add(&m);
    }

    let rp2040 = Part {
        name: "RP2040".to_string(),
        quantity: 15,
        category: "MCU".to_string(),
        alias: Some("RPi RP2040".to_string()),
        package: Some("QFN-56".to_string()),
        package_detail: Some("7x7mm P0.4mm 1EP3.2x32.mm".to_string()),
        mfr: Some("Raspberry Pi".to_string()),
        mfr_no: Some("SC0914(7)".to_string()),
        datasheet_link: Some(
            "https://datasheets.raspberrypi.com/rp2040/rp2040-datasheet.pdf".to_string(),
        ),
        product_link: Some(
            "https://www.raspberrypi.com/documentation/microcontrollers/rp2040.html".to_string(),
        ),
        image_link: Some(
            "https://www.raspberrypi.com/documentation/microcontrollers/images/rp2040.jpg"
                .to_string(),
        ),
        description: Some("Dual ARM Cortex-M0+ 133MHz, 264KB SRAM".to_string()),
        location: Some("Box #1".to_string()),
        starred: false,
        custom_fields: vec![
            CustomField {
                name: "Mouser #".to_string(),
                field_type: CustomFieldType::Normal,
                value: "358-SC09147".to_string(),
            },
            CustomField {
                name: "DigiKey #".to_string(),
                field_type: CustomFieldType::Normal,
                value: "2648-SC0914(7)CT-ND".to_string(),
            },
        ],
        suppliers: vec![
            Supplier{
               name:"Mouser".to_string(),
               link:"https://www.mouser.com/ProductDetail/Raspberry-Pi/SC09147?qs=T%252BzbugeAwjhSpdbCB4ve%252Bg%3D%3D".to_string(),
               price:Some(0.8),
               note:"7' reel".to_string(),
            },
            Supplier{
               name:"DigiKey".to_string(),
               link:"https://www.digikey.com/en/products/detail/raspberry-pi/SC0914-7/14306009?s=N4IgTCBcDa4GwBYAcBaAygYQAwE4CMCAFAOwCUGAKigHIAiIAugL5A".to_string(),
               price:Some(0.8),
               note:"7' reel".to_string(),
            },
        ],
    };
    let part_mgr = PartManager::new(&db);
    let _ = part_mgr.add(&rp2040);
}
