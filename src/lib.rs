use std::borrow::Cow;

pub mod elements;
pub mod functions;

pub enum Child {
    Element(Box<dyn Element>),
    Text(String),
}

pub trait Element {
    fn attributes(&self) -> &Vec<Attribute>;
    fn children(&self) -> &Vec<Child>;
    fn name(&self) -> &'static str;

    fn child<E: Element + 'static>(self, element: E) -> Self
        where
            Self: Sized;

    fn attribute(self, attr: Attribute) -> Self
        where
            Self: Sized;

    fn text(self, text: String) -> Self
        where
            Self: Sized,
    {
        // TODO do sanitization
        unsafe { self.text_unchecked(text) }
    }

    unsafe fn text_unchecked(self, text: String) -> Self
        where
            Self: Sized;

    fn build(&self) -> yew::virtual_dom::VNode {
        use yew::virtual_dom::*;

        let mut vtag = VTag::new(Cow::Borrowed(self.name()));
        let attrs = self
            .attributes()
            .iter()
            .map(|it| PositionalAttr::new(it.0, it.1.clone()))
            .collect::<Vec<_>>();
        vtag.attributes = Attributes::Vec(attrs);

        for child in self.children() {
            match child {
                Child::Element(element) => {
                    let vnode = element.build();
                    vtag.add_child(vnode);
                }
                Child::Text(text) => {
                    let vtext = VText::new(Cow::Owned(text.clone()));
                    vtag.add_child(VNode::from(vtext))
                }
            }
        }
        VNode::from(vtag)
    }
}

pub struct Attribute(&'static str, Cow<'static, str>);

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    use crate::functions::*;
    use crate::Element;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn it_works() {
        use yew::prelude::*;

        struct Comp;

        impl Component for Comp {
            type Message = ();
            type Properties = ();

            fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
                Self
            }

            fn update(&mut self, msg: Self::Message) -> ShouldRender {
                false
            }

            fn change(&mut self, _props: Self::Properties) -> ShouldRender {
                false
            }

            fn view(&self) -> Html {
                div().build()
            }
        }

        yew::start_app::<Comp>();

        assert_eq!(yew::utils::document().body().unwrap().inner_html(), r#"<div></div>"#)
    }
}
