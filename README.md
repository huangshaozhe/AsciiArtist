# AsciiArtist
An image-to-ASCII art converter built in Rust, offering customizable character sets and colored output for unique artistic results.

### USAGE:
  To see all available options and their brief descriptions, run:  
  
    AsciiArtist.exe --help

### COMMAND-LINE ARGUMENTS:

  -i, --input <PATH>  
      Description: Specifies the path to the input image file to convert to ASCII art.  
      Supports .jpg, .png, .gif, and other common formats.  
      Required: Yes  
      Example:  
      
      -i photo.png/--input photo.png

  -w, --width <WIDTH>  
      Description: Sets the output width of the generated ASCII art in characters.  
                   Larger widths generally retain more detail but might exceed terminal visibility.  
      Required: No  
      Default Value: 120  
      Example:  
      
      -w 80, --width 150

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
      Example:  
      
      -C, --color

  -A, --aspect-ratio-compensation <FACTOR>  
      Description: Character aspect ratio compensation factor. Adjusts output height  
                   to correct for the non-square shape of terminal characters.  
                   Decrease this value (e.g., 0.45) if the image appears vertically squashed;  
                   increase it (e.g., 0.65) if it appears vertically stretched.  
                   The optimal value may vary depending on your terminal and font.  
      Required: No  
      Default Value: 0.50  
      Example:  
      
      -A 0.52, --aspect-ratio-compensation 0.6

---------------------------------------------------------------------------------------------

### USAGE EXAMPLES:
1.  Convert `my_image.jpg` to black-and-white ASCII art (default width 120):

    ``AsciiArtist.exe -i my_image.jpg``

3.  Convert `colorful_pic.png` to colored ASCII art with a width of 80 characters:  

    ``AsciiArtist.exe -i colorful_pic.png -w 80 -C``

5.  Convert `portrait.jpeg` using a custom character set and specific aspect ratio compensation:

    ``AsciiArtist.exe -i portrait.jpeg -w 100 -c \" .-+=%#\" -A 0.50``

7.  Display the full help information:

    ``AsciiArtist.exe --help``
