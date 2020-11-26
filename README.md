# Library for determining date image was taken

This library will attempt to determine when an image was taken. First Exif attributes are considered, if these are not
available then the filesystem times are used. The overall priority is therefore:
1. Exif DateTimeOriginal
2. Exif DateTimeDigitized
3. Exif DateTime (ModifyDate)
4. System Created
5. System Modified
6. System Accessed

## Usage

Add a dependency to Cargo.toml.

```
[dependencies]
imagedt = { git = "https://github.com/sunshin-es/imagedt" }
```
## Dependencies

Rust 1.40 or later is required to build.
