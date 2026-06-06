use std::{process::Command};

struct Preferences {
    host:String,
    os:String,
    kernel:String,
    uptime:String,
    shell:String,
    mem:String,
}

fn sh(command:&str) -> String {
    let take = Command::new("bash")
        .args(["-c", command])
        .output()
        .expect("An error has occured while trying to get object");

    String::from_utf8_lossy(&take.stdout).trim().to_string()
}

fn take_config() -> Preferences {
    Preferences {
        host:sh(r#"echo "$USER@$(cat /etc/hostname)""#),
        os:sh(r#"echo "$(grep '^PRETTY_NAME=' /etc/os-release | cut -d'"' -f2) $(uname -m)""#),
        kernel:sh(r#"uname -s -r"#),
        uptime:sh(r#"uptime -p"#),
        shell:sh(r#"zsh --version | cut -d' ' -f1,2"#),
        mem:sh(r#"free -m | grep Mem | awk '{print $3 "MB / " $2 "MB"}'"#),
    }
}

fn main() {
    let config = take_config();
    let script = format!(r#"
        /\         {}
       //\\        {}
  ____//__\\____   OS: {}
  \.-//----\\-,/   Kernel: {}
   \v/      \v/    Uptime: {}
   /\\      //\    Shell: {}
  //_\\____//_\\   Memory: {}
 '----\\--//----`
       \\//
        \/
        "#,
        config.host,
        String::from("-").repeat(config.host.chars().count()),
        config.os,
        config.kernel,
        config.uptime,
        config.shell,
        config.mem);
    println!("{}",script)
}
