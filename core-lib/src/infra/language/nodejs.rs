use crate::infra::language::lang_utils;

pub fn get_nodejs_version() -> Option<String> {
    let node_version = lang_utils::exec_cmd("node", &["--version"]).unwrap().stdout;
    let formatted_version = node_version.trim();

    Some(String::from(formatted_version))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exec_with_output_stdout() {
        let string = get_nodejs_version();
    }
}