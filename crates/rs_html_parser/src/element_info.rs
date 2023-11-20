pub fn is_form_tag(tag_name: &str) -> bool {
    matches!(
        tag_name,
        "input" | "option" | "optgroup" | "select" | "button" | "datalist" | "textarea"
    )
}

pub fn is_p_tag(tag_name: &str) -> bool {
    tag_name == "p"
}

pub fn is_table_section_tags(tag_name: &str) -> bool {
    matches!(tag_name, "thead" | "tbody")
}

pub fn is_dd_dt_tags(tag_name: &str) -> bool {
    matches!(tag_name, "dd" | "dt")
}

pub fn is_rtp_tags(tag_name: &str) -> bool {
    matches!(tag_name, "rt" | "rp")
}

pub fn is_void_elements(tag_name: &str) -> bool {
    matches!(
        tag_name,
        "area"
            | "base"
            | "basefont"
            | "br"
            | "col"
            | "command"
            | "embed"
            | "frame"
            | "hr"
            | "img"
            | "input"
            | "isindex"
            | "keygen"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}

pub fn is_tr_th_td(tag_name: &str) -> bool {
    matches!(tag_name, "tr" | "th" | "td")
}

pub fn is_th(tag_name: &str) -> bool {
    tag_name == "th"
}

pub fn is_thead_th_td(tag_name: &str) -> bool {
    matches!(tag_name, "thead" | "th" | "td")
}

pub fn is_head_link_script(tag_name: &str) -> bool {
    matches!(tag_name, "head" | "link" | "script")
}

pub fn is_li(tag_name: &str) -> bool {
    tag_name == "li"
}

pub fn is_option(tag_name: &str) -> bool {
    tag_name == "option"
}

pub fn is_opt_group(tag_name: &str) -> bool {
    matches!(tag_name, "optgroup" | "option")
}

pub fn open_implies_close(tag_name: &str) -> Option<fn(tag_name: &str) -> bool> {
    match tag_name {
        "tr" => Some(is_tr_th_td),
        "th" => Some(is_th),
        "td" => Some(is_thead_th_td),
        "body" => Some(is_head_link_script),
        "li" => Some(is_li),
        "p" => Some(is_p_tag),
        "h1" => Some(is_p_tag),
        "h2" => Some(is_p_tag),
        "h3" => Some(is_p_tag),
        "h4" => Some(is_p_tag),
        "h5" => Some(is_p_tag),
        "h6" => Some(is_p_tag),
        "select" => Some(is_form_tag),
        "input" => Some(is_form_tag),
        "output" => Some(is_form_tag),
        "button" => Some(is_form_tag),
        "datalist" => Some(is_form_tag),
        "textarea" => Some(is_form_tag),
        "option" => Some(is_option),
        "optgroup" => Some(is_opt_group),
        "dd" => Some(is_dd_dt_tags),
        "dt" => Some(is_dd_dt_tags),
        "address" => Some(is_p_tag),
        "article" => Some(is_p_tag),
        "aside" => Some(is_p_tag),
        "blockquote" => Some(is_p_tag),
        "details" => Some(is_p_tag),
        "div" => Some(is_p_tag),
        "dl" => Some(is_p_tag),
        "fieldset" => Some(is_p_tag),
        "figcaption" => Some(is_p_tag),
        "figure" => Some(is_p_tag),
        "footer" => Some(is_p_tag),
        "form" => Some(is_p_tag),
        "header" => Some(is_p_tag),
        "hr" => Some(is_p_tag),
        "main" => Some(is_p_tag),
        "nav" => Some(is_p_tag),
        "ol" => Some(is_p_tag),
        "pre" => Some(is_p_tag),
        "section" => Some(is_p_tag),
        "table" => Some(is_p_tag),
        "ul" => Some(is_p_tag),
        "rt" => Some(is_rtp_tags),
        "rp" => Some(is_rtp_tags),
        "tbody" => Some(is_table_section_tags),
        "tfoot" => Some(is_table_section_tags),
        _ => None,
    }
}

pub fn is_foreign_context_elements(tag_name: &str) -> bool {
    matches!(tag_name, "math" | "svg")
}

pub fn is_html_integration_elements(tag_name: &str) -> bool {
    matches!(
        tag_name,
        "mi" | "mo" | "mn" | "ms" | "mtext" | "annotation-xml" | "foreignobject" | "desc" | "title"
    )
}
