pub(crate) fn get_name(original_name: &str) -> String {
    original_name
        .split("::")
        .last()
        .unwrap()
        .to_string()
        .replace(">", "")
}

pub(crate) fn to_string_vec(msg_list: &mut Vec<impl Into<String>>) -> Vec<String> {
    msg_list.drain(..).map(|item| item.into()).collect()
}
