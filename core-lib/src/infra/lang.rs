use crate::infra::language;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lang {
    name: String,
    version: String,
}

impl Lang {
    fn new(name: String, version: String) -> Lang {
        Lang {
            name,
            version
        }
    }
}

pub fn get_languages() -> Vec<Lang> {
    let mut languages = Vec::with_capacity(6);

    let java_version = language::get_java_version().unwrap();
    let php_version = language::get_php_version().unwrap();
    let ruby_version = language::get_ruby_version().unwrap();
    let nodejs_version = language::get_nodejs_version().unwrap();
    let python_version = language::get_python_version().unwrap();
    let golang_version = language::get_golang_version().unwrap();

    languages.push(Lang::new(String::from("java"), java_version));
    languages.push(Lang::new(String::from("php"), php_version));
    languages.push(Lang::new(String::from("ruby"), ruby_version));
    languages.push(Lang::new(String::from("nodejs"), nodejs_version));
    languages.push(Lang::new(String::from("python"), python_version));
    languages.push(Lang::new(String::from("golang"), golang_version));

    languages
}