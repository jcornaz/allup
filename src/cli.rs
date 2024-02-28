use std::path::PathBuf;

/// Probe services and print the result
#[derive(Debug, clap::Parser)]
#[command(version, about)]
pub struct Opts {
    /// File containing the list of endpoints to probe
    #[clap(env = "ALLUP_FILE")]
    pub file: PathBuf,
    /// Output the result as JSON
    #[clap(short, long)]
    pub json: bool,
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
