use crate::infra::language::lang_utils;

pub fn get_php_version() -> Option<String> {
    match lang_utils::exec_cmd(
        "php",
        &[
            "-r",
            "echo PHP_MAJOR_VERSION.'.'.PHP_MINOR_VERSION.'.'.PHP_RELEASE_VERSION;",
        ],
    ) {
        Some(php_cmd_output) => {
            let php_version = php_cmd_output.stdout;
            let formatted_version = format_php_version(&php_version)?;

            Some(formatted_version)
        }
        None => None,
    }
}

fn format_php_version(php_version: &str) -> Option<String> {
    let mut formatted_version = String::with_capacity(php_version.len() + 1);
    formatted_version.push('v');
    formatted_version.push_str(php_version);
    Some(formatted_version)
}