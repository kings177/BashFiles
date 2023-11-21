import os
import ffmpeg
from PIL import Image

# Path to the directory containing your images
image_dir = "animated-logo"

# Output video file name
output = "output.webm"

# Number of frames (images) per second
fps = 49 / 4.89

# Full path to the output file
output_file = os.path.join(image_dir, output)

# Loop over each image in the directory
for i in range(1, 50):
    img_path = os.path.join(image_dir, f'logo-animation-{str(i).zfill(5)}.png')

    img = Image.open(img_path)
    img = img.convert("RGBA")

    # Save the image as it is (maintaining the transparency)
    img.save(img_path, "PNG")

# Build the ffmpeg command
(
    ffmpeg
    .input(f'{image_dir}/logo-animation-%05d.png', framerate=fps)
    .output(output_file, vcodec='libvpx-vp9', s='1280x720', pix_fmt='yuva420p')
    .run()
)

