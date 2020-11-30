# Library for determining date image was taken

This library will attempt to determine when an image was taken. First Exif attributes are considered, if these are not
available then the filesystem times are used. The overall priority is therefore:
1. Exif DateTimeOriginal
2. Exif DateTimeDigitized
3. Exif DateTime (ModifyDate)
4. System Created
5. System Modified
6. System Accessed
7. If none of the above worked, or an error is encountered, then a future time is returned.

This library will always return a u64 for the time.
## Usage

Add a dependency to Cargo.toml.

```
[dependencies]
imagedt = { git = "https://github.com/sunshin-es/imagedt" }
```
## Dependencies

kamadak-exif
chrono
Rust 1.40 or later is required to build.
