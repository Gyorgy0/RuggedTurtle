pub struct LocaleFolder<'a> {
    pub locale_en: &'a str,
    pub locale_hu: &'a str,
}

pub struct IncludedFiles<'a> {
    pub locales: LocaleFolder<'a>,
}

pub const FILES: IncludedFiles = IncludedFiles {
    locales: LOCALE_FILES,
};

const LOCALE_FILES: LocaleFolder = LocaleFolder {
    locale_en: include_str!("locale/locale_en.yml"),
    locale_hu: include_str!("locale/locale_hu.yml"),
};
