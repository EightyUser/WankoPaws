#![windows_subsystem = "windows"]
#![allow(non_snake_case, unused)]

use raylib::prelude::*;
use vtubestudio::{Client, Error};
use vtubestudio::data::ParameterCreationRequest;
use vtubestudio::data::InjectParameterDataRequest;
use vtubestudio::data::ParameterValue;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use once_cell::sync::OnceCell;
use serde::Serialize;


#[tokio::main]
async fn main() -> Result<(), Error> {
	let C_VER = env!("CARGO_PKG_VERSION");
	let C_NAME = env!("CARGO_PKG_NAME");
	let C_AUTHOR = env!("CARGO_PKG_AUTHORS");
	let funny_cr = "Orginal Code by Maruseu (@nyaruseu). Fork by Eightyuser";
	let connVTS = true;

	let mut smooth = 0.0;
    let (mut rfButtPress, mut lfButtPress) = (0.0,0.0);
	let (mut shoulderLDown,mut shoulderRDown) = (0.0,0.0);
	let (mut thumbLStick,mut thumbRStick) = (0.0,0.0);


    let tokenPath = "./token";

	let mut storedToken = Some(
		match fs::read_to_string(tokenPath) {
			Ok(tok)=>{tok}
			Err(err)=>{"...".to_string()}
		}
	);
	
	let icon = Some(std::borrow::Cow::Borrowed("/9j/4QE6RXhpZgAATU0AKgAAAAgABwESAAMAAAABAAEAAAEaAAUAAAABAAAAYgEbAAUAAAABAAAAagEoAAMAAAABAAIAAAExAAIAAAAfAAAAcgEyAAIAAAAUAAAAkYdpAAQAAAABAAAAqAAAANQACvyAAAAnEAAK/IAAACcQQWRvYmUgUGhvdG9zaG9wIDIzLjEgKFdpbmRvd3MpADIwMjI6MDI6MDcgMjE6Mjg6MjQAAAAAAAOgAQADAAAAAf//AACgAgAEAAAAAQAAAICgAwAEAAAAAQAAAIAAAAAAAAAABgEDAAMAAAABAAYAAAEaAAUAAAABAAABIgEbAAUAAAABAAABKgEoAAMAAAABAAIAAAIBAAQAAAABAAABMgICAAQAAAABAAAAAAAAAAAAAABIAAAAAQAAAEgAAAAB/+0I3lBob3Rvc2hvcCAzLjAAOEJJTQQEAAAAAAAHHAIAAAIAAAA4QklNBCUAAAAAABDo8VzzL8EYoaJ7Z63FZNW6OEJJTQQ6AAAAAADlAAAAEAAAAAEAAAAAAAtwcmludE91dHB1dAAAAAUAAAAAUHN0U2Jvb2wBAAAAAEludGVlbnVtAAAAAEludGUAAAAAQ2xybQAAAA9wcmludFNpeHRlZW5CaXRib29sAAAAAAtwcmludGVyTmFtZVRFWFQAAAABAAAAAAAPcHJpbnRQcm9vZlNldHVwT2JqYwAAAAwAUAByAG8AbwBmACAAUwBlAHQAdQBwAAAAAAAKcHJvb2ZTZXR1cAAAAAEAAAAAQmx0bmVudW0AAAAMYnVpbHRpblByb29mAAAACXByb29mQ01ZSwA4QklNBDsAAAAAAi0AAAAQAAAAAQAAAAAAEnByaW50T3V0cHV0T3B0aW9ucwAAABcAAAAAQ3B0bmJvb2wAAAAAAENsYnJib29sAAAAAABSZ3NNYm9vbAAAAAAAQ3JuQ2Jvb2wAAAAAAENudENib29sAAAAAABMYmxzYm9vbAAAAAAATmd0dmJvb2wAAAAAAEVtbERib29sAAAAAABJbnRyYm9vbAAAAAAAQmNrZ09iamMAAAABAAAAAAAAUkdCQwAAAAMAAAAAUmQgIGRvdWJAb+AAAAAAAAAAAABHcm4gZG91YkBv4AAAAAAAAAAAAEJsICBkb3ViQG/gAAAAAAAAAAAAQnJkVFVudEYjUmx0AAAAAAAAAAAAAAAAQmxkIFVudEYjUmx0AAAAAAAAAAAAAAAAUnNsdFVudEYjUHhsQFIAAAAAAAAAAAAKdmVjdG9yRGF0YWJvb2wBAAAAAFBnUHNlbnVtAAAAAFBnUHMAAAAAUGdQQwAAAABMZWZ0VW50RiNSbHQAAAAAAAAAAAAAAABUb3AgVW50RiNSbHQAAAAAAAAAAAAAAABTY2wgVW50RiNQcmNAWQAAAAAAAAAAABBjcm9wV2hlblByaW50aW5nYm9vbAAAAAAOY3JvcFJlY3RCb3R0b21sb25nAAAAAAAAAAxjcm9wUmVjdExlZnRsb25nAAAAAAAAAA1jcm9wUmVjdFJpZ2h0bG9uZwAAAAAAAAALY3JvcFJlY3RUb3Bsb25nAAAAAAA4QklNA+0AAAAAABAASAAAAAEAAgBIAAAAAQACOEJJTQQmAAAAAAAOAAAAAAAAAAAAAD+AAAA4QklNBA0AAAAAAAQAAAAeOEJJTQQZAAAAAAAEAAAAHjhCSU0D8wAAAAAACQAAAAAAAAAAAQA4QklNJxAAAAAAAAoAAQAAAAAAAAACOEJJTQP1AAAAAABIAC9mZgABAGxmZgAGAAAAAAABAC9mZgABAKGZmgAGAAAAAAABADIAAAABAFoAAAAGAAAAAAABADUAAAABAC0AAAAGAAAAAAABOEJJTQP4AAAAAABwAAD/////////////////////////////A+gAAAAA/////////////////////////////wPoAAAAAP////////////////////////////8D6AAAAAD/////////////////////////////A+gAADhCSU0ECAAAAAAAEAAAAAEAAAJAAAACQAAAAAA4QklNBB4AAAAAAAQAAAAAOEJJTQQaAAAAAAM5AAAABgAAAAAAAAAAAAAAgAAAAIAAAAACAGkAYwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAgAAAAIAAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAAQAAAAAAAG51bGwAAAACAAAABmJvdW5kc09iamMAAAABAAAAAAAAUmN0MQAAAAQAAAAAVG9wIGxvbmcAAAAAAAAAAExlZnRsb25nAAAAAAAAAABCdG9tbG9uZwAAAIAAAAAAUmdodGxvbmcAAACAAAAABnNsaWNlc1ZsTHMAAAABT2JqYwAAAAEAAAAAAAVzbGljZQAAABIAAAAHc2xpY2VJRGxvbmcAAAAAAAAAB2dyb3VwSURsb25nAAAAAAAAAAZvcmlnaW5lbnVtAAAADEVTbGljZU9yaWdpbgAAAA1hdXRvR2VuZXJhdGVkAAAAAFR5cGVlbnVtAAAACkVTbGljZVR5cGUAAAAASW1nIAAAAAZib3VuZHNPYmpjAAAAAQAAAAAAAFJjdDEAAAAEAAAAAFRvcCBsb25nAAAAAAAAAABMZWZ0bG9uZwAAAAAAAAAAQnRvbWxvbmcAAACAAAAAAFJnaHRsb25nAAAAgAAAAAN1cmxURVhUAAAAAQAAAAAAAG51bGxURVhUAAAAAQAAAAAAAE1zZ2VURVhUAAAAAQAAAAAABmFsdFRhZ1RFWFQAAAABAAAAAAAOY2VsbFRleHRJc0hUTUxib29sAQAAAAhjZWxsVGV4dFRFWFQAAAABAAAAAAAJaG9yekFsaWduZW51bQAAAA9FU2xpY2VIb3J6QWxpZ24AAAAHZGVmYXVsdAAAAAl2ZXJ0QWxpZ25lbnVtAAAAD0VTbGljZVZlcnRBbGlnbgAAAAdkZWZhdWx0AAAAC2JnQ29sb3JUeXBlZW51bQAAABFFU2xpY2VCR0NvbG9yVHlwZQAAAABOb25lAAAACXRvcE91dHNldGxvbmcAAAAAAAAACmxlZnRPdXRzZXRsb25nAAAAAAAAAAxib3R0b21PdXRzZXRsb25nAAAAAAAAAAtyaWdodE91dHNldGxvbmcAAAAAADhCSU0EKAAAAAAADAAAAAI/8AAAAAAAADhCSU0EEQAAAAAAAQEAOEJJTQQUAAAAAAAEAAAAAThCSU0EIQAAAAAAVwAAAAEBAAAADwBBAGQAbwBiAGUAIABQAGgAbwB0AG8AcwBoAG8AcAAAABQAQQBkAG8AYgBlACAAUABoAG8AdABvAHMAaABvAHAAIAAyADAAMgAyAAAAAQA4QklNBAYAAAAAAAcAAQABAAEBAP/hFdNodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvADw/eHBhY2tldCBiZWdpbj0i77u/IiBpZD0iVzVNME1wQ2VoaUh6cmVTek5UY3prYzlkIj8+IDx4OnhtcG1ldGEgeG1sbnM6eD0iYWRvYmU6bnM6bWV0YS8iIHg6eG1wdGs9IkFkb2JlIFhNUCBDb3JlIDcuMS1jMDAwIDc5LmVkYTJiM2YsIDIwMjEvMTEvMTQtMTI6MzA6NDIgICAgICAgICI+IDxyZGY6UkRGIHhtbG5zOnJkZj0iaHR0cDovL3d3dy53My5vcmcvMTk5OS8wMi8yMi1yZGYtc3ludGF4LW5zIyI+IDxyZGY6RGVzY3JpcHRpb24gcmRmOmFib3V0PSIiIHhtbG5zOnhtcD0iaHR0cDovL25zLmFkb2JlLmNvbS94YXAvMS4wLyIgeG1sbnM6eG1wTU09Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9tbS8iIHhtbG5zOnN0RXZ0PSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvc1R5cGUvUmVzb3VyY2VFdmVudCMiIHhtbG5zOnN0UmVmPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvc1R5cGUvUmVzb3VyY2VSZWYjIiB4bWxuczpkYz0iaHR0cDovL3B1cmwub3JnL2RjL2VsZW1lbnRzLzEuMS8iIHhtbG5zOnBob3Rvc2hvcD0iaHR0cDovL25zLmFkb2JlLmNvbS9waG90b3Nob3AvMS4wLyIgeG1wOkNyZWF0b3JUb29sPSJBZG9iZSBQaG90b3Nob3AgMjMuMSAoV2luZG93cykiIHhtcDpDcmVhdGVEYXRlPSIyMDIyLTAxLTIzVDEzOjU1OjA4LTAzOjAwIiB4bXA6TWV0YWRhdGFEYXRlPSIyMDIyLTAyLTA3VDIxOjI4OjI0LTAzOjAwIiB4bXA6TW9kaWZ5RGF0ZT0iMjAyMi0wMi0wN1QyMToyODoyNC0wMzowMCIgeG1wTU06SW5zdGFuY2VJRD0ieG1wLmlpZDpkM2I2MWZlMC02YWNmLTRkNGMtOGE1Zi0wOWZkMDc0MGI1YzQiIHhtcE1NOkRvY3VtZW50SUQ9ImFkb2JlOmRvY2lkOnBob3Rvc2hvcDo4NzU4YWFkNi05ZjJjLWNkNGEtYTU3ZC1iOTI5M2NkNTRkZDkiIHhtcE1NOk9yaWdpbmFsRG9jdW1lbnRJRD0ieG1wLmRpZDphODAwZGY3Yi1hYTI5LTBjNGQtYWM0OC1hNmE4ZDg0MjZhOTEiIGRjOmZvcm1hdD0iaW1hZ2UvanBlZyIgcGhvdG9zaG9wOkNvbG9yTW9kZT0iMyI+IDx4bXBNTTpIaXN0b3J5PiA8cmRmOlNlcT4gPHJkZjpsaSBzdEV2dDphY3Rpb249ImNyZWF0ZWQiIHN0RXZ0Omluc3RhbmNlSUQ9InhtcC5paWQ6YTgwMGRmN2ItYWEyOS0wYzRkLWFjNDgtYTZhOGQ4NDI2YTkxIiBzdEV2dDp3aGVuPSIyMDIyLTAxLTIzVDEzOjU1OjA4LTAzOjAwIiBzdEV2dDpzb2Z0d2FyZUFnZW50PSJBZG9iZSBQaG90b3Nob3AgMjMuMSAoV2luZG93cykiLz4gPHJkZjpsaSBzdEV2dDphY3Rpb249InNhdmVkIiBzdEV2dDppbnN0YW5jZUlEPSJ4bXAuaWlkOjdlYzcwMzRiLWI3N2EtMDQ0Ni1hNzc0LTFiNzAwMTMwMGE0NCIgc3RFdnQ6d2hlbj0iMjAyMi0wMS0yM1QxMzo1NTowOC0wMzowMCIgc3RFdnQ6c29mdHdhcmVBZ2VudD0iQWRvYmUgUGhvdG9zaG9wIDIzLjEgKFdpbmRvd3MpIiBzdEV2dDpjaGFuZ2VkPSIvIi8+IDxyZGY6bGkgc3RFdnQ6YWN0aW9uPSJzYXZlZCIgc3RFdnQ6aW5zdGFuY2VJRD0ieG1wLmlpZDpiMjk1OWU3Mi0xNTM2LTUwNDgtYWI1OC0wNzZmOTNkZGMyNTEiIHN0RXZ0OndoZW49IjIwMjItMDItMDdUMjE6MjU6NTgtMDM6MDAiIHN0RXZ0OnNvZnR3YXJlQWdlbnQ9IkFkb2JlIFBob3Rvc2hvcCAyMy4xIChXaW5kb3dzKSIgc3RFdnQ6Y2hhbmdlZD0iLyIvPiA8cmRmOmxpIHN0RXZ0OmFjdGlvbj0iY29udmVydGVkIiBzdEV2dDpwYXJhbWV0ZXJzPSJmcm9tIGltYWdlL3BuZyB0byBpbWFnZS9qcGVnIi8+IDxyZGY6bGkgc3RFdnQ6YWN0aW9uPSJkZXJpdmVkIiBzdEV2dDpwYXJhbWV0ZXJzPSJjb252ZXJ0ZWQgZnJvbSBpbWFnZS9wbmcgdG8gaW1hZ2UvanBlZyIvPiA8cmRmOmxpIHN0RXZ0OmFjdGlvbj0ic2F2ZWQiIHN0RXZ0Omluc3RhbmNlSUQ9InhtcC5paWQ6NTIyN2ZjZDUtOTAwZi0yOTQ4LWFlOTktODc5YmE0OTlhNWFkIiBzdEV2dDp3aGVuPSIyMDIyLTAyLTA3VDIxOjI1OjU4LTAzOjAwIiBzdEV2dDpzb2Z0d2FyZUFnZW50PSJBZG9iZSBQaG90b3Nob3AgMjMuMSAoV2luZG93cykiIHN0RXZ0OmNoYW5nZWQ9Ii8iLz4gPHJkZjpsaSBzdEV2dDphY3Rpb249InNhdmVkIiBzdEV2dDppbnN0YW5jZUlEPSJ4bXAuaWlkOmM2ZTg5OWJmLTVkYjMtOGQ0YS1hZTczLWI4NTY4MGMyOWMxZCIgc3RFdnQ6d2hlbj0iMjAyMi0wMi0wN1QyMToyNzozNi0wMzowMCIgc3RFdnQ6c29mdHdhcmVBZ2VudD0iQWRvYmUgUGhvdG9zaG9wIDIzLjEgKFdpbmRvd3MpIiBzdEV2dDpjaGFuZ2VkPSIvIi8+IDxyZGY6bGkgc3RFdnQ6YWN0aW9uPSJjb252ZXJ0ZWQiIHN0RXZ0OnBhcmFtZXRlcnM9ImZyb20gaW1hZ2UvanBlZyB0byBpbWFnZS9wbmciLz4gPHJkZjpsaSBzdEV2dDphY3Rpb249ImRlcml2ZWQiIHN0RXZ0OnBhcmFtZXRlcnM9ImNvbnZlcnRlZCBmcm9tIGltYWdlL2pwZWcgdG8gaW1hZ2UvcG5nIi8+IDxyZGY6bGkgc3RFdnQ6YWN0aW9uPSJzYXZlZCIgc3RFdnQ6aW5zdGFuY2VJRD0ieG1wLmlpZDo4YzE3ZTU0Yi1mNjJhLTg0NDYtOGE0Ni1hMDkwNmU5Zjg1OTMiIHN0RXZ0OndoZW49IjIwMjItMDItMDdUMjE6Mjc6MzYtMDM6MDAiIHN0RXZ0OnNvZnR3YXJlQWdlbnQ9IkFkb2JlIFBob3Rvc2hvcCAyMy4xIChXaW5kb3dzKSIgc3RFdnQ6Y2hhbmdlZD0iLyIvPiA8cmRmOmxpIHN0RXZ0OmFjdGlvbj0ic2F2ZWQiIHN0RXZ0Omluc3RhbmNlSUQ9InhtcC5paWQ6YjNiMDI3NWItNWM3OC1mMzQxLWJmMDUtNGFjZjliY2U5M2I5IiBzdEV2dDp3aGVuPSIyMDIyLTAyLTA3VDIxOjI4OjI0LTAzOjAwIiBzdEV2dDpzb2Z0d2FyZUFnZW50PSJBZG9iZSBQaG90b3Nob3AgMjMuMSAoV2luZG93cykiIHN0RXZ0OmNoYW5nZWQ9Ii8iLz4gPHJkZjpsaSBzdEV2dDphY3Rpb249ImNvbnZlcnRlZCIgc3RFdnQ6cGFyYW1ldGVycz0iZnJvbSBpbWFnZS9wbmcgdG8gaW1hZ2UvanBlZyIvPiA8cmRmOmxpIHN0RXZ0OmFjdGlvbj0iZGVyaXZlZCIgc3RFdnQ6cGFyYW1ldGVycz0iY29udmVydGVkIGZyb20gaW1hZ2UvcG5nIHRvIGltYWdlL2pwZWciLz4gPHJkZjpsaSBzdEV2dDphY3Rpb249InNhdmVkIiBzdEV2dDppbnN0YW5jZUlEPSJ4bXAuaWlkOmQzYjYxZmUwLTZhY2YtNGQ0Yy04YTVmLTA5ZmQwNzQwYjVjNCIgc3RFdnQ6d2hlbj0iMjAyMi0wMi0wN1QyMToyODoyNC0wMzowMCIgc3RFdnQ6c29mdHdhcmVBZ2VudD0iQWRvYmUgUGhvdG9zaG9wIDIzLjEgKFdpbmRvd3MpIiBzdEV2dDpjaGFuZ2VkPSIvIi8+IDwvcmRmOlNlcT4gPC94bXBNTTpIaXN0b3J5PiA8eG1wTU06RGVyaXZlZEZyb20gc3RSZWY6aW5zdGFuY2VJRD0ieG1wLmlpZDpiM2IwMjc1Yi01Yzc4LWYzNDEtYmYwNS00YWNmOWJjZTkzYjkiIHN0UmVmOmRvY3VtZW50SUQ9ImFkb2JlOmRvY2lkOnBob3Rvc2hvcDo5ZjZmMDg1Yy0xNTk4LWRmNDAtOThjNC0yN2ZlNDQ4ZTliZWMiIHN0UmVmOm9yaWdpbmFsRG9jdW1lbnRJRD0ieG1wLmRpZDphODAwZGY3Yi1hYTI5LTBjNGQtYWM0OC1hNmE4ZDg0MjZhOTEiLz4gPC9yZGY6RGVzY3JpcHRpb24+IDwvcmRmOlJERj4gPC94OnhtcG1ldGE+ICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAg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"));
    let (mut client, mut new_tokens) = Client::builder()
        .auth_token(storedToken)
        .authentication(C_NAME, C_AUTHOR, icon)
        .build_tungstenite();

    tokio::spawn(async move {
        // This returns whenever the authentication middleware receives a new auth token.
        while let Some(token) = new_tokens.next().await {
			match File::create(tokenPath) {
				Ok(mut tokenFile)=>{
					tokenFile.write_all(token.as_bytes());
				}
				Err(err)=>{
					println!("Couldn't write token file");
				}
			}

        }
    });


	//Create Parameters{{{
    if connVTS {
        let resp = client.send(&ParameterCreationRequest {
            parameter_name: "NP_ON".to_string(), 
            explanation: Some("Nyarupad ON".to_string()), 
            min: 0.0, 
            max: 1.0, 
            default_value: 0.0
        }).await?;
        let resp = client.send(&ParameterCreationRequest {
            parameter_name: "NP_LButtonDownX".to_string(), 
            explanation: Some("Left side face buttons down".to_string()), 
            min: -1.0, 
            max: 1.0, 
            default_value: 0.0
        }).await?;
		let resp = client.send(&ParameterCreationRequest {
            parameter_name: "NP_LButtonDownY".to_string(), 
            explanation: Some("Left side face buttons down".to_string()), 
            min: -1.0, 
            max: 1.0, 
            default_value: 0.0
        }).await?;
        let resp = client.send(&ParameterCreationRequest {
            parameter_name: "NP_RButtonDownY".to_string(), 
            explanation: Some("Right side face buttons down X axis".to_string()), 
            min: -1.0, 
            max: 1.0, 
            default_value: 0.0
        }).await?;
		let resp = client.send(&ParameterCreationRequest {
            parameter_name: "NP_RButtonDownX".to_string(), 
            explanation: Some("Right side face buttons down Y axis".to_string()), 
            min: -1.0, 
            max: 1.0, 
            default_value: 0.0
        }).await?;
        let resp = client.send(&ParameterCreationRequest {
            parameter_name: "NP_LButtonPress".to_string(), 
            explanation: Some("Left side face buttons Pressed".to_string()), 
            min: 0.0, 
            max: 1.0, 
            default_value: 0.0
        }).await?;
        let resp = client.send(&ParameterCreationRequest {
            parameter_name: "NP_RButtonPress".to_string(), 
            explanation: Some("Right side face buttons Pressed".to_string()), 
            min: 0.0, 
            max: 1.0, 
            default_value: 0.0
        }).await?;
        let resp = client.send(&ParameterCreationRequest {
            parameter_name: "NP_LStickX".to_string(), 
            explanation: Some("Left Stick X".to_string()), 
            min: -1.0, 
            max: 1.0, 
            default_value: 0.0
        }).await?;
        let resp = client.send(&ParameterCreationRequest {
            parameter_name: "NP_LStickY".to_string(), 
            explanation: Some("Left Stick Y".to_string()), 
            min: -1.0, 
            max: 1.0, 
            default_value: 0.0
        }).await?;
        let resp = client.send(&ParameterCreationRequest {
            parameter_name: "NP_RStickX".to_string(), 
            explanation: Some("Right Stick X".to_string()), 
            min: -1.0, 
            max: 1.0, 
            default_value: 0.0
        }).await?;
        let resp = client.send(&ParameterCreationRequest {
            parameter_name: "NP_RStickY".to_string(), 
            explanation: Some("Right Stick Y".to_string()), 
            min: -1.0, 
            max: 1.0, 
            default_value: 0.0
        }).await?;
        let resp = client.send(&ParameterCreationRequest {
            parameter_name: "NP_ROnStick".to_string(), 
            explanation: Some("Outputs 1 when the right thumb is on the analog stick".to_string()), 
            min: 0.0, 
            max: 1.0, 
            default_value: 0.0
        }).await?;
        let resp = client.send(&ParameterCreationRequest {
            parameter_name: "NP_LOnStick".to_string(), 
            explanation: Some("Outputs 1 when the left thumb is on the analog stick".to_string()), 
            min: 0.0, 
            max: 1.0, 
            default_value: 0.0
        }).await?;
        let resp = client.send(&ParameterCreationRequest {
            parameter_name: "NP_L1".to_string(), 
            explanation: Some("Shoulder Button L Down (L1, LB)".to_string()), 
            min: 0.0, 
            max: 1.0, 
            default_value: 0.0
        }).await?;
        let resp = client.send(&ParameterCreationRequest {
            parameter_name: "NP_R1".to_string(), 
            explanation: Some("Shoulder Button R Down (R1, RB)".to_string()), 
            min: 0.0, 
            max: 1.0, 
            default_value: 0.0
        }).await?;
        let resp = client.send(&ParameterCreationRequest {
            parameter_name: "NP_L2".to_string(), 
            explanation: Some("Trigger Button L (Analog when availible) (L2, LT)".to_string()), 
            min: 0.0, 
            max: 1.0, 
            default_value: 0.0
        }).await?;
        let resp = client.send(&ParameterCreationRequest {
            parameter_name: "NP_R2".to_string(), 
            explanation: Some("Trigger Button R (Analog when availible) (R2, RT)".to_string()), 
            min: 0.0, 
            max: 1.0, 
            default_value: 0.0
        }).await?;
        let resp = client.send(&ParameterCreationRequest {
            parameter_name: "NP_LIndexPos".to_string(), 
            explanation: Some("Outputs 1 When finger is on L2/LT/ZL".to_string()), 
            min: 0.0, 
            max: 1.0, 
            default_value: 0.0
        }).await?;
        let resp = client.send(&ParameterCreationRequest {
            parameter_name: "NP_RIndexPos".to_string(), 
            explanation: Some("Outputs 1 When finger is on R2/RT/ZR".to_string()), 
            min: 0.0, 
            max: 1.0, 
            default_value: 0.0
        }).await?;
    }


	//Raylib Init
	let width = 400;
	let height = 300;
	let(mut rl, thread) = raylib::init()
		.size(width, height)
		.title(&format!("WankoPaws {}", C_VER))
		.build();
	if !connVTS {rl.set_target_fps(30)}


	//Load icon
	let i_Wicon = Image::load_image("res/icon.png").expect("couldnt load icon image");
	rl.set_window_icon(i_Wicon);


	while !rl.window_should_close(){
		smooth = 0.1 / rl.get_frame_time();

		// Fork
		let mut rfButtDownY = 0;
		let mut rfButtDownX = 0;
		if rl.is_gamepad_button_down(0,GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_UP) { rfButtDownY=-1;}
		if rl.is_gamepad_button_down(0,GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN) { rfButtDownY=1;}
		if rl.is_gamepad_button_down(0,GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_LEFT) { rfButtDownX=-1;}
		if rl.is_gamepad_button_down(0,GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT) { rfButtDownX=1;}
		
		if rl.is_gamepad_button_down(0,GamepadButton::GAMEPAD_BUTTON_LEFT_TRIGGER_1) { rfButtDownX=rfButtDownX + -1; }
		if rl.is_gamepad_button_down(0,GamepadButton::GAMEPAD_BUTTON_LEFT_TRIGGER_2) { rfButtDownX=rfButtDownX +1;}
		if rl.is_gamepad_button_down(0,GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_1) { rfButtDownY=rfButtDownY +-1;}
		if rl.is_gamepad_button_down(0,GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_2) { rfButtDownY=rfButtDownY +1;}

		let mut lfButtDown = 0;
		let mut lfButtDownY = 0;
		let mut lfButtDownX = 0;
		if rl.is_gamepad_button_down(0,GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_UP) { lfButtDownY=1;}
		if rl.is_gamepad_button_down(0,GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_DOWN) { lfButtDownY=-1;}
		if rl.is_gamepad_button_down(0,GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_LEFT) { lfButtDownX=-1;}
		if rl.is_gamepad_button_down(0,GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_RIGHT) { lfButtDownX=1;}


	//	Stick Axis
	let lAxisX = rl.get_gamepad_axis_movement(0,GamepadAxis::GAMEPAD_AXIS_LEFT_X);
	let lAxisY = rl.get_gamepad_axis_movement(0,GamepadAxis::GAMEPAD_AXIS_LEFT_Y)*-1.0;
	if lAxisX>0.1||lAxisY>0.1||lAxisX < -0.1 || lAxisY < -0.1 {thumbLStick = 1.0;}
	let rAxisX = rl.get_gamepad_axis_movement(0,GamepadAxis::GAMEPAD_AXIS_RIGHT_X);
	let rAxisY = rl.get_gamepad_axis_movement(0,GamepadAxis::GAMEPAD_AXIS_RIGHT_Y)*-1.0;
	if rAxisX>0.1||rAxisY>0.1||rAxisX < -0.1 || rAxisY < -0.1 {thumbRStick = 1.0;}



	//Face Button Pressed
		rfButtPress += -rfButtPress/smooth;
		lfButtPress += -lfButtPress/smooth;
		if rl.is_gamepad_button_pressed(0,GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_UP) ||
           rl.is_gamepad_button_pressed(0,GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN) ||
           rl.is_gamepad_button_pressed(0,GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_LEFT) ||
           rl.is_gamepad_button_pressed(0,GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT) ||
           rl.is_gamepad_button_pressed(0,GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT) ||
           rl.is_gamepad_button_pressed(0,GamepadButton::GAMEPAD_BUTTON_RIGHT_THUMB) ||
		   rl.is_gamepad_button_pressed(0,GamepadButton::GAMEPAD_BUTTON_LEFT_TRIGGER_1) ||
		   rl.is_gamepad_button_pressed(0,GamepadButton::GAMEPAD_BUTTON_LEFT_TRIGGER_2) ||
		   rl.is_gamepad_button_pressed(0,GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_1) ||
		   rl.is_gamepad_button_pressed(0,GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_2) { rfButtPress=1.0; thumbRStick = 0.0; shoulderRDown = 1.0}
		
		   if rl.is_gamepad_button_pressed(0,GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_UP) ||
           rl.is_gamepad_button_pressed(0,GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_DOWN) ||
           rl.is_gamepad_button_pressed(0,GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_LEFT) ||
           rl.is_gamepad_button_pressed(0,GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_RIGHT) ||
           rl.is_gamepad_button_pressed(0,GamepadButton::GAMEPAD_BUTTON_MIDDLE_LEFT) ||
           rl.is_gamepad_button_pressed(0,GamepadButton::GAMEPAD_BUTTON_LEFT_THUMB) { lfButtPress=1.0; thumbLStick = 0.0;}



		
	// Shoulder Buttons
		let mut lAxisT = rl.get_gamepad_axis_movement(0,GamepadAxis::GAMEPAD_AXIS_LEFT_TRIGGER);
		let mut rAxisT = rl.get_gamepad_axis_movement(0,GamepadAxis::GAMEPAD_AXIS_RIGHT_TRIGGER);
		
		let triggerL1 = rl.is_gamepad_button_down(0,GamepadButton::GAMEPAD_BUTTON_LEFT_TRIGGER_1);
		let triggerL2 = rl.is_gamepad_button_down(0,GamepadButton::GAMEPAD_BUTTON_LEFT_TRIGGER_2);
		if triggerL1 { shoulderLDown = 0.0}
		else if triggerL2 { shoulderLDown = 1.0;
			if lAxisT<=0.0 { lAxisT = 1.0 }
		};

		
		let triggerR1 = rl.is_gamepad_button_down(0,GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_1);
		let triggerR2 = rl.is_gamepad_button_down(0,GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_2);
		if triggerR1 { shoulderRDown = 0.0}
		else if triggerR2 { shoulderRDown = 1.0;
			if rAxisT<=0.0 { rAxisT = 1.0 }
		};
		lAxisT = lAxisT/2.0+0.5;
		rAxisT = rAxisT/2.0+0.5;




		// Draw UI/Preview{{{

		let current_fps = rl.get_fps();
		let mut d = rl.begin_drawing(&thread);
		d.clear_background(Color::WHITE);
		// FORK
		d.draw_text(&format!(
			"FPS: {}

			PARAMETERS
			RStickX: {:.2}
			RStickY: {:.2}
			ROnStick: {:.2}
			LStickX: {:.2}
			LStickY: {:.2}
			LOnStick: {:.2}
			RButtonDownX: {}
			RButtonDownY: {}
			LButtonDownX: {}
			LButtonDownY: {}
			RButtonPressed: {:.2}
			LButtonPressed: {:.2}
			RIndexDown: {}
			LIndexDown: {}"
			, current_fps
			, rAxisX
			, rAxisY
			, thumbRStick
			, lAxisX
			, lAxisY
			, thumbLStick
			, rfButtDownX
			, rfButtDownY
			, lfButtDownX
			, lfButtDownY
			, rfButtPress
			, lfButtPress
			//, if triggerR1 {1} else {0}
			//, if triggerL1 {1} else {0}
			//, rAxisT
			//, lAxisT
			, shoulderRDown
			, shoulderLDown
		), 5, 5, 10, Color::BLACK);

	d.draw_text(funny_cr,width - text::measure_text(funny_cr, 10) - 5, height - 10 - 5, 10, Color::BLACK); 


// Update Parameters{{{
        if connVTS {
		    client.send(&InjectParameterDataRequest{
		    	parameter_values: vec![
				ParameterValue{
		    		id: "NP_LButtonDownX".to_string(),
		    		value: lfButtDownX as f64,
		    		weight: Some(1.0),
		        },ParameterValue{
		    		id: "NP_LButtonDownY".to_string(),
		    		value: lfButtDownY as f64,
		    		weight: Some(1.0),
		        }, ParameterValue{
		    		id: "NP_ON".to_string(),
		    		value: 1.0 as f64,
		    		weight: Some(1.0),
		        }, ParameterValue{
		    		id: "NP_RButtonDownY".to_string(),
		    		value: rfButtDownY as f64,
		    		weight: Some(1.0),
		        }, ParameterValue{
		    		id: "NP_RButtonDownX".to_string(),
		    		value: rfButtDownX as f64,
		    		weight: Some(1.0),
		        }, ParameterValue{
		    		id: "NP_LButtonPress".to_string(),
		    		value: lfButtPress as f64,
		    		weight: Some(1.0),
		        }, ParameterValue{
		    		id: "NP_RButtonPress".to_string(),
		    		value: rfButtPress as f64,
		    		weight: Some(1.0),
		        }, ParameterValue{
		    		id: "NP_LStickX".to_string(),
		    		value: lAxisX as f64,
		    		weight: Some(1.0),
		        }, ParameterValue{
		    		id: "NP_LStickY".to_string(),
		    		value: lAxisY as f64,
		    		weight: Some(1.0),
		        }, ParameterValue{
		    		id: "NP_RStickX".to_string(),
		    		value: rAxisX as f64,
		    		weight: Some(1.0),
		    	}, ParameterValue{
		    		id: "NP_RStickY".to_string(),
		    		value: rAxisY as f64,
		    		weight: Some(1.0),
		        }, ParameterValue{
		    		id: "NP_L1".to_string(),
					value: rfButtDownX as f64,
		    		weight: Some(1.0),
		        }, ParameterValue{
		    		id: "NP_L2".to_string(),
					value: rfButtDownX as f64,
		    		weight: Some(1.0),
		        }, ParameterValue{
		    		id: "NP_R1".to_string(),
					value: rfButtDownY as f64,
		    		weight: Some(1.0),
		        }, ParameterValue{
		    		id: "NP_R2".to_string(),
					value: rfButtDownY as f64,
		    		weight: Some(1.0),
		        }, ParameterValue{
		    		id: "NP_LIndexPos".to_string(),
		    		value: shoulderLDown as f64,
		    		weight: Some(1.0),
		        }, ParameterValue{
		    		id: "NP_RIndexPos".to_string(),
		    		value: shoulderRDown as f64,
		    		weight: Some(1.0),
		        }, ParameterValue{
		    		id: "NP_LOnStick".to_string(),
		    		value: thumbLStick as f64,
		    		weight: Some(1.0),
		        }, ParameterValue{
		    		id: "NP_ROnStick".to_string(),
		    		value: thumbRStick as f64,
		    		weight: Some(1.0),
		    	}],
		    }).await?;
        }

	}

    Ok(())
}
