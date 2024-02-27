use std::path::PathBuf;

/// Probe services and print the result
#[derive(Debug, clap::Parser)]
#[command(version, about)]
pub struct Opts {
    pub file: PathBuf,
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use clap::Parser;

    use super::*;

    #[test]
    fn should_parse_file() {
        let opts = Opts::try_parse_from(["allup", "config.toml"]).unwrap();
        assert_eq!(opts.file, Path::new("config.toml"))
    }
}
