use std::fs::OpenOptions;

use tracing::{Level, Metadata, Subscriber};
use tracing_subscriber::{
    filter, fmt,
    layer::{Context, Filter, Layer},
    prelude::*,
    Registry,
};

use crate::language::lang;

const LOG_ERROR_FILENAME: &str = "log-error.log";
const LOG_DEBUG_FILENAME: &str = "log-debug.log";

struct DebugOnlyFilter;

impl<S> Filter<S> for DebugOnlyFilter {
    fn enabled(&self, meta: &Metadata<'_>, _: &Context<'_, S>) -> bool {
        meta.level() == &Level::DEBUG
    }
}

/*
 // Define a custom formatter
struct CustomFormatEvent;

impl<S, N> FormatEvent<S, N> for CustomFormatEvent
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> 'a + tracing::field::Visit,
{
    fn format_event(
        &self,
        ctx: &fmt::FmtContext<'_, S, N>,
        event: &tracing::Event<'_>,
    ) -> fmt::FormatEvent<'_, S, N> {
        // You can access the message and modify it here
        // For example, translate message based on some language setting
        // For simplicity, we'll just prepend a custom message
        let mut builder = ctx.new_event_format();

        // You could add custom logic here to translate or change messages
        // For example, replace the message based on language preference
        builder.event_format(move |writer, fields, _| {
            write!(writer, "[Custom Lang] ")?;
            fmt::format_fields(writer, fields)
        });

        builder.format_event(ctx, event)
    }
}
*/

pub(crate) fn create_subscriber() -> impl Subscriber + Send + Sync + 'static {
    // Creates error file.
    let err_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(LOG_ERROR_FILENAME)
        .expect(lang::LOG_ERROR_FILE_ERROR);

    // Creates debug file.
    let debug_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(LOG_DEBUG_FILENAME)
        .expect(lang::LOG_DEBUG_FILE_ERROR);

    Registry::default()
        .with(
            // stdout layer, to view everything in the console
            fmt::layer()
                // Add a format to print in another language the messages on
                // console.
                //.event_format(CustomFormatEvent)
                .compact()
                .with_ansi(true),
        )
        .with(
            // log-error file, to log the errors that arise
            fmt::layer()
                .json()
                .with_writer(err_file)
                .with_filter(filter::LevelFilter::from_level(Level::ERROR)),
        )
        .with(
            // log-debug file, to log the debug
            fmt::layer()
                .json()
                .with_writer(debug_file)
                .with_filter(DebugOnlyFilter),
        )
}
