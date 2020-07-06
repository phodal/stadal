use crate::infra::language::lang_utils;

pub fn get_ruby_version() -> Option<String> {
    let ruby_version = lang_utils::exec_cmd("ruby", &["-v"])?.stdout;
    let formatted_version = format_ruby_version(&ruby_version)?;
    Some(formatted_version)
}

fn format_ruby_version(ruby_version: &str) -> Option<String> {
    let version = ruby_version
        // split into ["ruby", "2.6.0p0", "linux/amd64"]
        .split_whitespace()
        // return "2.6.0p0"
        .nth(1)?
        .get(0..5)?;

    let mut formatted_version = String::with_capacity(version.len() + 1);
    formatted_version.push('v');
    formatted_version.push_str(version);
    Some(formatted_version)
}

