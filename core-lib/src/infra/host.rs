use heim::host;
use heim::units::{frequency, information, thermodynamic_temperature, time};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StadalHost {
    pub name: String,
    pub release: String,
    pub version: String,
    pub hostname: String,
    pub arch: String,
    pub uptime: String,
}

pub async fn get_host() -> StadalHost {
    let mut result = StadalHost {
        name: "".to_string(),
        release: "".to_string(),
        version: "".to_string(),
        hostname: "".to_string(),
        arch: "".to_string(),
        uptime: "".to_string()
    };

    let (platform_result, uptime_result) =
        futures::future::join(host::platform(), host::uptime()).await;

    if let Ok(platform) = platform_result {
        result.name = platform.system().to_string();
        result.release = platform.release().to_string();
        result.version = platform.version().to_string();
        result.hostname = platform.hostname().to_string();
        result.arch = platform.architecture().as_str().to_string();
    }

    if let Ok(uptime) = uptime_result {
        let uptime = uptime.get::<time::second>().round() as i64;
        result.uptime = uptime.to_string();
    }

    result
}