use std::{path::Path, env, fs};

use anyhow::bail;


fn default_user_agent(_:env::VarError) -> String {
    let authors = env::var("CARGO_PKG_AUTHORS").unwrap();
    let author = authors.split(",").next().unwrap();
    let email = author.split("<").last().unwrap().trim().split(">").next().unwrap();

    return format!(
        "{pkg_name}/{pkg_version} ({pkg_homepage}; {email})",
        pkg_name = env::var("CARGO_PKG_NAME").unwrap(),
        pkg_version = env::var("CARGO_PKG_VERSION").unwrap(),
        pkg_homepage = env::var("CARGO_PKG_HOMEPAGE").unwrap(),
        email = email
    )
}

fn connections(param: String) -> Vec<String> {
    param.trim().split(";").map(|s| {
        let fromto: Vec<&str> = s.trim().split(",").collect();
        format!("crate::config::Connection{{from:\"{}\",to:\"{}\"}}", fromto[0].trim(), fromto[1].trim())
    }).collect::<Vec<String>>()
}

fn main() -> anyhow::Result<()> {

    let wifi_ssid = env::var("PAPERTRAIN_WIFI_SSID").unwrap_or("".to_owned());
    if wifi_ssid.is_empty() {
        bail!("PAPERTRAIN_WIFI_SSID is required");
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("user_config.rs");

    let auth_method = match env::var("PAPERTRAIN_WIFI_AUTH_METHOD").unwrap_or("".to_owned()).as_ref() {
        "wep" => "embedded_svc::wifi::AuthMethod::WEP",
        "wpa" => "embedded_svc::wifi::AuthMethod::WPA",
        "wpa2personal" => "embedded_svc::wifi::AuthMethod::WPA2Personal",
        "wpawpa2personal" => "embedded_svc::wifi::AuthMethod::WPAWPA2Personal",
        "wpa2enterprise" => "embedded_svc::wifi::AuthMethod::WPA2Enterprise",
        "wpa3personal" => "embedded_svc::wifi::AuthMethod::WPA3Personal",
        "wpa2wpa3personal" => "embedded_svc::wifi::AuthMethod::WPA2WPA3Personal",
        "wapipersonal" => "embedded_svc::wifi::AuthMethod::WAPIPersonal",
        _ => "embedded_svc::wifi::AuthMethod::None"
    };

    let wifi_password: String = env::var("PAPERTRAIN_WIFI_PASSWORD").unwrap_or("".to_owned());

    if !auth_method.ends_with("None") && wifi_password.is_empty() {
        bail!("PAPERTRAIN_WIFI_AUTH_METHOD '{}' requires PAPERTRAIN_WIFI_PASSWORD to be set", auth_method);
    }

    let connections = env::var("PAPERTRAIN_CONNECTIONS").map_or(vec![], connections);

    fs::write(
        &dest_path,
         format!(r#"
const CONFIG: crate::config::Config<{num_connections}> = crate::config::Config {{
    wifi: crate::wifi::WifiConfig {{
        ssid: "{wifi_ssid}",
        password: "{wifi_password}",
        auth_method: {wifi_auth_method},
        channel: {wifi_channel}
    }},
    irail: crate::irail::IRailConfig {{
        url: "{irail_url}",
        user_agent: "{irail_user_agent}",
    }},
    connections: [{connections}]
}};
        "#,
        wifi_ssid = wifi_ssid,
        wifi_password = wifi_password,
        wifi_auth_method = auth_method,
        wifi_channel = env::var("PAPERTRAIN_IRAIL_URL").map_or("None".to_owned(), |c| format!("Some({})", c)),
        irail_url = env::var("PAPERTRAIN_IRAIL_URL").unwrap_or("https://api.irail.be".to_owned()),
        irail_user_agent = env::var("PAPERTRAIN_IRAIL_USER_AGENT").unwrap_or_else(default_user_agent),
        num_connections = connections.len(),
        connections = connections.join(",")
    )
    ).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=PAPERTRAIN_WIFI_SSID");
    println!("cargo:rerun-if-env-changed=PAPERTRAIN_WIFI_PASSWORD");
    println!("cargo:rerun-if-env-changed=PAPERTRAIN_WIFI_AUTH_METHOD");
    println!("cargo:rerun-if-env-changed=PAPERTRAIN_WIFI_CHANNEL");
    println!("cargo:rerun-if-env-changed=PAPERTRAIN_IRAIL_URL");
    println!("cargo:rerun-if-env-changed=PAPERTRAIN_IRAIL_USER_AGENT");
    println!("cargo:rerun-if-env-changed=PAPERTRAIN_CONNECTIONS");

    embuild::espidf::sysenv::output();

    Ok(())
}
