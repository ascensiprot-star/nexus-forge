use std::path::PathBuf;

pub struct Args {
    pub config_path: Option<PathBuf>,
    pub command: Command,
}

pub enum Command {
    Serve { port: u16 },
    Init { path: PathBuf },
    Status,
    Version,
}

pub fn parse_args() -> Args {
    let args: Vec<String> = std::env::args().collect();

    let mut config_path = None;
    let mut command = None;
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--config" | "-c" => {
                i += 1;
                if i < args.len() {
                    config_path = Some(PathBuf::from(&args[i]));
                }
            }
            "serve" => {
                let mut port = 3141u16;
                if i + 1 < args.len() {
                    if let Ok(p) = args[i + 1].parse::<u16>() {
                        port = p;
                        i += 1;
                    }
                }
                command = Some(Command::Serve { port });
            }
            "init" => {
                let path = if i + 1 < args.len() {
                    i += 1;
                    PathBuf::from(&args[i])
                } else {
                    PathBuf::from(".")
                };
                command = Some(Command::Init { path });
            }
            "status" => {
                command = Some(Command::Status);
            }
            "version" | "--version" | "-V" => {
                command = Some(Command::Version);
            }
            _ => {}
        }
        i += 1;
    }

    Args {
        config_path,
        command: command.unwrap_or(Command::Serve { port: 3141 }),
    }
}
