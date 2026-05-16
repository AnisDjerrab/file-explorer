use etcetera::{choose_base_strategy, BaseStrategy};
use std::{
    fs::{self, DirEntry},
    io::ErrorKind,
};

pub fn get_home_directory() -> String {
    // this works on linux, windows, macos and iOS, but not on android. we unfortunately have to hard code the home adress on android
    #[cfg(not(target_os = "android"))]
    match choose_base_strategy() {
        Ok(strategy) => strategy.home_dir().to_string_lossy().into_owned(),
        Err(_) => "Err: invalid path.".to_string(),
    }
    #[cfg(target_os = "android")]
    {
        std::fs::create_dir_all("/data/data/com.anis.file_explorer/vFS");
        "/data/data/com.anis.file_explorer/vFS".to_string()
    }
}

fn cvt_dir_entry_str(input: &mut Vec<DirEntry>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for entry in input {
        output.push(entry.file_name().to_string_lossy().to_string());
    }
    output
}

fn sort(unsorted_list: &mut Vec<DirEntry>, sort_method: String) -> Vec<String> {
    if sort_method == "A-Z" {
        unsorted_list.sort_by(|a, b| {
            let a_name = a.file_name().to_string_lossy().into_owned();
            let b_name = b.file_name().to_string_lossy().into_owned();

            a_name.cmp(&b_name)
        });
        let sorted_list = cvt_dir_entry_str(unsorted_list);
        sorted_list
    } else if sort_method == "Z-A" {
        unsorted_list.sort_by(|a, b| {
            let a_name = a.file_name().to_string_lossy().into_owned();
            let b_name = b.file_name().to_string_lossy().into_owned();

            b_name.cmp(&a_name)
        });
        let sorted_list = cvt_dir_entry_str(unsorted_list);
        sorted_list
    } else if sort_method == "last_modified" {
        unsorted_list.sort_by(|a, b| {
            let a_name = a.metadata().unwrap().modified().unwrap();
            let b_name = b.metadata().unwrap().modified().unwrap();

            a_name.cmp(&b_name)
        });
        let sorted_list = cvt_dir_entry_str(unsorted_list);
        sorted_list
    } else if sort_method == "most_recently_modified" {
        unsorted_list.sort_by(|a, b| {
            let a_name = a.metadata().unwrap().modified().unwrap();
            let b_name = b.metadata().unwrap().modified().unwrap();

            b_name.cmp(&a_name)
        });
        let sorted_list = cvt_dir_entry_str(unsorted_list);
        sorted_list
    } else if sort_method == "smallest_size" {
        unsorted_list.sort_by(|a, b| {
            let a_name = a.metadata().unwrap().len();
            let b_name = b.metadata().unwrap().len();

            a_name.cmp(&b_name)
        });
        let sorted_list = cvt_dir_entry_str(unsorted_list);
        sorted_list
    } else if sort_method == "biggest_size" {
        unsorted_list.sort_by(|a, b| {
            let a_name = a.metadata().unwrap().len();
            let b_name = b.metadata().unwrap().len();

            b_name.cmp(&a_name)
        });
        let sorted_list = cvt_dir_entry_str(unsorted_list);
        sorted_list
    } else if sort_method == "extension" {
        unsorted_list.sort_by(|a, b| {
            if a.metadata().unwrap().is_dir()
                || a.metadata().unwrap().is_symlink() && b.metadata().unwrap().is_dir()
                || b.metadata().unwrap().is_symlink()
            {
                let a_tmp = a.path();
                let b_tmp = b.path();
                let a_name = a.file_name().to_string_lossy().into_owned();
                let b_name = b.file_name().to_string_lossy().into_owned();

                a_name.cmp(&b_name)
            } else {
                let a_name = a.file_name().to_string_lossy().into_owned();
                let b_name = b.file_name().to_string_lossy().into_owned();

                a_name.cmp(&b_name)
            }
        });
        let sorted_list = cvt_dir_entry_str(unsorted_list);
        sorted_list
    } else {
        let unsorted_return_list = cvt_dir_entry_str(unsorted_list);
        unsorted_return_list
    }
}

#[tauri::command]
pub fn list_folders(path: String, sort_method: String) -> (i32, Vec<String>) {
    let mut unsorted_list: Vec<DirEntry> = Vec::new();
    for entry in fs::read_dir(path).into_iter().flatten() {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };
        if entry.metadata().map(|m| m.is_dir()).unwrap_or(false) {
            unsorted_list.push(entry);
        }
    }
    let sorted_list = sort(&mut unsorted_list, sort_method);
    (0, sorted_list)
}

pub fn list_files(path: String, sort_method: String) -> (i32, Vec<String>) {
    let mut unsorted_list: Vec<DirEntry> = Vec::new();
    for entry in fs::read_dir(path).into_iter().flatten() {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };
        if entry.metadata().map(|m| m.is_file()).unwrap_or(false) {
            unsorted_list.push(entry);
        }
    }
    let sorted_list = sort(&mut unsorted_list, sort_method);
    (0, sorted_list)
}

pub fn list_symlinks(path: String, sort_method: String) -> (i32, Vec<String>) {
    let mut unsorted_list: Vec<DirEntry> = Vec::new();
    for entry in fs::read_dir(path).into_iter().flatten() {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };
        if entry.metadata().map(|m| m.is_symlink()).unwrap_or(false) {
            unsorted_list.push(entry);
        }
    }
    let sorted_list = sort(&mut unsorted_list, sort_method);
    (0, sorted_list)
}

// this list contains all the mime types whith their corresponding file name
pub const MIME_ICONS: &[(&str, &str)] = &[
    ("application/affinity", "application-affinity"),
    (
        "application/affinity-template",
        "application-affinity-template",
    ),
    ("application/apk", "application-apk"),
    ("application/atom+xml", "application-atom+xml"),
    ("application/bitwig-clip", "application-bitwig-clip"),
    ("application/bitwig-device", "application-bitwig-device"),
    ("application/epub+zip", "application-epub+zip"),
    ("application/geo+json", "application-geo+json"),
    ("application/gml+xml", "application-gml+xml"),
    ("application/gml+xml", "qgis-gml"),
    ("application/gpx", "application-gpx"),
    ("application/gpx+xml", "application-gpx+xml"),
    ("application/hwp", "application-hwp"),
    ("application/hwpx", "application-hwpx"),
    ("application/illustrator", "application-illustrator"),
    ("application/java", "application-java"),
    ("application/javascript", "application-javascript"),
    ("application/json", "application-json"),
    ("application/loc+xml", "application-loc+xml"),
    ("application/mathematica", "application-mathematica"),
    (
        "application/mathematicaplayer",
        "application-mathematicaplayer",
    ),
    ("application/mbox", "application-mbox"),
    ("application/metalink+xml", "application-metalink+xml"),
    ("application/metalink4+xml", "application-metalink4+xml"),
    ("application/octet-stream", "application-octet-stream"),
    ("application/owl+xml", "application-owl+xml"),
    ("application/pdf", "application-pdf"),
    ("application/pgp", "application-pgp"),
    ("application/pgp-encrypted", "application-pgp-encrypted"),
    ("application/pgp-keys", "application-pgp-keys"),
    ("application/pgp-signature", "application-pgp-signature"),
    ("application/photoshop", "application-photoshop"),
    ("application/pkcs10", "application-pkcs10"),
    ("application/pkcs12", "application-pkcs12"),
    ("application/pkcs7-mime", "application-pkcs7-mime"),
    ("application/pkcs7-signature", "application-pkcs7-signature"),
    ("application/pkcs8", "application-pkcs8"),
    ("application/pkix-cert", "application-pkix-cert"),
    ("application/postscript", "application-postscript"),
    ("application/rss+xml", "application-rss+xml"),
    ("application/schema+json", "application-schema+json"),
    ("application/slf+xml", "application-slf+xml"),
    ("application/sql", "application-sql"),
    ("application/tcx+xml", "application-tcx+xml"),
    ("application/toml", "application-toml"),
    ("application/typescript", "application-typescript"),
    (
        "application/vnd-google-earth-kml",
        "application-vnd-google-earth-kml",
    ),
    (
        "application/vnd.adobe.aftereffects.project",
        "application-vnd.adobe.aftereffects.project",
    ),
    (
        "application/vnd.adobe.aftereffects.template",
        "application-vnd.adobe.aftereffects.template",
    ),
    ("application/vnd.adobe.xd", "application-vnd.adobe.xd"),
    (
        "application/vnd.amazon.mobi8-ebook",
        "application-vnd.amazon.mobi8-ebook",
    ),
    (
        "application/vnd.android.package-archive",
        "android-package-archive",
    ),
    (
        "application/vnd.android.package-archive",
        "application-vnd.android.package-archive",
    ),
    ("application/vnd.ant.fit", "application-vnd.ant.fit"),
    ("application/vnd.appimage", "application-vnd.appimage"),
    (
        "application/vnd.coffeescript",
        "application-vnd.coffeescript",
    ),
    (
        "application/vnd.comicbook+zip",
        "application-vnd.comicbook+zip",
    ),
    (
        "application/vnd.comicbook-rar",
        "application-vnd.comicbook-rar",
    ),
    (
        "application/vnd.debian.binary-package",
        "application-vnd.debian.binary-package",
    ),
    ("application/vnd.efi.img", "application-vnd.efi.img"),
    ("application/vnd.efi.iso", "application-vnd.efi.iso"),
    ("application/vnd.fai.igc", "application-vnd.fai.igc"),
    ("application/vnd.flatpak", "application-vnd.flatpak"),
    ("application/vnd.flatpak.ref", "application-vnd.flatpak.ref"),
    ("application/vnd.geo+json", "application-vnd.geo+json"),
    (
        "application/vnd.google-apps.document",
        "application-vnd.google-apps.document",
    ),
    ("application/vnd.google-apps.document", "gddoc"),
    (
        "application/vnd.google-apps.drawing",
        "application-vnd.google-apps.drawing",
    ),
    ("application/vnd.google-apps.drawing", "gddraw"),
    ("application/vnd.google-apps.form", "gdform"),
    (
        "application/vnd.google-apps.fusiontable",
        "application-vnd.google-apps.fusiontable",
    ),
    ("application/vnd.google-apps.fusiontable", "gdtable"),
    (
        "application/vnd.google-apps.map",
        "application-vnd.google-apps.map",
    ),
    (
        "application/vnd.google-apps.presentation",
        "application-vnd.google-apps.presentation",
    ),
    ("application/vnd.google-apps.presentation", "gdslides"),
    (
        "application/vnd.google-apps.site",
        "application-vnd.google-apps.site",
    ),
    (
        "application/vnd.google-apps.spreadsheet",
        "application-vnd.google-apps.spreadsheet",
    ),
    (
        "application/vnd.google-earth.kml",
        "application-vnd.google-earth.kml",
    ),
    (
        "application/vnd.google-earth.kml+xml",
        "application-vnd.google-earth.kml+xml",
    ),
    (
        "application/vnd.google-earth.kmz",
        "application-vnd.google-earth.kmz",
    ),
    ("application/vnd.iccprofile", "application-vnd.iccprofile"),
    (
        "application/vnd.insync.link.drive.doc",
        "application-vnd.insync.link.drive.doc",
    ),
    (
        "application/vnd.insync.link.drive.draw",
        "application-vnd.insync.link.drive.draw",
    ),
    (
        "application/vnd.insync.link.drive.form",
        "application-vnd.insync.link.drive.form",
    ),
    (
        "application/vnd.insync.link.drive.slides",
        "application-vnd.insync.link.drive.slides",
    ),
    (
        "application/vnd.insync.link.drive.table",
        "application-vnd.insync.link.drive.table",
    ),
    ("application/vnd.leocad", "application-vnd.leocad"),
    ("application/vnd.ms-access", "application-vnd.ms-access"),
    ("application/vnd.ms-access", "office-database"),
    ("application/vnd.ms-excel", "application-vnd.ms-excel"),
    (
        "application/vnd.ms-excel.spreadsheet.macroenabled.12",
        "application-vnd.ms-excel.spreadsheet.macroenabled.12",
    ),
    (
        "application/vnd.ms-powerpoint",
        "application-vnd.ms-powerpoint",
    ),
    (
        "application/vnd.ms-powerpoint.presentation.macroEnabled.12",
        "application-vnd.ms-powerpoint.presentation.macroEnabled.12",
    ),
    ("application/vnd.ms-word", "application-vnd.ms-word"),
    (
        "application/vnd.ms-word.document.macroenabled.12",
        "application-vnd.ms-word.document.macroenabled.12",
    ),
    ("application/vnd.nmea.nmea", "application-vnd.nmea.nmea"),
    (
        "application/vnd.oasis.opendocument.database",
        "application-vnd.oasis.opendocument.database",
    ),
    (
        "application/vnd.oasis.opendocument.database",
        "libreoffice-database",
    ),
    (
        "application/vnd.oasis.opendocument.database",
        "libreoffice-oasis-database",
    ),
    (
        "application/vnd.oasis.opendocument.database",
        "oasis-database",
    ),
    (
        "application/vnd.oasis.opendocument.database",
        "openoffice4-database",
    ),
    (
        "application/vnd.oasis.opendocument.database",
        "openoffice4-oasis-database",
    ),
    (
        "application/vnd.oasis.opendocument.database-template",
        "application-vnd.oasis.opendocument.database-template",
    ),
    (
        "application/vnd.oasis.opendocument.database-template",
        "libreoffice-database-template",
    ),
    (
        "application/vnd.oasis.opendocument.database-template",
        "libreoffice-oasis-database-template",
    ),
    (
        "application/vnd.oasis.opendocument.database-template",
        "oasis-database-template",
    ),
    (
        "application/vnd.oasis.opendocument.formula",
        "libreoffice-formula",
    ),
    (
        "application/vnd.oasis.opendocument.formula",
        "libreoffice-oasis-formula",
    ),
    (
        "application/vnd.oasis.opendocument.formula",
        "oasis-formula",
    ),
    (
        "application/vnd.oasis.opendocument.formula-template",
        "libreoffice-formula-template",
    ),
    (
        "application/vnd.oasis.opendocument.formula-template",
        "libreoffice-oasis-formula-template",
    ),
    (
        "application/vnd.oasis.opendocument.formula-template",
        "oasis-formula-template",
    ),
    (
        "application/vnd.oasis.opendocument.graphics",
        "libreoffice-drawing",
    ),
    (
        "application/vnd.oasis.opendocument.graphics",
        "libreoffice-oasis-drawing",
    ),
    (
        "application/vnd.oasis.opendocument.graphics",
        "oasis-drawing",
    ),
    (
        "application/vnd.oasis.opendocument.graphics",
        "openoffice4-drawing",
    ),
    (
        "application/vnd.oasis.opendocument.graphics",
        "openoffice4-oasis-drawing",
    ),
    (
        "application/vnd.oasis.opendocument.graphics-template",
        "libreoffice-drawing-template",
    ),
    (
        "application/vnd.oasis.opendocument.graphics-template",
        "libreoffice-oasis-drawing-template",
    ),
    (
        "application/vnd.oasis.opendocument.graphics-template",
        "oasis-drawing-template",
    ),
    (
        "application/vnd.oasis.opendocument.graphics-template",
        "openoffice4-drawing-template",
    ),
    (
        "application/vnd.oasis.opendocument.graphics-template",
        "openoffice4-oasis-drawing-template",
    ),
    (
        "application/vnd.oasis.opendocument.presentation",
        "libreoffice-oasis-presentation",
    ),
    (
        "application/vnd.oasis.opendocument.presentation",
        "libreoffice-presentation",
    ),
    (
        "application/vnd.oasis.opendocument.presentation",
        "oasis-presentation",
    ),
    (
        "application/vnd.oasis.opendocument.presentation",
        "openoffice4-oasis-presentation",
    ),
    (
        "application/vnd.oasis.opendocument.presentation",
        "openoffice4-presentation",
    ),
    (
        "application/vnd.oasis.opendocument.presentation-template",
        "libreoffice-oasis-presentation-template",
    ),
    (
        "application/vnd.oasis.opendocument.presentation-template",
        "libreoffice-presentation-template",
    ),
    (
        "application/vnd.oasis.opendocument.presentation-template",
        "oasis-presentation-template",
    ),
    (
        "application/vnd.oasis.opendocument.presentation-template",
        "openoffice4-presentation-template",
    ),
    (
        "application/vnd.oasis.opendocument.spreadsheet",
        "application-vnd.oasis.opendocument.spreadsheet",
    ),
    (
        "application/vnd.oasis.opendocument.spreadsheet",
        "libreoffice-oasis-spreadsheet",
    ),
    (
        "application/vnd.oasis.opendocument.spreadsheet",
        "libreoffice-spreadsheet",
    ),
    (
        "application/vnd.oasis.opendocument.spreadsheet",
        "oasis-spreadsheet",
    ),
    (
        "application/vnd.oasis.opendocument.spreadsheet",
        "openoffice4-oasis-spreadsheet",
    ),
    (
        "application/vnd.oasis.opendocument.spreadsheet",
        "openoffice4-spreadsheet",
    ),
    (
        "application/vnd.oasis.opendocument.spreadsheet-template",
        "libreoffice-oasis-spreadsheet-template",
    ),
    (
        "application/vnd.oasis.opendocument.spreadsheet-template",
        "libreoffice-spreadsheet-template",
    ),
    (
        "application/vnd.oasis.opendocument.spreadsheet-template",
        "oasis-spreadsheet-template",
    ),
    (
        "application/vnd.oasis.opendocument.text",
        "application-vnd.oasis.opendocument.text",
    ),
    (
        "application/vnd.oasis.opendocument.text",
        "libreoffice-empty",
    ),
    (
        "application/vnd.oasis.opendocument.text",
        "libreoffice-oasis-empty",
    ),
    (
        "application/vnd.oasis.opendocument.text",
        "libreoffice-oasis-text",
    ),
    (
        "application/vnd.oasis.opendocument.text",
        "libreoffice-text",
    ),
    ("application/vnd.oasis.opendocument.text", "oasis-empty"),
    ("application/vnd.oasis.opendocument.text", "oasis-text"),
    (
        "application/vnd.oasis.opendocument.text",
        "openoffice4-oasis-text",
    ),
    (
        "application/vnd.oasis.opendocument.text-master",
        "libreoffice-master-document",
    ),
    (
        "application/vnd.oasis.opendocument.text-master",
        "libreoffice-oasis-master-document",
    ),
    (
        "application/vnd.oasis.opendocument.text-master",
        "oasis-master-document",
    ),
    (
        "application/vnd.oasis.opendocument.text-master-template",
        "libreoffice-master-document-template",
    ),
    (
        "application/vnd.oasis.opendocument.text-master-template",
        "libreoffice-oasis-master-document-template",
    ),
    (
        "application/vnd.oasis.opendocument.text-master-template",
        "oasis-master-document-template",
    ),
    (
        "application/vnd.oasis.opendocument.text-template",
        "application-vnd.oasis.opendocument.text-template",
    ),
    (
        "application/vnd.oasis.opendocument.text-template",
        "libreoffice-empty-template",
    ),
    (
        "application/vnd.oasis.opendocument.text-template",
        "libreoffice-oasis-empty-template",
    ),
    (
        "application/vnd.oasis.opendocument.text-template",
        "libreoffice-oasis-text-template",
    ),
    (
        "application/vnd.oasis.opendocument.text-template",
        "libreoffice-text-template",
    ),
    (
        "application/vnd.oasis.opendocument.text-template",
        "oasis-empty-template",
    ),
    (
        "application/vnd.oasis.opendocument.text-template",
        "oasis-text-template",
    ),
    (
        "application/vnd.oasis.opendocument.text-template",
        "openoffice4-oasis-text-template",
    ),
    (
        "application/vnd.oasis.opendocument.text-template",
        "openoffice4-text-template",
    ),
    (
        "application/vnd.oasis.opendocument.text-web",
        "libreoffice-oasis-web",
    ),
    (
        "application/vnd.oasis.opendocument.text-web",
        "libreoffice-oasis-web-template",
    ),
    (
        "application/vnd.oasis.opendocument.text-web",
        "libreoffice-web",
    ),
    (
        "application/vnd.oasis.opendocument.text-web",
        "libreoffice-web-template",
    ),
    ("application/vnd.oasis.opendocument.text-web", "oasis-web"),
    (
        "application/vnd.oasis.opendocument.text-web",
        "oasis-web-template",
    ),
    (
        "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        "application-vnd.openxmlformats-officedocument.presentationml.presentation",
    ),
    (
        "application/vnd.openxmlformats-officedocument.presentationml.slideshow",
        "application-vnd.openxmlformats-officedocument.presentationml.slideshow",
    ),
    (
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "application-vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    ),
    (
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "application-vnd.openxmlformats-officedocument.wordprocessingml.document",
    ),
    (
        "application/vnd.oziexplorer.plt",
        "application-vnd.oziexplorer.plt",
    ),
    (
        "application/vnd.oziexplorer.rte",
        "application-vnd.oziexplorer.rte",
    ),
    (
        "application/vnd.oziexplorer.wpt",
        "application-vnd.oziexplorer.wpt",
    ),
    (
        "application/vnd.rmaps.sqlite",
        "application-vnd.rmaps.sqlite",
    ),
    ("application/vnd.scribus", "application-vnd.scribus"),
    ("application/vnd.shp", "application-vnd.shp"),
    ("application/vnd.shx", "application-vnd.shx"),
    ("application/vnd.snap", "application-vnd.snap"),
    ("application/vnd.sqlite2", "application-vnd.sqlite2"),
    ("application/vnd.sqlite3", "application-vnd.sqlite3"),
    ("application/vnd.sqlite3", "qgis-sqlite"),
    ("application/vnd.squashfs", "application-vnd.squashfs"),
    (
        "application/vnd.tcpdump.pcap",
        "org.wireshark.Wireshark-mimetype",
    ),
    ("application/vnd.wolfram.cdf", "application-vnd.wolfram.cdf"),
    (
        "application/vnd.wolfram.mathematica.package",
        "application-vnd.wolfram.mathematica.package",
    ),
    ("application/vnd.wolfram.nb", "application-vnd.wolfram.nb"),
    (
        "application/vnd.wolfram.player",
        "application-vnd.wolfram.player",
    ),
    ("application/vnd.wolfram.wl", "application-vnd.wolfram.wl"),
    ("application/vnd.wolfram.wls", "application-vnd.wolfram.wls"),
    ("application/wasm", "application-wasm"),
    (
        "application/x-adobe-indesign",
        "application-x-adobe-indesign",
    ),
    (
        "application/x-apple-diskimage",
        "application-x-apple-diskimage",
    ),
    (
        "application/x-applix-spreadsheet",
        "application-x-applix-spreadsheet",
    ),
    (
        "application/x-audacity-project",
        "application-x-audacity-project",
    ),
    (
        "application/x-audacity-project+sqlite3",
        "application-x-audacity-project+sqlite3",
    ),
    (
        "application/x-bibtex-text-file",
        "application-x-bibtex-text-file",
    ),
    ("application/x-bittorrent", "application-x-bittorrent"),
    ("application/x-bitwig-clip", "application-x-bitwig-clip"),
    (
        "application/x-bitwig-clip",
        "com.bitwig.BitwigStudio.application-bitwig-clip",
    ),
    ("application/x-bitwig-device", "application-x-bitwig-device"),
    (
        "application/x-bitwig-device",
        "com.bitwig.BitwigStudio.application-bitwig-device",
    ),
    (
        "application/x-bitwig-extension",
        "com.bitwig.BitwigStudio.application-bitwig-extension",
    ),
    (
        "application/x-bitwig-impulse",
        "com.bitwig.BitwigStudio.application-bitwig-impulse",
    ),
    (
        "application/x-bitwig-modulator",
        "com.bitwig.BitwigStudio.application-bitwig-modulator",
    ),
    (
        "application/x-bitwig-module",
        "com.bitwig.BitwigStudio.application-bitwig-module",
    ),
    (
        "application/x-bitwig-package",
        "com.bitwig.BitwigStudio.application-bitwig-package",
    ),
    (
        "application/x-bitwig-preset",
        "com.bitwig.BitwigStudio.application-bitwig-preset",
    ),
    (
        "application/x-bitwig-project",
        "com.bitwig.BitwigStudio.application-bitwig-project",
    ),
    (
        "application/x-bitwig-remote-controls",
        "com.bitwig.BitwigStudio.application-bitwig-remote-controls",
    ),
    ("application/x-bitwig-studio", "application-x-bitwig-studio"),
    ("application/x-blender", "application-x-blender"),
    ("application/x-bps-patch", "application-x-bps-patch"),
    ("application/x-bsdiff", "application-x-bsdiff"),
    ("application/x-bzpostscript", "application-x-bzpostscript"),
    ("application/x-cb7", "application-x-cb7"),
    ("application/x-cba", "application-x-cba"),
    ("application/x-cbr", "application-x-cbr"),
    ("application/x-cbt", "application-x-cbt"),
    ("application/x-cbz", "application-x-cbz"),
    ("application/x-cd-image", "application-x-cd-image"),
    ("application/x-clojure", "application-x-clojure"),
    ("application/x-clojurescript", "application-x-clojurescript"),
    (
        "application/x-com.bitwig-clip.BitwigStudio",
        "application-x-com.bitwig-clip.BitwigStudio",
    ),
    (
        "application/x-com.bitwig-device.BitwigStudio",
        "application-x-com.bitwig-device.BitwigStudio",
    ),
    (
        "application/x-com.bitwig.BitwigStudio",
        "application-x-com.bitwig.BitwigStudio",
    ),
    (
        "application/x-compressed-iso",
        "application-x-compressed-iso",
    ),
    ("application/x-cson", "application-x-cson"),
    ("application/x-dbf", "application-x-dbf"),
    ("application/x-deb", "application-x-deb"),
    ("application/x-designer", "application-x-designer"),
    ("application/x-e-theme", "application-x-e-theme"),
    ("application/x-emerald-theme", "application-x-emerald-theme"),
    (
        "application/x-fender-fenderstudio",
        "com.fender.studio.application-x.fender-fenderstudio",
    ),
    (
        "application/x-fender-jamtrack",
        "com.fender.studio.application-x.fender-jamtrack",
    ),
    ("application/x-fictionbook", "application-x-fictionbook"),
    (
        "application/x-fictionbook+xml",
        "application-x-fictionbook+xml",
    ),
    ("application/x-firmware", "application-x-firmware"),
    ("application/x-gdbm", "application-x-gdbm"),
    ("application/x-gdscript", "application-x-gdscript"),
    ("application/x-generic", "application-x-generic"),
    ("application/x-generic", "package-x-generic"),
    (
        "application/x-gettext-translation",
        "application-x-gettext-translation",
    ),
    ("application/x-godot-project", "application-x-godot-project"),
    (
        "application/x-godot-resource",
        "application-x-godot-resource",
    ),
    ("application/x-godot-scene", "application-x-godot-scene"),
    ("application/x-godot-shader", "application-x-godot-shader"),
    ("application/x-gpx", "application-x-gpx"),
    ("application/x-gpx+xml", "application-x-gpx+xml"),
    ("application/x-gzpdf", "application-x-gzpdf"),
    ("application/x-gzpostscript", "application-x-gzpostscript"),
    ("application/x-hwp", "application-x-hwp"),
    ("application/x-hwpx", "application-x-hwpx"),
    ("application/x-ips-patch", "application-x-ips-patch"),
    ("application/x-ipynb+json", "application-x-ipynb+json"),
    ("application/x-iso", "application-x-iso"),
    (
        "application/x-iso9600-appimage",
        "application-x-iso9600-appimage",
    ),
    ("application/x-jar", "application-x-jar"),
    ("application/x-java", "application-x-java"),
    ("application/x-java-applet", "application-x-java-applet"),
    ("application/x-java-archive", "application-x-java-archive"),
    ("application/x-javascript", "application-x-javascript"),
    ("application/x-julia", "application-x-julia"),
    ("application/x-kdenlive", "application-x-kdenlive"),
    ("application/x-keepass", "application-x-keepass"),
    ("application/x-keepass2", "application-x-keepass2"),
    ("application/x-keepassx", "application-x-keepassx"),
    ("application/x-keepassxc", "application-x-keepassxc"),
    (
        "application/x-keepassxc",
        "org.keepassxc.KeePassXC-application-x-keepassxc",
    ),
    (
        "application/x-kexi-connectiondata",
        "application-x-kexi-connectiondata",
    ),
    (
        "application/x-kexiproject-sqlite2",
        "application-x-kexiproject-sqlite2",
    ),
    (
        "application/x-kexiproject-sqlite3",
        "application-x-kexiproject-sqlite3",
    ),
    ("application/x-kgetlist", "application-x-kgetlist"),
    ("application/x-kpresenter", "application-x-kpresenter"),
    ("application/x-krita", "application-x-krita"),
    (
        "application/x-krita-assistant",
        "application-x-krita-assistant",
    ),
    (
        "application/x-krita-paintoppresent",
        "application-x-krita-paintoppresent",
    ),
    ("application/x-ktheme", "application-x-ktheme"),
    ("application/x-lzpdf", "application-x-lzpdf"),
    ("application/x-mobi8-ebook", "application-x-mobi8-ebook"),
    (
        "application/x-mobipocket-ebook",
        "application-x-mobipocket-ebook",
    ),
    ("application/x-model", "application-x-model"),
    ("application/x-msdownload", "application-x-msdownload"),
    (
        "application/x-musescore4portable",
        "application-x-musescore4portable",
    ),
    (
        "application/x-musescore4portable+xml",
        "application-x-musescore4portable+xml",
    ),
    (
        "application/x-nero-burning-rom-image",
        "gnome-mime-application-x-nero-disk-image",
    ),
    ("application/x-nero-burning-rom-image", "nero-disk-image"),
    ("application/x-ole-storage", "application-x-ole-storage"),
    ("application/x-osm+xml", "application-x-osm+xml"),
    ("application/x-pem-key", "application-x-pem-key"),
    ("application/x-perl", "application-x-perl"),
    ("application/x-photoshop", "application-x-photoshop"),
    ("application/x-php", "application-x-php"),
    ("application/x-pkcs12", "application-x-pkcs12"),
    ("application/x-pkcs7", "application-x-pkcs7"),
    (
        "application/x-pkcs7-certificates",
        "application-x-pkcs7-certificates",
    ),
    (
        "application/x-ptoptimizer-script",
        "application-x-ptoptimizer-script",
    ),
    (
        "application/x-python-bytecode",
        "application-x-python-bytecode",
    ),
    ("application/x-qgis", "qgis-qgs"),
    ("application/x-qgis-layer", "qgis-qlr"),
    (
        "application/x-qtractor-archive",
        "org.rncbc.qtractor.application-x-qtractor-archive",
    ),
    (
        "application/x-qtractor-session",
        "org.rncbc.qtractor.application-x-qtractor-session",
    ),
    (
        "application/x-qtractor-template",
        "org.rncbc.qtractor.application-x-qtractor-template",
    ),
    (
        "application/x-raw-disk-image",
        "application-x-raw-disk-image",
    ),
    (
        "application/x-raw-disk-image-xz-compressed",
        "application-x-raw-disk-image-xz-compressed",
    ),
    (
        "application/x-raw-floppy-disk-image",
        "application-x-raw-floppy-disk-image",
    ),
    ("application/x-rdata", "application-x-rdata"),
    ("application/x-reaper", "cockos-reaper-document"),
    ("application/x-reaper-backup", "cockos-reaper-backup"),
    ("application/x-reaper-peak", "cockos-reaper-peak"),
    ("application/x-reaper-template", "cockos-reaper-template2"),
    ("application/x-reaper-template", "cockos-reaper-template"),
    ("application/x-reaper-theme", "cockos-reaper-theme"),
    ("application/x-rpm", "application-x-rpm"),
    ("application/x-ruby", "application-x-ruby"),
    ("application/x-shapefile", "qgis-shp"),
    ("application/x-shellscript", "application-x-shellscript"),
    ("application/x-source-rpm", "application-x-source-rpm"),
    ("application/x-spectrum", "application-x-spectrum"),
    ("application/x-spectrum-tzx", "application-x-spectrum-tzx"),
    ("application/x-sqlite2", "application-x-sqlite2"),
    ("application/x-sqlite3", "application-x-sqlite3"),
    ("application/x-subrip", "application-x-subrip"),
    ("application/x-theme", "application-x-theme"),
    ("application/x-typescript", "application-x-typescript"),
    (
        "application/x-virtualbox-hdd",
        "application-x-virtualbox-hdd",
    ),
    ("application/x-virtualbox-hdd", "virtualbox-hdd"),
    (
        "application/x-virtualbox-ova",
        "application-x-virtualbox-ova",
    ),
    ("application/x-virtualbox-ova", "virtualbox-ova"),
    (
        "application/x-virtualbox-ovf",
        "application-x-virtualbox-ovf",
    ),
    ("application/x-virtualbox-ovf", "virtualbox-ovf"),
    (
        "application/x-virtualbox-vbox",
        "application-x-virtualbox-vbox",
    ),
    ("application/x-virtualbox-vbox", "virtualbox-vbox"),
    (
        "application/x-virtualbox-vbox-extpack",
        "application-x-virtualbox-vbox-extpack",
    ),
    (
        "application/x-virtualbox-vbox-extpack",
        "virtualbox-vbox-extpack",
    ),
    (
        "application/x-virtualbox-vdi",
        "application-x-virtualbox-vdi",
    ),
    ("application/x-virtualbox-vdi", "virtualbox-vdi"),
    (
        "application/x-virtualbox-vhd",
        "application-x-virtualbox-vhd",
    ),
    ("application/x-virtualbox-vhd", "virtualbox-vhd"),
    (
        "application/x-virtualbox-vmdk",
        "application-x-virtualbox-vmdk",
    ),
    ("application/x-virtualbox-vmdk", "virtualbox-vmdk"),
    (
        "application/x-windows-themepack",
        "application-x-windows-themepack",
    ),
    ("application/x-x509-ca-cert", "application-x-x509-ca-cert"),
    (
        "application/x-x509-user-cert",
        "application-x-x509-user-cert",
    ),
    ("application/x-xopp", "application-x-xopp"),
    ("application/x-xopp", "gnome-mime-application-x-xopp"),
    ("application/x-xopt", "application-x-xopt"),
    ("application/x-xopt", "gnome-mime-application-x-xopt"),
    ("application/x-xzpdf", "application-x-xzpdf"),
    ("application/x-yaml", "application-x-yaml"),
    ("application/x-zim", "org.kiwix.desktop.x-zim_source"),
    (
        "application/x-zip-compressed-fb2",
        "application-x-zip-compressed-fb2",
    ),
    ("application/xml", "application-xml"),
    ("application/xslt+xml", "application-xslt+xml"),
    ("application/yaml", "application-yaml"),
    (
        "audio/x-dawproject",
        "com.bitwig.BitwigStudio.audio-x.dawproject",
    ),
    ("chemical/x-cache", "chemical-x-cache"),
    ("image/vnd.adobe.photoshop", "image-vnd.adobe.photoshop"),
    ("image/vnd.djvu", "image-vnd.djvu"),
    ("image/vnd.djvu+multipage", "image-vnd.djvu+multipage"),
    ("image/vnd.dxf", "qgis-dxf"),
    ("image/x-djvu", "image-x-djvu"),
    ("image/x-generic", "image-x-generic"),
    ("image/x-krita", "image-x-krita"),
    ("image/x-photoshop", "image-x-photoshop"),
    ("inode/symlink", "inode-symlink"),
    ("model/obj", "model-obj"),
    ("model/stl", "model-stl"),
    ("model/x-stl-binary", "model-x-stl-binary"),
    ("model/x.stl-ascii", "model-x.stl-ascii"),
    ("model/x.stl-binary", "model-x.stl-binary"),
    ("multipart/encrypted", "multipart-encrypted"),
    ("multipart/signed", "multipart-signed"),
    ("text/arduino", "text-arduino"),
    ("text/asciidoc", "text-asciidoc"),
    ("text/coffeescript", "text-coffeescript"),
    ("text/css", "text-css"),
    ("text/csv", "text-csv"),
    ("text/csv-schema", "text-csv-schema"),
    ("text/fsharp", "text-fsharp"),
    ("text/javascript", "text-javascript"),
    ("text/json", "text-json"),
    ("text/less", "text-less"),
    ("text/markdown", "text-markdown"),
    ("text/pureDataPatch", "text-pureDataPatch"),
    ("text/rdf+xml", "text-rdf+xml"),
    ("text/ruby", "text-ruby"),
    ("text/rust", "text-rust"),
    ("text/spreadsheet", "text-spreadsheet"),
    ("text/tab-separated-values", "text-tab-separated-values"),
    ("text/vtt", "text-vtt"),
    ("text/x-R", "text-x-R"),
    ("text/x-arduino", "text-x-arduino"),
    ("text/x-ass", "text-x-ass"),
    ("text/x-bibtex", "text-x-bibtex"),
    ("text/x-c", "text-x-c"),
    ("text/x-c++", "text-x-c++"),
    ("text/x-c++hdr", "text-x-c++hdr"),
    ("text/x-c++src", "text-x-c++src"),
    ("text/x-chdr", "text-x-chdr"),
    ("text/x-clojure", "text-x-clojure"),
    ("text/x-clojurescript", "text-x-clojurescript"),
    ("text/x-cmake", "text-x-cmake"),
    ("text/x-cobol", "text-x-cobol"),
    ("text/x-coffeescript", "text-x-coffeescript"),
    ("text/x-common-lisp", "text-x-common-lisp"),
    ("text/x-cpp", "text-x-cpp"),
    ("text/x-cpphdr", "text-x-cpphdr"),
    ("text/x-cppsrc", "text-x-cppsrc"),
    ("text/x-csharp", "text-x-csharp"),
    ("text/x-csrc", "text-x-csrc"),
    ("text/x-css", "text-x-css"),
    ("text/x-emacs-lisp", "text-x-emacs-lisp"),
    ("text/x-fsharp", "text-x-fsharp"),
    ("text/x-generic", "text-x-generic"),
    ("text/x-gettext-translation", "text-x-gettext-translation"),
    (
        "text/x-gettext-translation-template",
        "text-x-gettext-translation-template",
    ),
    ("text/x-go", "text-x-go"),
    ("text/x-gradle", "text-x-gradle"),
    ("text/x-gradle-kotlin", "text-x-gradle-kotlin"),
    ("text/x-hex", "text-x-hex"),
    ("text/x-java", "text-x-java"),
    ("text/x-java-source", "text-x-java-source"),
    ("text/x-javascript", "text-x-javascript"),
    ("text/x-julia", "text-x-julia"),
    ("text/x-kotlin", "text-x-kotlin"),
    ("text/x-less", "text-x-less"),
    ("text/x-log", "text-x-log"),
    ("text/x-lua", "text-x-lua"),
    ("text/x-makefile", "gnome-mime-text-x-makefile"),
    ("text/x-makefile", "text-x-makefile"),
    ("text/x-markdown", "text-x-markdown"),
    ("text/x-maxima-out", "text-x-maxima-out"),
    ("text/x-maximasession", "text-x-maximasession"),
    ("text/x-meson", "text-x-meson"),
    ("text/x-ocaml", "text-x-ocaml"),
    ("text/x-octave", "text-x-octave"),
    ("text/x-patch", "text-x-patch"),
    ("text/x-php", "text-x-php"),
    ("text/x-pkgbuild", "text-x-pkgbuild"),
    ("text/x-preview", "text-x-preview"),
    ("text/x-python", "text-x-python"),
    ("text/x-python3", "text-x-python3"),
    ("text/x-r", "text-x-r"),
    ("text/x-r-markdown", "text-x-r-markdown"),
    ("text/x-r-source", "text-x-r-source"),
    ("text/x-ruby", "text-x-ruby"),
    ("text/x-sass", "text-x-sass"),
    ("text/x-script", "text-x-script"),
    ("text/x-scss", "text-x-scss"),
    ("text/x-sql", "text-x-sql"),
    ("text/x-ssa", "text-x-ssa"),
    ("text/x-stex", "text-x-stex"),
    ("text/x-tex", "text-x-tex"),
    ("text/x-texinfo", "text-x-texinfo"),
    ("text/x-typescript", "text-x-typescript"),
    ("text/x-typst", "text-x-typst"),
    ("text/x-vala", "text-x-vala"),
    ("text/x-yaml", "text-x-yaml"),
    ("text/x-zig", "text-x-zig"),
    ("text/xml", "text-xml"),
    ("text/yaml", "text-yaml"),
    ("video/x-generic", "video-x-generic"),
    ("x-content/ebook-reader", "x-content-ebook-reader"),
];

// total: 466 entries

pub fn cvt_mimetype_iconfilename(mimetype: String) -> String {
    let mime_icons_path = "assets/mimetypes/";
    let relative_icon_path = "../src/assets/mimetypes/";
    let icon_extension = ".svg";
    let output_mime_icon_file = MIME_ICONS
        .binary_search_by_key(&mimetype, |&(m, _)| m.to_string())
        .ok()
        .map(|i| MIME_ICONS[i].1)
        .unwrap_or("invalid_path");
    let mut return_path = format!(
        "{}{}{}",
        &mime_icons_path, &output_mime_icon_file, &icon_extension
    );
    let relative_path = format!(
        "{}{}{}",
        &relative_icon_path, &output_mime_icon_file, &icon_extension
    );
    if !std::path::Path::new(&relative_path).exists() {
        return_path = "assets/mimetypes/text-x-generic.svg".to_string();
    }
    return_path
}

pub fn ls_dir(
    path: String,
    sort_method: String,
    sort_method_dfs: String,
) -> (Vec<String>, Vec<String>, Vec<String>) {
    // first : get all the files
    let mut status_list = Vec::new();
    let mut file_list = list_files(path.clone(), sort_method.clone()).1;
    let mut list_icons_files: Vec<String> = Vec::new();
    for file in &file_list {
        let full_path = format!("{}{}", &path, &file);
        let mime: &str =
            tree_magic_mini::from_filepath(std::path::Path::new(&full_path)).unwrap_or("");
        list_icons_files.push(cvt_mimetype_iconfilename(mime.to_string()));
        status_list.push("f".to_string());
    }
    // second : the directories now
    let mut dir_list = list_folders(path.clone(), sort_method.clone()).1;
    let mut list_icons_dir: Vec<String> = Vec::new();
    for dir in &dir_list {
        list_icons_dir.push("assets/places/folder.svg".to_string());
        status_list.push("d".to_string());
    }
    // third : the symlinks now
    let mut symlink_list = list_symlinks(path.clone(), sort_method.clone()).1;
    let mut list_icons_symlinks: Vec<String> = Vec::new();
    for symlink in &symlink_list {
        let full_path = format!("{}{}", &path, &symlink);
        let mime: &str =
            tree_magic_mini::from_filepath(std::path::Path::new(&full_path)).unwrap_or("");
        list_icons_symlinks.push(cvt_mimetype_iconfilename(mime.to_string()));
        status_list.push("s".to_string());
    }
    let mut merged_file_list = Vec::new();
    merged_file_list.append(&mut file_list);
    merged_file_list.append(&mut dir_list);
    merged_file_list.append(&mut symlink_list);
    let mut merged_icon_list = Vec::new();
    merged_icon_list.append(&mut list_icons_files);
    merged_icon_list.append(&mut list_icons_dir);
    merged_icon_list.append(&mut list_icons_symlinks);
    (merged_file_list, merged_icon_list, status_list)
}

// this just checks if the path is valid even if I do not have any permission of any kind over it.
pub fn path_exists(path: String, ignore: bool) -> (bool, bool) {
    if !ignore {
        match fs::metadata(path) {
            Ok(_) => (true, false),
            Err(e) => match e.kind() {
                ErrorKind::PermissionDenied => (false, true),
                _ => (false, false),
            },
        }
    } else {
        match fs::metadata(path) {
            Ok(_) => (true, false),
            Err(e) => (false, false),
        }
    }
}
