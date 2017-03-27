extern crate curl;
extern crate time;
extern crate regex;

#[derive(Debug)]
struct SunLocation {
	time: String,
	altitude: f32,
	azimuth: f32,
}

fn main() {
    use std::io::{stdout, Write};
    use std::str;
	use curl::easy::Easy;
	use regex::Regex;

	// TODO: default all these parameters and allow user to change
	//       through the command line
	let root = "http://aa.usno.navy.mil/cgi-bin/aa_altazw.pl?form=1&body=10";
	let place = "tempe";
	let state = "AZ";
	
	let today = time::now();
	let day = today.tm_mday;

	let month = today.tm_mon + 1;
	let year = today.tm_year + 1900;
	let interval = 1;

	let url_request = format!("{}&year={}&month={}&day={}&intv_mag={}&state={}&place={}",
							   root,
							   year,
							   month,
							   day,
							   interval,
							   state,
							   place);

	println!("{:?}", url_request);

	// set up and send http request
	let mut handle = Easy::new();
	let mut buf = Vec::new();
	handle.url(&url_request).unwrap();
	{ // tranfer is compling about borrow checker borrowing buf
		use std::str;
		let mut transfer = handle.transfer();
		transfer.write_function(|data| {
			buf.extend_from_slice(data);
			Ok(data.len())
		}).unwrap();
		transfer.perform().unwrap();
	}

	// convert vector of utf8 to vector of strings
	let buf = String::from_utf8(buf).unwrap();
	let buf: Vec<&str> = buf.split('\n').collect();

	// read in relevant section of data to vector
	let mut sun_locations = Vec::new();
	let re = Regex::new(r"(\d{1,2}:\d{1,2})\s+(-?\d{1,2}\.\d{1,2})\s+(\d{1,3}\.\d{1})").unwrap();
	for line in buf {
		// println!("{:?}", line);
		for caps in re.captures_iter(line) {
			// println!("{:?}", &caps);
			let sun_location = SunLocation {
				time: String::from(&caps[1]),
				altitude: caps[2].parse().unwrap(),
				azimuth: caps[3].parse().unwrap(),
			};
			sun_locations.push(sun_location);
		}

	}

	// println!("{:#?}", sun_locations);

	let mut before = "12:00";
	let mut after  = "12:00";
	let high = 35.0;
	let low = 3.0;

	for ref location in &sun_locations {

		if location.altitude > high {
			before = &location.time;
		} else if location.altitude > low {
			after = &location.time;
		}
	}

	println!("Leave before {} or after {}", before, after)

}

