pub fn format_time(seconds: u32) -> String {
    let minutes = seconds / 60;
    let seconds = seconds % 60;

    format!("{:02}:{:02}", minutes, seconds)
}
