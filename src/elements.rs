pub struct Html {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Html {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "html"
    }
}

impl Html {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn manifest(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("manifest", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Base {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Base {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "base"
    }
}

impl Base {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn href(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("href", attribute));
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn target(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("target", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Head {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Head {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "head"
    }
}

impl Head {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Link {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Link {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "link"
    }
}

impl Link {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn crossorigin(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("crossorigin", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn href(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("href", attribute));
        self
    }

    pub fn hreflang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("hreflang", attribute));
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn importance(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("importance", attribute));
        self
    }

    pub fn integrity(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("integrity", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn media(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("media", attribute));
        self
    }

    pub fn referrerpolicy(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("referrerpolicy", attribute));
        self
    }

    pub fn rel(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("rel", attribute));
        self
    }

    pub fn sizes(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("sizes", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Meta {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Meta {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "meta"
    }
}

impl Meta {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn charset(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("charset", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn content(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("content", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn http_equiv(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("http_equiv", attribute));
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn name(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("name", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Style {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Style {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "style"
    }
}

impl Style {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn media(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("media", attribute));
        self
    }

    pub fn scoped(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("scoped", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }

    pub fn r#type(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("type", attribute));
        self
    }
}

pub struct Title {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Title {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "title"
    }
}

impl Title {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Body {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Body {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "body"
    }
}

impl Body {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn background(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("background", attribute));
        self
    }

    pub fn bgcolor(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("bgcolor", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Address {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Address {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "address"
    }
}

impl Address {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Article {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Article {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "article"
    }
}

impl Article {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Aside {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Aside {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "aside"
    }
}

impl Aside {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Footer {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Footer {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "footer"
    }
}

impl Footer {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Header {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Header {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "header"
    }
}

impl Header {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Main {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Main {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "main"
    }
}

impl Main {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Nav {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Nav {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "nav"
    }
}

impl Nav {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Section {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Section {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "section"
    }
}

impl Section {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Blockquote {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Blockquote {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "blockquote"
    }
}

impl Blockquote {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn cite(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("cite", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Dd {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Dd {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "dd"
    }
}

impl Dd {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Div {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Div {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "div"
    }
}

impl Div {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Dl {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Dl {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "dl"
    }
}

impl Dl {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Dt {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Dt {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "dt"
    }
}

impl Dt {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Figcaption {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Figcaption {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "figcaption"
    }
}

impl Figcaption {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Figure {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Figure {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "figure"
    }
}

impl Figure {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Hr {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Hr {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "hr"
    }
}

impl Hr {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn align(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("align", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn color(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("color", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Li {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Li {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "li"
    }
}

impl Li {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }

    pub fn value(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("value", attribute));
        self
    }
}

pub struct Ol {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Ol {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "ol"
    }
}

impl Ol {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn reversed(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "reversed",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn start(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("start", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct P {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for P {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "p"
    }
}

impl P {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Pre {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Pre {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "pre"
    }
}

impl Pre {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Ul {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Ul {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "ul"
    }
}

impl Ul {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct A {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for A {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "a"
    }
}

impl A {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn download(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("download", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn href(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("href", attribute));
        self
    }

    pub fn hreflang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("hreflang", attribute));
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn media(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("media", attribute));
        self
    }

    pub fn ping(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("ping", attribute));
        self
    }

    pub fn referrerpolicy(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("referrerpolicy", attribute));
        self
    }

    pub fn rel(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("rel", attribute));
        self
    }

    pub fn shape(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("shape", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn target(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("target", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Abbr {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Abbr {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "abbr"
    }
}

impl Abbr {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct B {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for B {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "b"
    }
}

impl B {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Bdi {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Bdi {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "bdi"
    }
}

impl Bdi {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Bdo {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Bdo {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "bdo"
    }
}

impl Bdo {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Br {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Br {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "br"
    }
}

impl Br {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Cite {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Cite {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "cite"
    }
}

impl Cite {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Code {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Code {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "code"
    }
}

impl Code {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Data {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Data {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "data"
    }
}

impl Data {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }

    pub fn value(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("value", attribute));
        self
    }
}

pub struct Dfn {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Dfn {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "dfn"
    }
}

impl Dfn {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Em {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Em {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "em"
    }
}

impl Em {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct I {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for I {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "i"
    }
}

impl I {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Kbd {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Kbd {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "kbd"
    }
}

impl Kbd {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Mark {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Mark {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "mark"
    }
}

impl Mark {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Q {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Q {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "q"
    }
}

impl Q {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn cite(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("cite", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Rp {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Rp {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "rp"
    }
}

impl Rp {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Rt {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Rt {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "rt"
    }
}

impl Rt {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Ruby {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Ruby {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "ruby"
    }
}

impl Ruby {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct S {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for S {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "s"
    }
}

impl S {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Samp {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Samp {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "samp"
    }
}

impl Samp {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Small {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Small {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "small"
    }
}

impl Small {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Span {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Span {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "span"
    }
}

impl Span {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Strong {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Strong {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "strong"
    }
}

impl Strong {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Sub {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Sub {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "sub"
    }
}

impl Sub {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Sup {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Sup {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "sup"
    }
}

impl Sup {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Time {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Time {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "time"
    }
}

impl Time {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn datetime(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("datetime", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct U {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for U {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "u"
    }
}

impl U {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Var {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Var {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "var"
    }
}

impl Var {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Wbr {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Wbr {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "wbr"
    }
}

impl Wbr {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Area {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Area {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "area"
    }
}

impl Area {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn alt(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("alt", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn coords(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("coords", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn download(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("download", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn href(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("href", attribute));
        self
    }

    pub fn hreflang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("hreflang", attribute));
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn media(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("media", attribute));
        self
    }

    pub fn ping(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("ping", attribute));
        self
    }

    pub fn referrerpolicy(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("referrerpolicy", attribute));
        self
    }

    pub fn rel(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("rel", attribute));
        self
    }

    pub fn shape(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("shape", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn target(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("target", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Audio {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Audio {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "audio"
    }
}

impl Audio {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn autoplay(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "autoplay",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn buffered(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("buffered", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn controls(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "controls",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn crossorigin(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("crossorigin", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn r#loop(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "loop",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn muted(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "muted",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn preload(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("preload", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn src(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("src", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Img {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Img {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "img"
    }
}

impl Img {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn align(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("align", attribute));
        self
    }

    pub fn alt(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("alt", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn border(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("border", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn crossorigin(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("crossorigin", attribute));
        self
    }

    pub fn decoding(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("decoding", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn height(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("height", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn importance(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("importance", attribute));
        self
    }

    pub fn intrinsicsize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("intrinsicsize", attribute));
        self
    }

    pub fn ismap(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "ismap",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn loading(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("loading", attribute));
        self
    }

    pub fn referrerpolicy(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("referrerpolicy", attribute));
        self
    }

    pub fn sizes(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("sizes", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn src(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("src", attribute));
        self
    }

    pub fn srcset(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("srcset", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }

    pub fn usemap(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("usemap", attribute));
        self
    }

    pub fn width(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("width", attribute));
        self
    }
}

pub struct Map {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Map {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "map"
    }
}

impl Map {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn name(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("name", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Track {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Track {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "track"
    }
}

impl Track {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn default(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "default",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn kind(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("kind", attribute));
        self
    }

    pub fn label(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("label", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn src(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("src", attribute));
        self
    }

    pub fn srclang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("srclang", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Video {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Video {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "video"
    }
}

impl Video {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn autoplay(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "autoplay",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn buffered(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("buffered", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn controls(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "controls",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn crossorigin(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("crossorigin", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn height(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("height", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn r#loop(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "loop",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn muted(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "muted",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn poster(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("poster", attribute));
        self
    }

    pub fn preload(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("preload", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn src(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("src", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }

    pub fn width(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("width", attribute));
        self
    }
}

pub struct Embed {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Embed {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "embed"
    }
}

impl Embed {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn height(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("height", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn src(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("src", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }

    pub fn r#type(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("type", attribute));
        self
    }

    pub fn width(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("width", attribute));
        self
    }
}

pub struct Iframe {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Iframe {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "iframe"
    }
}

impl Iframe {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn align(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("align", attribute));
        self
    }

    pub fn allow(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("allow", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn csp(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("csp", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn height(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("height", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn importance(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("importance", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn loading(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("loading", attribute));
        self
    }

    pub fn name(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("name", attribute));
        self
    }

    pub fn referrerpolicy(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("referrerpolicy", attribute));
        self
    }

    pub fn sandbox(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("sandbox", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn src(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("src", attribute));
        self
    }

    pub fn srcdoc(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("srcdoc", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }

    pub fn width(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("width", attribute));
        self
    }
}

pub struct Object {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Object {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "object"
    }
}

impl Object {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn border(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("border", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn data(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("data", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn form(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("form", attribute));
        self
    }

    pub fn height(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("height", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn name(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("name", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }

    pub fn r#type(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("type", attribute));
        self
    }

    pub fn usemap(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("usemap", attribute));
        self
    }

    pub fn width(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("width", attribute));
        self
    }
}

pub struct Param {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Param {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "param"
    }
}

impl Param {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn name(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("name", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }

    pub fn value(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("value", attribute));
        self
    }
}

pub struct Picture {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Picture {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "picture"
    }
}

impl Picture {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Portal {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Portal {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "portal"
    }
}

impl Portal {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Source {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Source {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "source"
    }
}

impl Source {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn media(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("media", attribute));
        self
    }

    pub fn sizes(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("sizes", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn src(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("src", attribute));
        self
    }

    pub fn srcset(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("srcset", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }

    pub fn r#type(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("type", attribute));
        self
    }
}

pub struct Svg {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Svg {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "svg"
    }
}

impl Svg {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Math {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Math {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "math"
    }
}

impl Math {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Canvas {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Canvas {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "canvas"
    }
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn height(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("height", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }

    pub fn width(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("width", attribute));
        self
    }
}

pub struct Noscript {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Noscript {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "noscript"
    }
}

impl Noscript {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Script {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Script {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "script"
    }
}

impl Script {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn r#async(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "async",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn charset(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("charset", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn crossorigin(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("crossorigin", attribute));
        self
    }

    pub fn defer(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("defer", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn importance(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("importance", attribute));
        self
    }

    pub fn integrity(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("integrity", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn language(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("language", attribute));
        self
    }

    pub fn referrerpolicy(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("referrerpolicy", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn src(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("src", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }

    pub fn r#type(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("type", attribute));
        self
    }
}

pub struct Del {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Del {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "del"
    }
}

impl Del {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn cite(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("cite", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn datetime(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("datetime", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Ins {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Ins {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "ins"
    }
}

impl Ins {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn cite(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("cite", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn datetime(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("datetime", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Caption {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Caption {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "caption"
    }
}

impl Caption {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn align(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("align", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Col {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Col {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "col"
    }
}

impl Col {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn align(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("align", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn bgcolor(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("bgcolor", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn span(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("span", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Colgroup {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Colgroup {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "colgroup"
    }
}

impl Colgroup {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn align(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("align", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn bgcolor(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("bgcolor", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn span(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("span", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Table {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Table {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "table"
    }
}

impl Table {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn align(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("align", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn background(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("background", attribute));
        self
    }

    pub fn bgcolor(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("bgcolor", attribute));
        self
    }

    pub fn border(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("border", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn summary(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("summary", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Tbody {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Tbody {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "tbody"
    }
}

impl Tbody {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn align(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("align", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn bgcolor(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("bgcolor", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Td {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Td {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "td"
    }
}

impl Td {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn align(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("align", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn background(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("background", attribute));
        self
    }

    pub fn bgcolor(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("bgcolor", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn colspan(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("colspan", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn headers(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("headers", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn rowspan(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("rowspan", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Tfoot {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Tfoot {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "tfoot"
    }
}

impl Tfoot {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn align(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("align", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn bgcolor(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("bgcolor", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Th {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Th {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "th"
    }
}

impl Th {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn align(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("align", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn background(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("background", attribute));
        self
    }

    pub fn bgcolor(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("bgcolor", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn colspan(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("colspan", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn headers(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("headers", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn rowspan(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("rowspan", attribute));
        self
    }

    pub fn scope(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("scope", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Thead {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Thead {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "thead"
    }
}

impl Thead {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn align(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("align", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Tr {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Tr {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "tr"
    }
}

impl Tr {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn align(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("align", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn bgcolor(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("bgcolor", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Button {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Button {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "button"
    }
}

impl Button {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn autofocus(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "autofocus",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn disabled(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "disabled",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn form(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("form", attribute));
        self
    }

    pub fn formaction(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("formaction", attribute));
        self
    }

    pub fn formenctype(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("formenctype", attribute));
        self
    }

    pub fn formmethod(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("formmethod", attribute));
        self
    }

    pub fn formnovalidate(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "formnovalidate",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn formtarget(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("formtarget", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn name(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("name", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }

    pub fn r#type(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("type", attribute));
        self
    }

    pub fn value(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("value", attribute));
        self
    }
}

pub struct Datalist {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Datalist {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "datalist"
    }
}

impl Datalist {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Fieldset {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Fieldset {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "fieldset"
    }
}

impl Fieldset {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn disabled(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "disabled",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn form(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("form", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn name(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("name", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Form {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Form {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "form"
    }
}

impl Form {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accept(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("accept", attribute));
        self
    }

    pub fn accept_charset(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accept_charset", attribute));
        self
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn action(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("action", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn autocomplete(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocomplete", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn enctype(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("enctype", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn method(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("method", attribute));
        self
    }

    pub fn name(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("name", attribute));
        self
    }

    pub fn novalidate(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "novalidate",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn target(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("target", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Input {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Input {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "input"
    }
}

impl Input {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accept(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("accept", attribute));
        self
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn alt(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("alt", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn autocomplete(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocomplete", attribute));
        self
    }

    pub fn autofocus(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "autofocus",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn capture(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("capture", attribute));
        self
    }

    pub fn checked(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "checked",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn dirname(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dirname", attribute));
        self
    }

    pub fn disabled(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "disabled",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn form(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("form", attribute));
        self
    }

    pub fn formaction(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("formaction", attribute));
        self
    }

    pub fn formenctype(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("formenctype", attribute));
        self
    }

    pub fn formmethod(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("formmethod", attribute));
        self
    }

    pub fn formnovalidate(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "formnovalidate",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn formtarget(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("formtarget", attribute));
        self
    }

    pub fn height(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("height", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn list(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("list", attribute));
        self
    }

    pub fn max(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("max", attribute));
        self
    }

    pub fn maxlength(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("maxlength", attribute));
        self
    }

    pub fn minlength(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("minlength", attribute));
        self
    }

    pub fn min(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("min", attribute));
        self
    }

    pub fn multiple(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "multiple",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn name(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("name", attribute));
        self
    }

    pub fn pattern(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("pattern", attribute));
        self
    }

    pub fn placeholder(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("placeholder", attribute));
        self
    }

    pub fn readonly(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "readonly",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn required(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "required",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn size(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("size", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn src(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("src", attribute));
        self
    }

    pub fn step(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("step", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }

    pub fn r#type(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("type", attribute));
        self
    }

    pub fn usemap(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("usemap", attribute));
        self
    }

    pub fn value(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("value", attribute));
        self
    }

    pub fn width(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("width", attribute));
        self
    }
}

pub struct Label {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Label {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "label"
    }
}

impl Label {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn r#for(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("for", attribute));
        self
    }

    pub fn form(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("form", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Legend {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Legend {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "legend"
    }
}

impl Legend {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Meter {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Meter {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "meter"
    }
}

impl Meter {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn form(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("form", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn high(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("high", attribute));
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn low(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("low", attribute));
        self
    }

    pub fn max(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("max", attribute));
        self
    }

    pub fn min(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("min", attribute));
        self
    }

    pub fn optimum(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("optimum", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }

    pub fn value(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("value", attribute));
        self
    }
}

pub struct Optgroup {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Optgroup {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "optgroup"
    }
}

impl Optgroup {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn disabled(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "disabled",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn label(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("label", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Option {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Option {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "option"
    }
}

impl Option {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn disabled(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "disabled",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn label(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("label", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn selected(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "selected",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }

    pub fn value(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("value", attribute));
        self
    }
}

pub struct Output {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Output {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "output"
    }
}

impl Output {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn r#for(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("for", attribute));
        self
    }

    pub fn form(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("form", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn name(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("name", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Progress {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Progress {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "progress"
    }
}

impl Progress {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn form(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("form", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn max(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("max", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }

    pub fn value(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("value", attribute));
        self
    }
}

pub struct Select {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Select {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "select"
    }
}

impl Select {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn autocomplete(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocomplete", attribute));
        self
    }

    pub fn autofocus(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "autofocus",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn disabled(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "disabled",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn form(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("form", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn multiple(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "multiple",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn name(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("name", attribute));
        self
    }

    pub fn required(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "required",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn size(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("size", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Textarea {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Textarea {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "textarea"
    }
}

impl Textarea {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn autocomplete(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocomplete", attribute));
        self
    }

    pub fn autofocus(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "autofocus",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn cols(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("cols", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn dirname(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dirname", attribute));
        self
    }

    pub fn disabled(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "disabled",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn enterkeyhint(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("enterkeyhint", attribute));
        self
    }

    pub fn form(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("form", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn inputmode(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("inputmode", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn maxlength(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("maxlength", attribute));
        self
    }

    pub fn minlength(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("minlength", attribute));
        self
    }

    pub fn name(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("name", attribute));
        self
    }

    pub fn placeholder(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("placeholder", attribute));
        self
    }

    pub fn readonly(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "readonly",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn required(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "required",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn rows(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("rows", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }

    pub fn wrap(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("wrap", attribute));
        self
    }
}

pub struct Details {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Details {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "details"
    }
}

impl Details {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn open(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "open",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Dialog {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Dialog {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "dialog"
    }
}

impl Dialog {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Menu {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Menu {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "menu"
    }
}

impl Menu {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }

    pub fn r#type(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("type", attribute));
        self
    }
}

pub struct Summary {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Summary {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "summary"
    }
}

impl Summary {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Slot {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Slot {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "slot"
    }
}

impl Slot {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Template {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Template {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "template"
    }
}

impl Template {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Acronym {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Acronym {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "acronym"
    }
}

impl Acronym {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Applet {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Applet {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "applet"
    }
}

impl Applet {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn align(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("align", attribute));
        self
    }

    pub fn alt(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("alt", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn code(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("code", attribute));
        self
    }

    pub fn codebase(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("codebase", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Basefont {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Basefont {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "basefont"
    }
}

impl Basefont {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn color(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("color", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Bgsound {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Bgsound {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "bgsound"
    }
}

impl Bgsound {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn r#loop(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "loop",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Big {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Big {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "big"
    }
}

impl Big {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Blink {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Blink {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "blink"
    }
}

impl Blink {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Center {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Center {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "center"
    }
}

impl Center {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Content {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Content {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "content"
    }
}

impl Content {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Dir {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Dir {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "dir"
    }
}

impl Dir {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Font {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Font {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "font"
    }
}

impl Font {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn color(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("color", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Frame {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Frame {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "frame"
    }
}

impl Frame {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Frameset {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Frameset {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "frameset"
    }
}

impl Frameset {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Hgroup {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Hgroup {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "hgroup"
    }
}

impl Hgroup {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Image {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Image {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "image"
    }
}

impl Image {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Keygen {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Keygen {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "keygen"
    }
}

impl Keygen {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn autofocus(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "autofocus",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn challenge(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("challenge", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn disabled(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "disabled",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn form(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("form", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn keytype(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("keytype", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn name(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("name", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Marquee {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Marquee {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "marquee"
    }
}

impl Marquee {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn bgcolor(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("bgcolor", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn r#loop(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "loop",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Menuitem {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Menuitem {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "menuitem"
    }
}

impl Menuitem {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Nobr {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Nobr {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "nobr"
    }
}

impl Nobr {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Noembed {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Noembed {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "noembed"
    }
}

impl Noembed {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Noframes {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Noframes {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "noframes"
    }
}

impl Noframes {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Plaintext {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Plaintext {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "plaintext"
    }
}

impl Plaintext {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Rb {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Rb {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "rb"
    }
}

impl Rb {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Rtc {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Rtc {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "rtc"
    }
}

impl Rtc {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Shadow {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Shadow {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "shadow"
    }
}

impl Shadow {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Spacer {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Spacer {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "spacer"
    }
}

impl Spacer {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Strike {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Strike {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "strike"
    }
}

impl Strike {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Tt {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Tt {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "tt"
    }
}

impl Tt {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct Xmp {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for Xmp {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "xmp"
    }
}

impl Xmp {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct H1 {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for H1 {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "h1"
    }
}

impl H1 {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct H2 {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for H2 {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "h2"
    }
}

impl H2 {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct H3 {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for H3 {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "h3"
    }
}

impl H3 {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct H4 {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for H4 {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "h4"
    }
}

impl H4 {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct H5 {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for H5 {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "h5"
    }
}

impl H5 {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}

pub struct H6 {
    attributes: Vec<crate::Attribute>,
    children: Vec<crate::Child>,
}

impl crate::Element for H6 {
    fn attributes(&self) -> &Vec<crate::Attribute> {
        &self.attributes
    }

    fn children(&self) -> &Vec<crate::Child> {
        &self.children
    }

    fn child<E: crate::Element + 'static>(mut self, element: E) -> Self {
        self.children.push(crate::Child::Element(Box::new(element)));
        self
    }

    unsafe fn text_unchecked(mut self, text: String) -> Self {
        self.children.push(crate::Child::Text(text));
        self
    }

    fn attribute(mut self, attr: crate::Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    fn name(&self) -> &'static str {
        "h6"
    }
}

impl H6 {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("accesskey", attribute));
        self
    }

    pub fn autocapitalize(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("autocapitalize", attribute));
        self
    }

    pub fn class(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("class", attribute));
        self
    }

    pub fn contenteditable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contenteditable", attribute));
        self
    }

    pub fn contextmenu(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("contextmenu", attribute));
        self
    }

    pub fn dir(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("dir", attribute));
        self
    }

    pub fn draggable(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("draggable", attribute));
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        if value {
            self.attributes.push(crate::Attribute(
                "hidden",
                std::borrow::Cow::<'static, str>::Borrowed("true"),
            ));
        }
        self
    }

    pub fn id(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("id", attribute));
        self
    }

    pub fn itemprop(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("itemprop", attribute));
        self
    }

    pub fn lang(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("lang", attribute));
        self
    }

    pub fn slot(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("slot", attribute));
        self
    }

    pub fn spellcheck(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("spellcheck", attribute));
        self
    }

    pub fn style(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("style", attribute));
        self
    }

    pub fn tabindex(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("tabindex", attribute));
        self
    }

    pub fn title(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes.push(crate::Attribute("title", attribute));
        self
    }

    pub fn translate(mut self, attribute: std::borrow::Cow<'static, str>) -> Self {
        self.attributes
            .push(crate::Attribute("translate", attribute));
        self
    }
}
