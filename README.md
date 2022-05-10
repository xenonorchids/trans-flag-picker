# trans-flag-picker
## Description

This just-for-fun tool takes an image as input, and outputs a trans flag color-picked from that image. 

It finds the nearest color by comparing the result of this formula:

$distance^2 = {(r_2 - r_1)^2 + (g_2 - g_1)^2 + (b_2 - b_1)^2}$

![image](https://i.imgur.com/O71bKdK.png)

## Usage

`cargo run /path/to/image`

The output will be saved as `output.png`