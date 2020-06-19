extern crate log;
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

use std::io::Read;
use std::str;
use std::collections::HashMap;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

use yaserde::YaDeserialize;

use strum_macros::EnumString;

#[derive(Debug, YaDeserialize, EnumString, PartialEq)]
enum EventType {
	#[strum(serialize="0 BOOTSTRAP")]
	Bootstrap,
	#[strum(serialize="1 BOOT")]
	Boot,
	#[strum(serialize="2 PERIODIC")]
	Periodic,
	#[strum(serialize="3 SCHEDULED")]
	Scheduled,
	#[strum(serialize="4 VALUE CHANGE")]
	ValueChange,
	#[strum(serialize="5 KICKED")]
	Kicked,
	#[strum(serialize="6 CONNECTION REQUEST")]
	ConnectionRequest,
	#[strum(serialize="7 TRANSFER COMPLETE")]
	TransferComplete,
	#[strum(serialize="8 DIAGNOSTICS COMPLETE")]
	DiagnosticsComplete,
	#[strum(serialize="9 REQUEST DOWNLOAD")]
	RequestDownload,
	#[strum(serialize="10 AUTONOMOUS TRANSFER COMPLETE")]
	AutonomousTransferComplete,
	#[strum(serialize="11 DU STATE CHANGE COMPLETE")]
	DuStateChangeComplete,
	#[strum(serialize="12 AUTONOMOUS DU STATE CHANGE COMPLETE")]
	AutonomousDuStateChangeComplete,
	#[strum(serialize="13 WAKEUP")]
	Wakeup,
	#[strum(serialize="M Reboot")]
	Reboot,
	#[strum(serialize="M ScheduleInform")]
	ScheduleInform,
	#[strum(serialize="M Download")]
	Download,
	#[strum(serialize="M ScheduleDownload")]
	ScheduleDownload,
	#[strum(serialize="M Upload")]
	Upload,
	#[strum(serialize="M ChangeDUState")]
	ChangeDUState,
}

impl Default for EventType {
    fn default() -> Self { EventType::Bootstrap }
}

#[derive(Debug, Default, YaDeserialize, PartialEq)]
#[yaserde(
    prefix = "soap_env",
	namespace = "soap_env: http://schemas.xmlsoap.org/soap/envelope/",
	namespace = "soap_enc: http://schemas.xmlsoap.org/soap/encoding/",
	namespace = "xsd: http://www.w3.org/2001/XMLSchema",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	// namespace = "cwmp: urn:dslforum-org:cwmp-1-2",
)]
struct Envelope {
	#[yaserde(rename = "Body")]
	body: Body,
}

#[derive(Debug, Default, YaDeserialize, PartialEq)]
struct Body {
	#[yaserde(rename = "Inform")]
	inform: Inform
}

#[derive(Debug, Default, YaDeserialize, PartialEq)]
struct Inform {
	#[yaserde(rename = "RetryCount")]
	retry_count: u8,

	#[yaserde(rename = "DeviceId")]
	device_id: DeviceId,

	#[yaserde(rename = "Event")]
	events: Vec<EventStruct>
	// current_time: Datetime
	// parameter_list: Vec<Parameter>
}

#[derive(Debug, Default, YaDeserialize, PartialEq)]
struct DeviceId {
	#[yaserde(rename = "Manufacturer")]
	manufacturer: String,
	#[yaserde(rename = "OUI")]
	oui: String,
	#[yaserde(rename = "ProductClass")]
	product_class: String,
	#[yaserde(rename = "SerialNumber")]
	serial_number: String,
}

#[derive(Debug, Default, YaDeserialize, PartialEq)]
struct EventStruct {
	#[yaserde(rename = "EventCode")]
	event_code: EventType
}

async fn inform(req: HttpRequest, body: String) -> impl Responder {
	println!("{:?}", body);


	let inform: Envelope = yaserde::de::from_str(&body).unwrap();
	println!("{:?}", inform);

	"Hello"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	std::env::set_var("RUST_LOG", "debug,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
        	.wrap(Logger::default())
            .route("/", web::post().to(inform))
    })
    .bind("0.0.0.0:7547")?
    .run()
    .await
}