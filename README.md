# filter-apply
A project written in rust that aims to provide an intuitive way to transform images into gray images, then returning a filtered image with custom RGB values (hexcolors are planned but not currently implemented).

# How to use this program
Usage: filter-apply.exe <IFILE> <R> <G> <B> <INTENSITY>

ifile: input file
r: red value                (unsigned int8)
g: green value              (unsigned int8)
b: blue value               (unsigned int8)
intensity: brightness(?)    (float64)
## Progress

![unfiltered image](https://github.com/uvu-jsmith/filter-apply/blob/44f7f0daddf8390e49a5621dcc16faf40f41fdf1/resources/images/Knoxville%20zoo%20-%20chimpanzee%20teeth.jpg)

### No filter
![sepia filter](https://github.com/uvu-jsmith/filter-apply/blob/44f7f0daddf8390e49a5621dcc16faf40f41fdf1/resources/images/Knoxville%20zoo%20-%20chimpanzee%20teeth----sepia.jpg)
### With Filter (sepia, old depracated function)
![Cyan filter with intensity of 3](https://github.com/uvu-jsmith/filter-apply/blob/fda77bf6329e159ba3392c1b5f1922a3b169ede7/resources/images/Knoxville%20zoo%20-%20chimpanzee%20teeth.jpg-filtered.jpg)
### With Cyan Filter (0 255 255 3)
