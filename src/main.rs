use std::{fs,process::Command};
use dirs;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
struct Preferences {
    host:String,
    os:String,
    kernel:String,
    uptime:String,
    shell:String,
    mem:String,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            host:"echo $USER@$(cat /etc/hostname)".to_string(),
            os:"echo \"$(grep '^PRETTY_NAME=' /etc/os-release | cut -d'\"' -f2) $(uname -m)\"".to_string(),
            kernel:"uname -s -r".to_string(),
            uptime:"uptime -p".to_string(),
            shell:"zsh --version | cut -d' ' -f1,2".to_string(),
            mem:"free -m | grep Mem | awk '{print $3 \"MB / \" $2 \"MB\"}'".to_string(),
        }
    }
}

fn sh(command:&str) -> String {
    let take = Command::new("bash")
        .args(["-c", command])
        .output()
        .expect("An error has occured while trying to get object");

    String::from_utf8_lossy(&take.stdout).trim().to_string()
}

fn shacharit() -> String {
    let get_time = sh("timedatectl show | grep -i RTCTimeUSec | awk '{print $3}'");
    let time_vec: Vec<&str> = get_time.split(':').collect();

    let hours: i32 = time_vec[0].parse().unwrap();
    let minutes: i32 = time_vec[1].parse().unwrap();

    let is_shacharit = if hours <= 9 && hours >= 6 {
        if hours == 6 && minutes >= 30 {
            true
        }
        else if hours == 6 && minutes <= 30 {
            false
        }
        else {
            true
        }
    }
    else {
        false
    };

    let min_all = hours*60+minutes;

    let hour_remainder = if 380-min_all > 0 {
        (380-min_all)/60
    }
    else {
        (380-min_all+1440)/60
    };

    let minute_remainder = if 380-min_all > 0 {
        (380-min_all)%60
    }
    else {
        (380-min_all+1440)%60
    };

    let shacharit_message = if is_shacharit == true {
        format!("its time to pray!")
    }
    else {
        format!("{} hours, {} minutes left",hour_remainder,minute_remainder)
    };

    shacharit_message
}

fn take_config() -> Preferences {
    if let Some(home) = dirs::home_dir() {
        let config_path = home.join(".config").join("jewfetch").join("config.json");

        if let Ok(json_content) = fs::read_to_string(config_path) {
            if let Ok(settings) = serde_json::from_str(&json_content) {
                return settings;
            }
        }
    }

    Preferences::default()
}

fn main() {
    let config = take_config();

    let blue = "\x1b[34m";
    let reset_color = "\x1b[0m";
    let space = "  ";

    let script = format!(r#"
        /\         {}
       //\\        {}
  ____//__\\____   OS: {}
  \.-//----\\-,/   Kernel: {}
   \v/      \v/    Uptime: {}
   /\\      //\    Shell: {}
  //_\\____//_\\   Memory: {}
 '----\\--//----`  Shacharit: {}
       \\//        {}
        \/         {}
        "#,
        sh(&config.host),
        "-".repeat(sh(&config.host).chars().count()),
        sh(&config.os),
        sh(&config.kernel),
        sh(&config.uptime),
        sh(&config.shell),
        sh(&config.mem),
        shacharit(),
        space,
        space);

    println!();
    for line in script.lines() {
        if line.is_empty() { continue; }
        else if line.len() > 16 {
            let (ascii, info) = line.split_at(16);
            println!("{}{}{}{}",blue,ascii,reset_color,info);
        }
    }
    println!();
}
