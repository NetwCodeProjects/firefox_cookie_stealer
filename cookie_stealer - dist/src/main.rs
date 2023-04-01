use rusqlite::{params, Connection};
use dirs_next;

fn main() {
    let home_dir = dirs_next::home_dir().expect("Failed to get the home directory");
    let profile_base_dir = if cfg!(windows) {
        home_dir.join("AppData").join("Roaming").join("Mozilla").join("Firefox")
    } else {
        home_dir.join(".mozilla").join("firefox")
    };
    let profile_dir = profile_base_dir.join(get_firefox_profile_name());
    let cookie_db_path = profile_dir.join("cookies.sqlite");
    println!("Opening cookie database at: {:?}", cookie_db_path);
    let conn = Connection::open(cookie_db_path).unwrap();

    let mut stmt = conn.prepare("SELECT name, value, host FROM moz_cookies").unwrap();
    let cookies = stmt.query_map(params![], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?))
    }).unwrap();

    for cookie_result in cookies {
        let cookie = cookie_result.unwrap();
        let name: String = cookie.0;
        let value: String = cookie.1;
        let host: String = cookie.2;

        println!("Name: {}, Value: {}, Host: {}", name, value/*String::from_utf8_lossy(&decrypted_value)*/, host);
    }
}

fn get_firefox_profile_name() -> String {
    let home_dir = dirs_next::home_dir().expect("Failed to get the home directory");
    let profile_base_dir = if cfg!(windows) {
        home_dir.join("AppData").join("Roaming").join("Mozilla").join("Firefox")
    } else {
        home_dir.join(".mozilla").join("firefox")
    };
    let profiles_ini_path = profile_base_dir.join("profiles.ini");

    let profiles_ini = std::fs::read_to_string(profiles_ini_path).unwrap();
    let default_profile_path = profiles_ini
        .lines()
        .filter(|line| line.starts_with("Path="))
        .nth(1)
        .unwrap()
        .split("=")
        .nth(1)
        .unwrap();

    default_profile_path.to_string()
}