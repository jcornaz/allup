use std::io::{self, Write};

use colored::{Color, Colorize};

use crate::model::ProbeResult;

pub fn render(mut writer: impl Write, results: &[ProbeResult]) -> io::Result<()> {
    let name_width = results
        .iter()
        .map(|r| r.endpoint.name.len())
        .max()
        .unwrap_or_default();
    for result in results {
        render_one(&mut writer, name_width, result)?;
        writeln!(writer)?;
    }
    Ok(())
}

fn render_one(mut writer: impl Write, name_width: usize, result: &ProbeResult) -> io::Result<()> {
    let color = if result.error.is_some() {
        Color::Red
    } else {
        Color::Green
    };
    write!(
        writer,
        "{: <width$}",
        format!("{}: ", result.endpoint.name).color(color),
        width = name_width + 2,
    )?;
    match result.error.as_ref() {
        None => {
            write!(
                writer,
                "{} {}",
                "OK".bold().color(color),
                format!("({} ms)", result.duration).color(color).italic()
            )?;
        }
        Some(err) => {
            let msg = match err {
                crate::model::Error::Timeout => {
                    format!("TIME OUT ({} ms)", result.endpoint.timeout)
                }
                crate::model::Error::UnexpectedStatusCode(status) => format!("STATUS: {status}"),
                crate::model::Error::TooManyRedirects => "TOO MANY REDIRECTS".into(),
                crate::model::Error::Unreachable => "UNREACHABLE".into(),
            };
            write!(writer, "{}", msg.color(color).bold())?;
        }
    }
    Ok(())
}
