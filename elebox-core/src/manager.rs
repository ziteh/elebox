use std::path::PathBuf;

use crate::{
    CategoryHandler, Database, DbCategory, DbManufacturer, DbPackage, DbPart, EleboxError,
    JammDatabase, ManufacturerHandler, PackageHandler, PartHandler, Transferable,
    CATEGORIES_BUCKET, MFR_BUCKET, PACKAGES_BUCKET, PARTS_BUCKET,
};

const PART_FILENAME: &str = "elebox_export_parts.yaml";
const PACKAGE_FILENAME: &str = "elebox_export_packages.yaml";
const CATEGORY_FILENAME: &str = "elebox_export_categories.yaml";
const MFR_FILENAME: &str = "elebox_export_mfrs.yaml";

pub struct Manager {
    part_db: Box<dyn Database<DbPart>>,
    package_db: Box<dyn Database<DbPackage>>,
    category_db: Box<dyn Database<DbCategory>>,
    mfr_db: Box<dyn Database<DbManufacturer>>,
}

impl Manager {
    pub fn new(
        part_db: Box<dyn Database<DbPart>>,
        package_db: Box<dyn Database<DbPackage>>,
        category_db: Box<dyn Database<DbCategory>>,
        mfr_db: Box<dyn Database<DbManufacturer>>,
    ) -> Self {
        Self {
            part_db,
            package_db,
            category_db,
            mfr_db,
        }
    }

    pub fn init(&self) -> Result<(), EleboxError> {
        self.part_db.init()?;
        self.category_db.init()?;
        self.package_db.init()?;
        self.mfr_db.init()?;
        Ok(())
    }

    pub fn check(&self) -> Result<(), EleboxError> {
        self.part_db.check()?;
        self.category_db.check()?;
        self.package_db.check()?;
        self.mfr_db.check()?;
        Ok(())
    }

    pub fn part(&self) -> PartHandler {
        PartHandler {
            db: &*self.part_db,
            pkg_db: &*self.package_db,
            cat_db: &*self.category_db,
            mfr_db: &*self.mfr_db,
        }
    }

    pub fn category(&self) -> CategoryHandler {
        CategoryHandler {
            db: &*self.category_db,
        }
    }

    pub fn package(&self) -> PackageHandler {
        PackageHandler {
            db: &*self.package_db,
        }
    }

    pub fn manufacturer(&self) -> ManufacturerHandler {
        ManufacturerHandler { db: &*self.mfr_db }
    }

    pub fn export(&self, path: &PathBuf) -> Result<(), EleboxError> {
        let filename = path.join(PART_FILENAME);
        let _ = self.part().export(&filename)?;

        let filename = path.join(PACKAGE_FILENAME);
        let _ = self.package().export(&filename)?;

        let filename = path.join(CATEGORY_FILENAME);
        let _ = self.category().export(&filename)?;

        let filename = path.join(MFR_FILENAME);
        let _ = self.manufacturer().export(&filename)?;

        Ok(())
    }

    pub fn import(&self, path: &PathBuf) -> Result<(), EleboxError> {
        let filename = path.join(CATEGORY_FILENAME);
        let _ = self.category().import(&filename)?;

        let filename = path.join(PACKAGE_FILENAME);
        let _ = self.package().import(&filename)?;

        let filename = path.join(MFR_FILENAME);
        let _ = self.manufacturer().import(&filename)?;

        let filename = path.join(PART_FILENAME);
        let _ = self.part().import(&filename)?;

        Ok(())
    }

    pub fn from(
        part_db: Box<dyn Database<DbPart>>,
        package_db: Box<dyn Database<DbPackage>>,
        category_db: Box<dyn Database<DbCategory>>,
        mfr_db: Box<dyn Database<DbManufacturer>>,
        path: &PathBuf,
    ) -> Result<Self, EleboxError> {
        // TODO
        let mgr = Self::new(part_db, package_db, category_db, mfr_db);
        mgr.init();

        let filename = path.join(CATEGORY_FILENAME);
        let _ = mgr.category().import(&filename)?;

        let filename = path.join(PACKAGE_FILENAME);
        let _ = mgr.package().import(&filename)?;

        let filename = path.join(MFR_FILENAME);
        let _ = mgr.manufacturer().import(&filename)?;

        let filename = path.join(PART_FILENAME);
        let _ = mgr.part().import(&filename)?;

        Ok(mgr)
    }
}
