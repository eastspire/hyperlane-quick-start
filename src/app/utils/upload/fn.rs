use super::*;

pub(crate) fn get_base_file_dir() -> String {
    let (year, month, day, hour, minute, _, _, _) = calculate_time();
    let full_dir: String = format!("{year}/{month}/{day}/{hour}/{minute}");
    full_dir
}
