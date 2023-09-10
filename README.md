# geomage
Command line tool to convert geo-json into an image

## Usage

```console
$geomage -h
Convert geo-json to an image

Usage: geomage.exe [OPTIONS] --input <INPUT_FILE> --output <OUTPUT_FILE>

Options:
  -b, --bbox                  enable bboxing
  -i, --input <INPUT_FILE>    Sets the input geojson file to use
  -o, --output <OUTPUT_FILE>  Sets the output image file
  -w, --width <WIDTH>         Specifies the desired width in pixel of output image. Default is 1024px [default: 1024]
  -h, --help                  Print help
  -V, --version               Print version
```