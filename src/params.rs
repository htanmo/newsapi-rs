#[derive(Debug, Default, Clone)]
pub struct TopHeadlinesParams {
    pub(crate) country: Option<String>,
    pub(crate) category: Option<String>,
    pub(crate) sources: Option<String>,
    pub(crate) q: Option<String>,
    pub(crate) page_size: Option<u32>,
    pub(crate) page: Option<u32>,
}

impl TopHeadlinesParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    pub fn sources(mut self, sources: impl Into<String>) -> Self {
        self.sources = Some(sources.into());
        self
    }

    pub fn q(mut self, query: impl Into<String>) -> Self {
        self.q = Some(query.into());
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page.max(1));
        self
    }

    pub fn page_size(mut self, size: u32) -> Self {
        self.page_size = Some(size.min(100));
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct EverythingParams {
    pub(crate) q: Option<String>,
    pub(crate) q_in_title: Option<String>,
    pub(crate) sources: Option<String>,
    pub(crate) domains: Option<String>,
    pub(crate) from: Option<String>,
    pub(crate) to: Option<String>,
    pub(crate) language: Option<String>,
    pub(crate) sort_by: Option<String>,
    pub(crate) page_size: Option<u32>,
    pub(crate) page: Option<u32>,
}

impl EverythingParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn q(mut self, query: impl Into<String>) -> Self {
        self.q = Some(query.into());
        self
    }

    pub fn q_in_title(mut self, query: impl Into<String>) -> Self {
        self.q_in_title = Some(query.into());
        self
    }

    pub fn sources(mut self, sources: impl Into<String>) -> Self {
        self.sources = Some(sources.into());
        self
    }

    pub fn from(mut self, from: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self
    }

    pub fn to(mut self, to: impl Into<String>) -> Self {
        self.to = Some(to.into());
        self
    }

    pub fn domains(mut self, domains: impl Into<String>) -> Self {
        self.domains = Some(domains.into());
        self
    }

    pub fn language(mut self, lang: impl Into<String>) -> Self {
        self.language = Some(lang.into());
        self
    }

    pub fn sort_by(mut self, sort: impl Into<String>) -> Self {
        self.sort_by = Some(sort.into());
        self
    }

    pub fn page_size(mut self, size: u32) -> Self {
        self.page_size = Some(size.min(100));
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page.max(1));
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct SourceParams {
    pub(crate) category: Option<String>,
    pub(crate) language: Option<String>,
    pub(crate) country: Option<String>,
}

impl SourceParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    pub fn language(mut self, lang: impl Into<String>) -> Self {
        self.language = Some(lang.into());
        self
    }

    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }
}
