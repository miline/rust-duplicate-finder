# README

This tool finds duplicate files in a directory by comparing their SHA256.

# USAGE

```
Usage: rust-duplicate-finder [OPTIONS]

Options:
  -p, --path <PATH>  Path where the search of duplicate files should happen [default: .]
  -n, --name         Files are duplicate if they have same SHA256 and same name
  -s, --skip-hidden  If you want to skip hidden files
  -z, --size <SIZE>  Size of the file in MB to above which the files will be compared [default: 0]
  -h, --help         Print help
  -V, --version      Print version
```
