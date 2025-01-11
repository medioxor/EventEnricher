use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::time::Duration;

use ferrisetw::native::ExtendedDataItem;
use ferrisetw::EventRecord;
use ferrisetw::parser::Parser;
use ferrisetw::provider::Provider;
use ferrisetw::provider::TraceFlags;
use ferrisetw::schema::Schema;
use ferrisetw::schema_locator::SchemaLocator;
use ferrisetw::trace::UserTrace;

static N_EVENTS: AtomicU32 = AtomicU32::new(0);

fn etw_callback(record: &EventRecord, schema_locator: &SchemaLocator) {
    N_EVENTS.fetch_add(1, Ordering::SeqCst);

    match schema_locator.event_schema(record) {
        Err(err) => {
            println!("Unable to get the ETW schema for event: {:?}", err);
        }

        Ok(schema) => {
            parse_etw_event(&schema, record);
        }
    }
}

fn parse_etw_event(schema: &Schema, record: &EventRecord) {
    let parser = Parser::create(record, schema);
    match record.event_id() {
        1 => {
            let image_name: String = parser.try_parse("FilePath").unwrap();
            let command_line: String = parser.try_parse("CommandLine").unwrap();
            let process_id: u64 = parser.try_parse("ProcessId").unwrap();

            println!(
                "Process Start: \n\tPID: {}, \n\tImage: {}, \n\tCmd: {}",
                process_id, image_name, command_line
            );
        }

        2 => {
            let process_id: u64 = parser.try_parse("ProcessId").unwrap();
            let thread_id: u64 = parser.try_parse("ThreadId").unwrap();
            println!(
                "Thread Start: \n\tPID: {}, \n\tTID: {}",
                process_id, thread_id
            );
        }

        _ => {
            println!("Unknown event: {:?}", record.event_id());
        }
    }
    record.extended_data().iter().for_each(|edata| {
        let item = edata.to_extended_data_item();
        match item {
            ExtendedDataItem::StackTrace64(trace) => {
                println!("\tStack Trace:");
                for address in trace.addresses() {
                    println!("\t\t0x{:x}", address);
                }
            }

            _ => {}
        }
    });
}

fn main() {
    let event_enricher = Provider::by_guid("D8909C24-5BE9-4502-98CA-AB7BDC24899D")
        .add_callback(etw_callback)
        .trace_flags(TraceFlags::EVENT_ENABLE_PROPERTY_PROCESS_START_KEY)
        .trace_flags(TraceFlags::EVENT_ENABLE_PROPERTY_STACK_TRACE)
        .build();

    let trace = UserTrace::new()
        .named("EventEnricherService".to_owned())
        .enable(event_enricher)
        .start_and_process()
        .unwrap();

    std::thread::sleep(Duration::new(5, 0));

    trace.stop().unwrap(); // This is not required, as it will automatically be stopped on Drop
    println!("Done: {:?} events", N_EVENTS);
}
