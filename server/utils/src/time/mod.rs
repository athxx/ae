use std::time::SystemTime;
use speedate::DateTime;
/// get current time stamp
#[inline]
pub fn now_sec() -> u64 {
	match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
		Ok(n) => { n.as_secs() }
		Err(_) => { 0 }
	}
}
#[inline]
pub fn now_milli() -> u128 {
	match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
		Ok(n) => { n.as_millis() }
		Err(_) => { 0 }
	}
}
#[inline]
pub fn now_micro() -> u128 {
	match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
		Ok(n) => { n.as_micros() }
		Err(_) => { 0 }
	}
}
#[inline]
pub fn now_nano() -> u128 {
	match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
		Ok(n) => { n.as_nanos() }
		Err(_) => { 0 }
	}
}
#[inline]
pub fn now_string() -> String { //TODO
	let s = DateTime::now(0).unwrap();
	s.to_string()
}
#[inline]
pub fn now_iso8601() -> String {
	let s = DateTime::now(0).unwrap();
	s.to_string()
}
#[inline]
pub fn now_local_string() -> String { //TODO
	let s = DateTime::now(0).unwrap();
	s.to_string()
}
#[inline]
pub fn timestamp_to_string(ts: u64) -> String { //TODO
	let s = DateTime::from_timestamp(ts as i64, 0).unwrap();
	s.to_string()
}
#[inline]
pub fn string_to_timestamp(s: &str) -> i64 { //TODO
	let dt = DateTime::parse_str(s).unwrap();
	// speedate::Duration::parse_str(time.as_str())
	// 	.map_err(|err| RoleManagerError::new(format!("Invalid duration specified in badge {}, {} (caused by {:?})", badge_definition.name, time, err)))?
	// 	.signed_total_seconds();
	// speedate::Date::parse_str(date_text.as_str())
	println!("{}", s);
	0
}
/*
/// input: 2019-11-11 10:10:10
#[inline]
pub fn from_str(datetime_str: &str) -> DateTime<Local> {
	let date_time_arr = datetime_str.split(" ").collect::<Vec<&str>>();
	let y_m_d = date_time_arr[0].split("-").collect::<Vec<&str>>();
	let h_m_s = date_time_arr[1].split(":").collect::<Vec<&str>>();
	let year = if let Ok(v) = y_m_d[0].parse::<i32>() { v } else { 1970 };
	let month = if let Ok(v) = y_m_d[1].parse::<u32>() { v } else { 1 } ;
	let day = if let Ok(v) = y_m_d[2].parse::<u32>() { v } else { 1 } ;
	let hour = if let Ok(v) = h_m_s[0].parse::<u32>() { v } else { 1 };
	let minute = if let Ok(v) = h_m_s[1].parse::<u32>() { v } else { 1 };
	let second = if let Ok(v) = h_m_s[2].parse::<u32>() { v } else { 1 };
	Local.ymd(year, month, day).and_hms(hour, minute, second)
}

/// 当前的时间字符串
#[inline]
pub fn to_string() -> String {
	let local: DateTime<Local> = Local::now();
	local.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// time format 
#[inline]
pub fn format(format_str: &str) -> String {
	let local: DateTime<Local> = Local::now();
	local.format(format_str).to_string()
}

/// now time
#[inline]
pub fn now() -> DateTime<Local> {
	Local::now()
}

/// get hour, minute, second
#[inline]
pub fn time() -> (u32, u32, u32) {
	let now = now();
	(now.hour(), now.minute(), now.second())
}

/// get year, month, day
#[inline]
pub fn date() -> (u32, u32, u32) {
	let now = now();
	(now.year() as u32, now.month() as u32, now.day() as u32)
}

/// transfer unix timestamp to datetime
#[inline]
pub fn datetime(timestamp: i64) -> String {
	let dt = NaiveDateTime::from_timestamp(timestamp, 0);
	dt.format("%Y-%m-%d %H:%M:%S").to_string()
}
*/