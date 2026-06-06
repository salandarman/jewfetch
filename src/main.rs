use std::{process::Command};

struct Preferences {
    host:String,
    os:String,
    kernel:String,
    uptime:String,
    shell:String,
    mem:String,
    shacharit:String,
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
    let shacharit_message = shacharit();

    Preferences {
        host:sh(r#"echo "$USER@$(cat /etc/hostname)""#),
        os:sh(r#"echo "$(grep '^PRETTY_NAME=' /etc/os-release | cut -d'"' -f2) $(uname -m)""#),
        kernel:sh(r#"uname -s -r"#),
        uptime:sh(r#"uptime -p"#),
        shell:sh(r#"zsh --version | cut -d' ' -f1,2"#),
        mem:sh(r#"free -m | grep Mem | awk '{print $3 "MB / " $2 "MB"}'"#),
        shacharit:shacharit_message,
    }
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
 '----\\--//----`  Shabbat: {}
       \\//        {}
        \/         {}
        "#,
        config.host,
        "-".repeat(config.host.chars().count()),
        config.os,
        config.kernel,
        config.uptime,
        config.shell,
        config.mem,
        config.shacharit,
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
