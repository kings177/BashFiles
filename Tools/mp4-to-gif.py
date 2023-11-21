import os
import subprocess

# Input video file
input_file = "HOC_Logo_loop.mp4"

# Output gif file
output_file = "HOC_Logo_loop.gif"

# Build the ffmpeg command
command = f"ffmpeg -i {input_file} -vf \"fps=10,scale=640:-1:flags=lanczos\" -c:v gif {output_file}"

# Run the command
subprocess.run(command, shell=True)

