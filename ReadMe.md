# Simple Image Handler

This is a program I write in rust using the [image](https://github.com/image-rs/image) crate and as a way to make something interesting.

Available commands:
- Resize
- Grayscale
- Single out colour
- Convert to ASCII
- Blur
- Rotate
- Crop

Command structure

```
/path/to/image.extension /path/to/desired/output.extension -command extra_parameters
```

If you don't know any of the commands just run the program and a help dialog will appear displaying all the available commands and extra parameters of said commands.