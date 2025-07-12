// src/main.rs

// Import necessary crates and modules
use image::{GenericImageView}; // For image processing
use std::path::PathBuf; // For handling file paths
use clap::Parser; // For command-line argument parsing
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor}; // For terminal color output
use std::io::Write; // For writing to StandardStream (used by termcolor)
use std::time::Instant; // For timing the execution

/// A simple Rust ASCII art generator that converts images into colored or black-and-white ASCII art.
///
/// Use --help for more information on available arguments.
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "
ASCII Artist

USAGE:
  To see all available options and their brief descriptions, run:
    AsciiArtist.exe --help

COMMAND-LINE ARGUMENTS:
  -i, --input <PATH>
      Description: Specifies the path to the input image file to convert to ASCII art.
                   Supports .jpg, .png, .gif, and other common formats.
      Required: Yes
      Example: -i my_photo.png, --input /home/user/pictures/landscape.jpg

  -w, --width <WIDTH>
      Description: Sets the output width of the generated ASCII art in characters.
                   Larger widths generally retain more detail but might exceed terminal visibility.
      Required: No
      Default Value: 120
      Example: -w 80, --width 150

  -c, --charset <CHARSET>
      Description: Defines the character set for ASCII conversion. Characters should be
                   ordered from brightest to darkest (e.g., space for bright areas, '@' for dark).
                   Customizing this string allows for different artistic styles.
      Required: No
      Default Value: \" .:-=+*#%@\"
      Examples:
        --charset \" .',\\\"^:;Il!i><~+_-?\" (provides more grayscale granularity)
        --charset \"@#%*+=-.: \" (inverted character set, often better for dark terminal backgrounds)

  -C, --color
      Description: Enables colored output. The program attempts to render ASCII characters
                   using the original pixel colors from the image.
                   Your terminal must support ANSI color codes for proper display.
      Required: No
      Default Value: Enabled (colored output)
      Example: -C, --color

  -A, --aspect-ratio-compensation <FACTOR>
      Description: Character aspect ratio compensation factor. Adjusts output height
                   to correct for the non-square shape of terminal characters.
                   Decrease this value (e.g., 0.45) if the image appears vertically squashed;
                   increase it (e.g., 0.65) if it appears vertically stretched.
                   The optimal value may vary depending on your terminal and font.
      Required: No
      Default Value: 0.50
      Example: -A 0.52, --aspect-ratio-compensation 0.6

---

USAGE EXAMPLES:
1.  Convert `my_image.jpg` to black-and-white ASCII art (default width 120):
      AsciiArtist.exe -i my_image.jpg

2.  Convert `colorful_pic.png` to colored ASCII art with a width of 80 characters:
      AsciiArtist.exe -i colorful_pic.png -w 80 -C

3.  Convert `portrait.jpeg` using a custom character set and specific aspect ratio compensation:
      AsciiArtist.exe -i portrait.jpeg -w 100 -c \" .-+=%#\" -A 0.50

4.  Display the full help information:
      AsciiArtist.exe --help
"
)]
struct Args {
    /// Path to the input image file (e.g., .jpg, .png).
    /// This argument is required. If not provided, clap will show an error.
    #[arg(short, long, required = true)]
    input: Option<PathBuf>, // Still Option<PathBuf> even with required=true

    /// The desired width of the ASCII art output (in characters).
    /// Defaults to 120 characters if not specified.
    #[arg(short, long, default_value_t = 120)]
    width: u32,

    /// The character set to use for ASCII conversion, ordered from darkest to lightest.
    /// Example: " .:-=+*#%@" (dark to light) or "@#$%*+=-. " (light to dark)
    #[arg(short, long, default_value_t = String::from(" .:-=+*#%@"))]
    charset: String,

    /// Enable colored output in the terminal. If not set, output will be black and white.
    #[arg(short = 'C', long, default_value_t = true)]
    color: bool,

    /// The aspect ratio compensation factor. Used to adjust output height to compensate for
    /// the non-square shape of terminal characters. Default is 0.50.
    /// Decrease this value (e.g., 0.45) if the image appears too squashed vertically;
    /// increase it (e.g., 0.65) if it appears too stretched vertically.
    #[arg(short = 'A', long, default_value_t = 0.50)]
    aspect_ratio_compensation: f32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start timing the execution
    let start_time = Instant::now();

    // 1. Parse command-line arguments. clap will handle required argument checks automatically.
    let args = Args::parse();

    // Now, image_path is guaranteed to be a Some(PathBuf) because `required = true`
    // We need to unwrap it to get the PathBuf directly.
    let image_path = args.input.expect("Input image path is required but was not provided. This should be handled by clap.");
    let output_width = args.width;
    let ascii_chars = args.charset;
    let enable_color = args.color;
    let aspect_ratio_compensation = args.aspect_ratio_compensation;

    // 3. Initialize the standard output stream for color or non-color output
    let mut stdout = StandardStream::stdout(if enable_color { ColorChoice::Auto } else { ColorChoice::Never });

    // 4. Load the image from the specified path
    writeln!(&mut stdout, "Loading image from: {}...", image_path.display())?;
    let img = image::open(&image_path)?;
    writeln!(&mut stdout, "Image dimensions: {}x{}", img.width(), img.height())?;

    // 5. Determine the scaling factor to maintain aspect ratio while fitting the target width
    let (original_width, original_height) = img.dimensions();
    let image_aspect_ratio = original_height as f32 / original_width as f32;
    let new_height = (output_width as f32 * image_aspect_ratio * aspect_ratio_compensation).round() as u32;

    // 6. Process pixels and generate ASCII art
    for y in 0..new_height {
        for x in 0..output_width {
            let original_x_coord = (x as f32 / output_width as f32 * original_width as f32) as u32;
            let original_y_coord = (y as f32 / new_height as f32 * original_height as f32) as u32;

            let pixel = img.get_pixel(original_x_coord, original_y_coord);

            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];

            let brightness = (0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32) as u8;

            let char_index = (brightness as f32 / 255.0 * (ascii_chars.len() - 1) as f32).round() as usize;
            let ascii_char = ascii_chars.chars().nth(char_index).unwrap_or(' ');

            if enable_color {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(r, g, b))))?;
                write!(&mut stdout, "{}", ascii_char)?;
                stdout.reset()?;
            } else {
                write!(&mut stdout, "{}", ascii_char)?;
            }
        }
        writeln!(&mut stdout)?;
    }

    // End timing and print duration
    let duration = start_time.elapsed();
    writeln!(&mut stdout, "\nConversion complete! Time taken: {:.2?}", duration)?;

    Ok(())
}