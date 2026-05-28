use std::fmt;
use tracing::Level;
use tracing_subscriber::fmt::{FormatEvent, FormatFields};
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::fmt::format::Writer;
use chrono::Local;
use tracing_subscriber::registry::LookupSpan;
use nu_ansi_term::Color;

struct CustomTime;

impl FormatTime for CustomTime {
    fn format_time(&self, w: &mut Writer<'_>) -> fmt::Result {
        let now = Local::now();
        write!(w, "{}", now.format("%Y-%m-%d_%H:%M:%S%.3f"))
    }
}

struct CustomFormatter;

impl <S, N> FormatEvent<S, N> for CustomFormatter
where 
    S: tracing::Subscriber + for <'a> LookupSpan <'a> ,
    N: for <'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &tracing_subscriber::fmt::FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> fmt::Result
    {
        let mut make_log = || -> fmt::Result {
            let metadata = event.metadata();
            let level = *metadata.level();
    
            let color = match level {
                Level::ERROR => Color::Red,
                Level::WARN => Color::Yellow,
                Level::INFO => Color::Green,
                Level::DEBUG => Color::Cyan,
                Level::TRACE => Color::LightBlue,
            };
    
            CustomTime.format_time(&mut writer)?;  
        
            write!(writer, "{}", color.prefix())?;
    
            write!(writer, " ")?;
    
            write!(writer, "{:<5} {}:{}> ", level.to_string(), metadata.module_path().unwrap_or("unknown"), metadata.line().unwrap_or(0))?;
    
            ctx.field_format().format_fields(writer.by_ref(), event)?;
            writeln!(writer, "{}", color.suffix())?;
            Ok(())
        };

        if let Err(e) = make_log() {
            eprintln!("logging system internal error: {}", e);   
        }
        
        Ok(())
    }
}


pub fn init_logging(log_level: &str) {
    use tracing_subscriber::fmt;
    use tracing_subscriber::EnvFilter;

    fmt()
        .with_env_filter(EnvFilter::new(log_level))
        .event_format(CustomFormatter)
        .init();
}


#[cfg(test)]
mod tests {

}
