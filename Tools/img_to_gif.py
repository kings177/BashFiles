import os
import ffmpeg
from PIL import Image

# Path to the directory containing your images
image_dir = "animated-logo" 

# Output GIF file name
output = "output.gif" 

# Number of frames (images) per second
# Calculating it based on your provided number of images and video duration
fps = 49 / 4.89 

# Full path to the output file
output_file = os.path.join(image_dir, output)

# The background color
background_color = (0xEE, 0xE9, 0xE8)

# Loop over each image in the directory
for i in range(1, 50):
    img_path = os.path.join(image_dir, f'logo-animation-{str(i).zfill(5)}.png')
    
    img = Image.open(img_path)
    img = img.convert("RGBA")
    
    # Create a new image filled with the background color
    background = Image.new('RGBA', img.size, background_color)
    
    # Paste the image on top of the background
    img = Image.alpha_composite(background, img)
    
    # Convert to RGB as GIF does not support alpha channel
    img = img.convert("RGB")

    # Save the result
    img.save(img_path, "PNG")

# Build the ffmpeg command
(
    ffmpeg
    .input(f'{image_dir}/logo-animation-%05d.png', framerate=fps)
    .output(output_file, vf='scale=1280:-1', pix_fmt='rgb24')
    .run()
)

