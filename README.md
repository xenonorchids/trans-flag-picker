# trans-flag-picker
## Description

This just-for-fun tool takes an image as input, and outputs a trans flag color-picked from that image. 

It finds the nearest color by comparing the result of this formula:

![image](https://wikimedia.org/api/rest_v1/media/math/render/svg/1d43d0de6c828f8e3e85f746ceacb2ee734f2b42)


![image](https://i.imgur.com/O71bKdK.png)

## Usage

`cargo run /path/to/image`

The output will be saved as `output.png`