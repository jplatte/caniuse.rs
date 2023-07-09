use xilem_html::{
    elements::{code, Code},
    OneOf2,
};

pub fn view_text(mut text: &str) -> Vec<OneOf2<&str, Code<&str>>> {
    let mut res = Vec::new();

    while let Some(backtick_pos) = text.find('`') {
        if backtick_pos != 0 {
            res.push(OneOf2::A(&text[..backtick_pos]));
        }
        text = &text[backtick_pos + 1..];

        let next_backtick_pos = match text.find('`') {
            Some(pos) => pos,
            None => {
                // This should never happen, backticks should be balanced
                break;
            }
        };

        res.push(OneOf2::B(code(&text[..next_backtick_pos])));
        text = &text[next_backtick_pos + 1..];
    }

    // Use the rest of the text verbatim
    res.push(OneOf2::A(text));
    res
}

/* pub fn home_button() -> Html {
    let classes: Classes = "button".into();
    html! {
        <RouterLink to={AppRoute::Index} classes={classes}>{fa_home()}</RouterLink>
    }
}*/
