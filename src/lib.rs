#![feature(const_fn)]
#![feature(const_vec_new)]

mod util {
    pub enum Either<L, R> {
        Left(L),
        Right(R),
    }
}

pub mod html {
    use crate::util::Either;

    use std::rc::Rc;

    pub enum ParseError {
        NotImplemented,
    }

    pub struct HtmlClass(String);

    pub trait ElementIdentifier {
        fn matches(&self, other_idx: usize, other: &Box<HtmlDomElement>)
                -> bool;
    }

    pub trait HtmlDomElement {
        fn children<'a>(&'a self) -> &'a Vec<Box<HtmlDomElement>>;
        fn parent<'a>(&'a self) -> &'a HtmlDomElement;
        fn siblings(&self) -> Vec<Rc<HtmlDomElement>>;
        fn add_child(&mut self, child: Box<HtmlDomElement>);
        fn remove_child(&mut self, child: Box<ElementIdentifier>);
        fn classes<'a>(&'a self) -> &'a Vec<HtmlClass>;
        fn name<'a>(&'a self) -> &'a str;
        fn id<'a>(&'a self) -> &'a str;
    }

    struct HtmlRootElement {
        children: Vec<Box<HtmlDomElement>>,
        classes: Vec<HtmlClass>,
        name: Either<String, &'static str>,
        id: Either<String, &'static str>,
    }

    impl HtmlRootElement {
        const fn new() -> HtmlRootElement {
            HtmlRootElement {
                children: Vec::new(),
                classes: Vec::new(),
                name: Either::Right(""),
                id: Either::Right(""),
            }
        }
    }

    impl HtmlDomElement for HtmlRootElement {
        fn children<'a>(&'a self) -> &'a Vec<Box<HtmlDomElement>> {
            &self.children
        }

        fn parent<'a>(&'a self) -> &'a HtmlDomElement {
            self
        }

        fn siblings(&self) -> Vec<Rc<HtmlDomElement>> {
            Vec::new()
        }

        fn add_child(&mut self, child: Box<HtmlDomElement>) {
            self.children.push(child);
        }

        fn remove_child(&mut self, child_id: Box<ElementIdentifier>) {
            let mut to_remove = None;
            for (i, child) in self.children.iter().enumerate() {
                if child_id.matches(i, child) {
                    to_remove = Some(i);
                }
            }
            match to_remove {
                Some(idx) => {
                    self.children.remove(idx);
                }
                None => {}
            };
        }

        fn classes<'a>(&'a self) -> &'a Vec<HtmlClass> {
            &self.classes
        }

        fn name<'a>(&'a self) -> &'a str {
            match &self.name {
                Either::Left(x) => &x,
                Either::Right(x) => x,
            }
        }

        fn id<'a>(&'a self) -> &'a str {
            match &self.id {
                Either::Left(x) => &x,
                Either::Right(x) => x,
            }
        }
    }

    pub struct HtmlDom {
        document: HtmlRootElement,
    }

    impl HtmlDom {
        const fn new() -> HtmlDom {
            HtmlDom {
                document: HtmlRootElement::new(),
            }
        }
    }

    struct HtmlParser {
        script_nesting_level: u8,
        parser_paused: bool,
    }

    impl HtmlParser {
        const fn new() -> HtmlParser {
            HtmlParser {
                script_nesting_level: 0,
                parser_paused: false,
            }
        }

        fn parse(&mut self, html_str: &str) -> Either<ParseError, HtmlDom> {
            let ret: HtmlDom = HtmlDom::new();
            let mut chars = html_str.chars();
            // TODO: implement
            Either::Left(ParseError::NotImplemented)
        }
    }

    pub fn parse_html(html_str: &str) -> Either<ParseError, HtmlDom> {
        HtmlParser::new().parse(html_str)
    }
}
