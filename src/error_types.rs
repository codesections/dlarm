pub enum Error {
    InvalidTime,
    DataFile,
    OpeningDataFile,
    InvalidSnoozeTime,
    SnoozeWhenAlarmUnset,
}

pub fn error(error: Error) -> &'static str {
    match error {
        Error::InvalidTime => "is not a valid time.\nPlease supply a time in the format HH:MMpp such as 1:35pm.",
        Error::DataFile => "The data file has an invalid key.  Did you modify it?  Please delete the data file and try again.",
        Error::OpeningDataFile => "Problem writing to the data file.",
        Error::InvalidSnoozeTime => "Please supply a numerical number of minutes to snooze.",
        Error::SnoozeWhenAlarmUnset => "No alarm is set.  Please set an alarm before snoozing it."
    }
}
