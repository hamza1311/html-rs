use crate::Element;
pub fn html() -> crate::elements::Html {
    crate::elements::Html::new()
}

pub fn base() -> crate::elements::Base {
    crate::elements::Base::new()
}

pub fn head() -> crate::elements::Head {
    crate::elements::Head::new()
}

pub fn link() -> crate::elements::Link {
    crate::elements::Link::new()
}

pub fn meta() -> crate::elements::Meta {
    crate::elements::Meta::new()
}

pub fn style(value: String) -> crate::elements::Style {
    crate::elements::Style::new().text(value)
}

pub fn title(value: String) -> crate::elements::Title {
    crate::elements::Title::new().text(value)
}

pub fn body() -> crate::elements::Body {
    crate::elements::Body::new()
}

pub fn address() -> crate::elements::Address {
    crate::elements::Address::new()
}

pub fn article() -> crate::elements::Article {
    crate::elements::Article::new()
}

pub fn aside() -> crate::elements::Aside {
    crate::elements::Aside::new()
}

pub fn footer() -> crate::elements::Footer {
    crate::elements::Footer::new()
}

pub fn header() -> crate::elements::Header {
    crate::elements::Header::new()
}

pub fn main() -> crate::elements::Main {
    crate::elements::Main::new()
}

pub fn nav() -> crate::elements::Nav {
    crate::elements::Nav::new()
}

pub fn section() -> crate::elements::Section {
    crate::elements::Section::new()
}

pub fn blockquote() -> crate::elements::Blockquote {
    crate::elements::Blockquote::new()
}

pub fn dd() -> crate::elements::Dd {
    crate::elements::Dd::new()
}

pub fn div() -> crate::elements::Div {
    crate::elements::Div::new()
}

pub fn dl() -> crate::elements::Dl {
    crate::elements::Dl::new()
}

pub fn dt() -> crate::elements::Dt {
    crate::elements::Dt::new()
}

pub fn figcaption() -> crate::elements::Figcaption {
    crate::elements::Figcaption::new()
}

pub fn figure() -> crate::elements::Figure {
    crate::elements::Figure::new()
}

pub fn hr() -> crate::elements::Hr {
    crate::elements::Hr::new()
}

pub fn li() -> crate::elements::Li {
    crate::elements::Li::new()
}

pub fn ol() -> crate::elements::Ol {
    crate::elements::Ol::new()
}

pub fn p(value: String) -> crate::elements::P {
    crate::elements::P::new().text(value)
}

pub fn pre() -> crate::elements::Pre {
    crate::elements::Pre::new()
}

pub fn ul() -> crate::elements::Ul {
    crate::elements::Ul::new()
}

pub fn a(value: String) -> crate::elements::A {
    crate::elements::A::new().text(value)
}

pub fn abbr(value: String) -> crate::elements::Abbr {
    crate::elements::Abbr::new().text(value)
}

pub fn b(value: String) -> crate::elements::B {
    crate::elements::B::new().text(value)
}

pub fn bdi(value: String) -> crate::elements::Bdi {
    crate::elements::Bdi::new().text(value)
}

pub fn bdo() -> crate::elements::Bdo {
    crate::elements::Bdo::new()
}

pub fn br() -> crate::elements::Br {
    crate::elements::Br::new()
}

pub fn cite() -> crate::elements::Cite {
    crate::elements::Cite::new()
}

pub fn code(value: String) -> crate::elements::Code {
    crate::elements::Code::new().text(value)
}

pub fn data() -> crate::elements::Data {
    crate::elements::Data::new()
}

pub fn dfn(value: String) -> crate::elements::Dfn {
    crate::elements::Dfn::new().text(value)
}

pub fn em(value: String) -> crate::elements::Em {
    crate::elements::Em::new().text(value)
}

pub fn i(value: String) -> crate::elements::I {
    crate::elements::I::new().text(value)
}

pub fn kbd() -> crate::elements::Kbd {
    crate::elements::Kbd::new()
}

pub fn mark() -> crate::elements::Mark {
    crate::elements::Mark::new()
}

pub fn q() -> crate::elements::Q {
    crate::elements::Q::new()
}

pub fn rp() -> crate::elements::Rp {
    crate::elements::Rp::new()
}

pub fn rt() -> crate::elements::Rt {
    crate::elements::Rt::new()
}

pub fn ruby() -> crate::elements::Ruby {
    crate::elements::Ruby::new()
}

pub fn s() -> crate::elements::S {
    crate::elements::S::new()
}

pub fn samp() -> crate::elements::Samp {
    crate::elements::Samp::new()
}

pub fn small(value: String) -> crate::elements::Small {
    crate::elements::Small::new().text(value)
}

pub fn span() -> crate::elements::Span {
    crate::elements::Span::new()
}

pub fn strong(value: String) -> crate::elements::Strong {
    crate::elements::Strong::new().text(value)
}

pub fn sub() -> crate::elements::Sub {
    crate::elements::Sub::new()
}

pub fn sup() -> crate::elements::Sup {
    crate::elements::Sup::new()
}

pub fn time() -> crate::elements::Time {
    crate::elements::Time::new()
}

pub fn u() -> crate::elements::U {
    crate::elements::U::new()
}

pub fn var() -> crate::elements::Var {
    crate::elements::Var::new()
}

pub fn wbr() -> crate::elements::Wbr {
    crate::elements::Wbr::new()
}

pub fn area() -> crate::elements::Area {
    crate::elements::Area::new()
}

pub fn audio() -> crate::elements::Audio {
    crate::elements::Audio::new()
}

pub fn img() -> crate::elements::Img {
    crate::elements::Img::new()
}

pub fn map() -> crate::elements::Map {
    crate::elements::Map::new()
}

pub fn track() -> crate::elements::Track {
    crate::elements::Track::new()
}

pub fn video() -> crate::elements::Video {
    crate::elements::Video::new()
}

pub fn embed() -> crate::elements::Embed {
    crate::elements::Embed::new()
}

pub fn iframe() -> crate::elements::Iframe {
    crate::elements::Iframe::new()
}

pub fn object() -> crate::elements::Object {
    crate::elements::Object::new()
}

pub fn param() -> crate::elements::Param {
    crate::elements::Param::new()
}

pub fn picture() -> crate::elements::Picture {
    crate::elements::Picture::new()
}

pub fn portal() -> crate::elements::Portal {
    crate::elements::Portal::new()
}

pub fn source() -> crate::elements::Source {
    crate::elements::Source::new()
}

pub fn svg() -> crate::elements::Svg {
    crate::elements::Svg::new()
}

pub fn math() -> crate::elements::Math {
    crate::elements::Math::new()
}

pub fn canvas() -> crate::elements::Canvas {
    crate::elements::Canvas::new()
}

pub fn noscript() -> crate::elements::Noscript {
    crate::elements::Noscript::new()
}

pub fn script(value: String) -> crate::elements::Script {
    crate::elements::Script::new().text(value)
}

pub fn del() -> crate::elements::Del {
    crate::elements::Del::new()
}

pub fn ins() -> crate::elements::Ins {
    crate::elements::Ins::new()
}

pub fn caption() -> crate::elements::Caption {
    crate::elements::Caption::new()
}

pub fn col() -> crate::elements::Col {
    crate::elements::Col::new()
}

pub fn colgroup() -> crate::elements::Colgroup {
    crate::elements::Colgroup::new()
}

pub fn table() -> crate::elements::Table {
    crate::elements::Table::new()
}

pub fn tbody() -> crate::elements::Tbody {
    crate::elements::Tbody::new()
}

pub fn td() -> crate::elements::Td {
    crate::elements::Td::new()
}

pub fn tfoot() -> crate::elements::Tfoot {
    crate::elements::Tfoot::new()
}

pub fn th() -> crate::elements::Th {
    crate::elements::Th::new()
}

pub fn thead() -> crate::elements::Thead {
    crate::elements::Thead::new()
}

pub fn tr() -> crate::elements::Tr {
    crate::elements::Tr::new()
}

pub fn button(value: String) -> crate::elements::Button {
    crate::elements::Button::new().text(value)
}

pub fn datalist() -> crate::elements::Datalist {
    crate::elements::Datalist::new()
}

pub fn fieldset() -> crate::elements::Fieldset {
    crate::elements::Fieldset::new()
}

pub fn form() -> crate::elements::Form {
    crate::elements::Form::new()
}

pub fn input() -> crate::elements::Input {
    crate::elements::Input::new()
}

pub fn label(value: String) -> crate::elements::Label {
    crate::elements::Label::new().text(value)
}

pub fn legend() -> crate::elements::Legend {
    crate::elements::Legend::new()
}

pub fn meter() -> crate::elements::Meter {
    crate::elements::Meter::new()
}

pub fn optgroup() -> crate::elements::Optgroup {
    crate::elements::Optgroup::new()
}

pub fn option() -> crate::elements::Option {
    crate::elements::Option::new()
}

pub fn output() -> crate::elements::Output {
    crate::elements::Output::new()
}

pub fn progress() -> crate::elements::Progress {
    crate::elements::Progress::new()
}

pub fn select() -> crate::elements::Select {
    crate::elements::Select::new()
}

pub fn textarea() -> crate::elements::Textarea {
    crate::elements::Textarea::new()
}

pub fn details() -> crate::elements::Details {
    crate::elements::Details::new()
}

pub fn dialog() -> crate::elements::Dialog {
    crate::elements::Dialog::new()
}

pub fn menu() -> crate::elements::Menu {
    crate::elements::Menu::new()
}

pub fn summary() -> crate::elements::Summary {
    crate::elements::Summary::new()
}

pub fn slot() -> crate::elements::Slot {
    crate::elements::Slot::new()
}

pub fn template() -> crate::elements::Template {
    crate::elements::Template::new()
}

pub fn acronym() -> crate::elements::Acronym {
    crate::elements::Acronym::new()
}

pub fn applet() -> crate::elements::Applet {
    crate::elements::Applet::new()
}

pub fn basefont() -> crate::elements::Basefont {
    crate::elements::Basefont::new()
}

pub fn bgsound() -> crate::elements::Bgsound {
    crate::elements::Bgsound::new()
}

pub fn big() -> crate::elements::Big {
    crate::elements::Big::new()
}

pub fn blink() -> crate::elements::Blink {
    crate::elements::Blink::new()
}

pub fn center() -> crate::elements::Center {
    crate::elements::Center::new()
}

pub fn content() -> crate::elements::Content {
    crate::elements::Content::new()
}

pub fn dir() -> crate::elements::Dir {
    crate::elements::Dir::new()
}

pub fn font() -> crate::elements::Font {
    crate::elements::Font::new()
}

pub fn frame() -> crate::elements::Frame {
    crate::elements::Frame::new()
}

pub fn frameset() -> crate::elements::Frameset {
    crate::elements::Frameset::new()
}

pub fn hgroup() -> crate::elements::Hgroup {
    crate::elements::Hgroup::new()
}

pub fn image() -> crate::elements::Image {
    crate::elements::Image::new()
}

pub fn keygen() -> crate::elements::Keygen {
    crate::elements::Keygen::new()
}

pub fn marquee() -> crate::elements::Marquee {
    crate::elements::Marquee::new()
}

pub fn menuitem() -> crate::elements::Menuitem {
    crate::elements::Menuitem::new()
}

pub fn nobr() -> crate::elements::Nobr {
    crate::elements::Nobr::new()
}

pub fn noembed() -> crate::elements::Noembed {
    crate::elements::Noembed::new()
}

pub fn noframes() -> crate::elements::Noframes {
    crate::elements::Noframes::new()
}

pub fn plaintext() -> crate::elements::Plaintext {
    crate::elements::Plaintext::new()
}

pub fn rb() -> crate::elements::Rb {
    crate::elements::Rb::new()
}

pub fn rtc() -> crate::elements::Rtc {
    crate::elements::Rtc::new()
}

pub fn shadow() -> crate::elements::Shadow {
    crate::elements::Shadow::new()
}

pub fn spacer() -> crate::elements::Spacer {
    crate::elements::Spacer::new()
}

pub fn strike() -> crate::elements::Strike {
    crate::elements::Strike::new()
}

pub fn tt() -> crate::elements::Tt {
    crate::elements::Tt::new()
}

pub fn xmp() -> crate::elements::Xmp {
    crate::elements::Xmp::new()
}

pub fn h1(value: String) -> crate::elements::H1 {
    crate::elements::H1::new().text(value)
}

pub fn h2(value: String) -> crate::elements::H2 {
    crate::elements::H2::new().text(value)
}

pub fn h3(value: String) -> crate::elements::H3 {
    crate::elements::H3::new().text(value)
}

pub fn h4(value: String) -> crate::elements::H4 {
    crate::elements::H4::new().text(value)
}

pub fn h5(value: String) -> crate::elements::H5 {
    crate::elements::H5::new().text(value)
}

pub fn h6(value: String) -> crate::elements::H6 {
    crate::elements::H6::new().text(value)
}
