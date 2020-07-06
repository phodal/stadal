use crate::infra::language::{java_utils, lang_utils};

pub fn get_java_version() -> Option<String> {
    match run_get_java_version() {
        Some(java_version) => {
            let formatted_version = format_java_version(java_version)?;
            Some(formatted_version)
        }
        None => None,
    }
}

fn run_get_java_version() -> Option<String> {
    let java_command = match std::env::var("JAVA_HOME") {
        Ok(java_home) => format!("{}/bin/java", java_home),
        Err(_) => String::from("java"),
    };

    let output = lang_utils::exec_cmd(&java_command.as_str(), &["-Xinternalversion"])?;
    Some(format!("{}{}", output.stdout, output.stderr))
}

/// Extract the java version from `java_out`.
fn format_java_version(java_out: String) -> Option<String> {
    java_utils::parse_jre_version(&java_out).map(|result| format!("v{}", result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_java_version_openjdk() {
        let java_8 = String::from("OpenJDK 64-Bit Server VM (25.222-b10) for linux-amd64 JRE (1.8.0_222-b10), built on Jul 11 2019 10:18:43 by \"openjdk\" with gcc 4.4.7 20120313 (Red Hat 4.4.7-23)");
        let java_11 = String::from("OpenJDK 64-Bit Server VM (11.0.4+11-post-Ubuntu-1ubuntu219.04) for linux-amd64 JRE (11.0.4+11-post-Ubuntu-1ubuntu219.04), built on Jul 18 2019 18:21:46 by \"build\" with gcc 8.3.0");
        assert_eq!(format_java_version(java_11), Some(String::from("v11.0.4")));
        assert_eq!(format_java_version(java_8), Some(String::from("v1.8.0")));
    }
}