use phf::{Map, phf_map, phf_set, Set};

pub const FORM_TAGS: Set<&'static str> = phf_set! {
    "input",
    "option",
    "optgroup",
    "select",
    "button",
    "datalist",
    "textarea",
};

pub const P_TAG: Set<&'static str> = phf_set! {
    "p"
};

pub const TABLE_SECTION_TAGS : Set<&'static str> = phf_set! {
    "thead",
    "tbody"
};

pub const DD_DT_TAGS: Set<&'static str> = phf_set! {
    "dd", "dt"
};

pub const RTP_TAGS : Set<&'static str> = phf_set! {
    "rt", "rp"
};


pub const OPEN_IMPLIES_CLOSE : Map<&'static str, Set<&'static str>> = phf_map! {
    "tr" => phf_set! {"tr", "th", "td" },
    "th" => phf_set! {"th" },
    "td" => phf_set! {"thead", "th", "td" },
    "body" => phf_set! {"head", "link", "script" },
    "li" => phf_set! {"li" },
    "p" => P_TAG,
    "h1" => P_TAG,
    "h2" => P_TAG,
    "h3" => P_TAG,
    "h4" => P_TAG,
    "h5" => P_TAG,
    "h6" => P_TAG,
    "select" => FORM_TAGS,
    "input" => FORM_TAGS,
    "output" => FORM_TAGS,
    "button" => FORM_TAGS,
    "datalist" => FORM_TAGS,
    "textarea" => FORM_TAGS,
    "option" => phf_set! { "option" },
    "optgroup" => phf_set! {"optgroup", "option" },
    "dd" => DD_DT_TAGS,
    "dt" => DD_DT_TAGS,
    "address" => P_TAG,
    "article" => P_TAG,
    "aside" => P_TAG,
    "blockquote" => P_TAG,
    "details" => P_TAG,
    "div" => P_TAG,
    "dl" => P_TAG,
    "fieldset" => P_TAG,
    "figcaption" => P_TAG,
    "figure" => P_TAG,
    "footer" => P_TAG,
    "form" => P_TAG,
    "header" => P_TAG,
    "hr" => P_TAG,
    "main" => P_TAG,
    "nav" => P_TAG,
    "ol" => P_TAG,
    "pre" => P_TAG,
    "section" => P_TAG,
    "table" => P_TAG,
    "ul" => P_TAG,
    "rt" => RTP_TAGS,
    "rp" => RTP_TAGS,
    "tbody" => TABLE_SECTION_TAGS,
    "tfoot" => TABLE_SECTION_TAGS,
};

pub const VOID_ELEMENTS: Set<&'static str> = phf_set! {
    "area",
    "base",
    "basefont",
    "br",
    "col",
    "command",
    "embed",
    "frame",
    "hr",
    "img",
    "input",
    "isindex",
    "keygen",
    "link",
    "meta",
    "param",
    "source",
    "track",
    "wbr",
};

pub const FOREIGN_CONTEXT_ELEMENTS: Set<&'static str> = phf_set! {"math", "svg" };

pub const HTML_INTEGRATION_ELEMENTS: Set<&'static str> = phf_set! {
    "mi",
    "mo",
    "mn",
    "ms",
    "mtext",
    "annotation-xml",
    "foreignobject",
    "desc",
    "title",
};
