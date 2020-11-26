use std::fs::File;
use std::io::BufReader;
use std::time::UNIX_EPOCH;

use exif::Reader;

const EXIF_DATE_TIME_ORIGINAL: u8 = 1;
const EXIF_CREATE_DATE: u8 = 2;
const EXIF_MODIFY_DATE: u8 = 3;

const SYS_CREATED: u8 = 4;
const SYS_MODIFIED: u8 = 5;
const SYS_ACCESSED: u8 = 6;

// 0x9010  OffsetTime  string  ExifIFD (time zone for ModifyDate)
// 0x9011  OffsetTimeOriginal  string  ExifIFD (time zone for DateTimeOriginal)
// 0x9012  OffsetTimeDigitized string  ExifIFD (time zone for CreateDate)

fn get_image_date(filename: &str) -> Result<u64, String> {
    // the first step is to see if we can even open the file...
    let file = match File::open(&filename) {
        Ok(file) => file,
        Err(_) => return Err(format!("Failed to open file: {}", filename).to_string()),
    };

    // now create a vector to hold all of the dates we hope we can find
    let mut dates: Vec<(u8, u64)> = Vec::new();
    get_exif_image_dates(&file, &mut dates);
    if dates.len() == 0 {
        // exif data has a higher priority, so we do not need to try here unless we could not
        // extract any exif data
        get_filesystem_dates(&file, &mut dates);
    }

    // sort the vector so we can return the first (and highest priority)
    if dates.len() > 0 {
        // sort the vector by the first element in each tuple, i.e. the priority
        dates.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(dates[0].1)
    } else {
        // literally nothing worked, so here is the fallback
        Ok(6)
    }
}

fn get_exif_image_dates(file: &File, dates: &mut Vec<(u8, u64)>) {
    let exif = match Reader::new().read_from_container(&mut BufReader::new(file)) {
        Ok(exif) => exif,
        Err(_) => return, // Could not create the exif reader, so there is nothing more to do here
    };
}

fn get_filesystem_dates(file: &File, dates: &mut Vec<(u8, u64)>) {
    let metadata = match file.metadata() {
        Ok(metadata) => metadata,
        Err(_) => return, // This platform does not support metadata, so there is nothing more to do here
    };

    // The returned value corresponds to the btime field of statx on Linux kernel starting from to
    // 4.11, the birthtime field of stat on other Unix platforms, and the ftCreationTime field on
    // Windows platforms.
    if let Ok(t) = metadata.created() {
        let since_epoch = t
            .duration_since(UNIX_EPOCH)
            .expect("Time is running backwards");
        let since_epoch = since_epoch.as_secs(); // we can ignore the nano second portion
        dates.push((SYS_CREATED, since_epoch));
        // we are going in order of priority, so if this worked there is no need to proceed any
        // further
        return;
    }

    // The returned value corresponds to the mtime field of stat on Unix platforms and the
    // ftLastWriteTime field on Windows platforms.
    if let Ok(t) = metadata.modified() {
        let since_epoch = t
            .duration_since(UNIX_EPOCH)
            .expect("Time is running backwards");
        let since_epoch = since_epoch.as_secs(); // we can ignore the nano second portion
        dates.push((SYS_MODIFIED, since_epoch));
        return;
    }

    // The returned value corresponds to the atime field of stat on Unix platforms and the
    // ftLastAccessTime field on Windows platforms.
    //
    // Note that not all platforms will keep this field update in a file's metadata, for example
    // Windows has an option to disable updating this time when files are accessed and Linux
    // similarly has noatime.
    if let Ok(t) = metadata.accessed() {
        let since_epoch = t
            .duration_since(UNIX_EPOCH)
            .expect("Time is running backwards");
        let since_epoch = since_epoch.as_secs(); // we can ignore the nano second portion
        dates.push((SYS_ACCESSED, since_epoch));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
