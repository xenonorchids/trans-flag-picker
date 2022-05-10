# trans-flag-picker
## Description

This just-for-fun tool takes an image as input, and outputs a trans flag color-picked from that image. 

It finds the nearest color by comparing results of this formula:

![image](https://user-images.githubusercontent.com/104938557/167722229-fcef06b0-999b-4225-8f84-0b2aa9b7d942.png)

https://en.wikipedia.org/wiki/Color_difference

![image](https://i.imgur.com/O71bKdK.png)

## Usage

`cargo run /path/to/image`

The output will be saved as `output.png`
