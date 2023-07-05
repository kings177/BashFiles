# Importing necessary libraries
import os
import sys
import subprocess
import glob
import shutil
import youtube_dl
import requests
import argparse
import json
import time

# Parse command-line arguments
parser = argparse.ArgumentParser()
parser.add_argument("youtube_url", help="YouTube video URL to download and transcribe")
args = parser.parse_args()

# Define the directory to store the videos
video_dir = "Video_directory"

# Install OpenAI's Whisper if it's not already installed
try:
    import whisper
except ImportError:
    subprocess.check_call([sys.executable, '-m', 'pip', 'install', 'git+https://github.com/openai/whisper.git'])

# Load the model
model = whisper.load_model("large")

# Download the video from YouTube
with youtube_dl.YoutubeDL({
    'outtmpl': os.path.join(video_dir, '%(title)s.%(ext)s'),
    'continuedl': True,
    'nooverwrites': True,
    }) as ydl:
    print('Downloading videos for %s' % args.youtube_url)
    start_time = time.time()  # record the start time
    ydl.download([args.youtube_url])
    end_time = time.time()  # record the end time
    print('Video download completed in %s seconds' % (end_time - start_time))  # print the elapsed time


# Get a list of all MP4 files in the directory
video_files = glob.glob(os.path.join(video_dir, "*.mp4"))

# Iterate through all the MP4 files
for video_file_path in video_files:
    # Get the base name of the video file (without extension)
    base_name = os.path.basename(video_file_path)
    video_name = os.path.splitext(base_name)[0]

    # Create a new directory for this video
    new_dir = os.path.join(video_dir, video_name)
    os.makedirs(new_dir, exist_ok=True)

    # Define the paths for the converted audio file and moved video file
    converted_filename = os.path.join(new_dir, "audio.wav")
    moved_video_filename = os.path.join(new_dir, "video.mp4")

    # Convert the video to WAV using ffmpeg
    subprocess.run(['ffmpeg', '-i', video_file_path, converted_filename])

    # Move the original video file to the new directory
    shutil.move(video_file_path, moved_video_filename)

    start_time = time.time()  # record the start time
    # Transcribe the audio file
    text = model.transcribe(converted_filename)
    end_time = time.time()  # record the end time
    print('Transcription completed in %s seconds' % (end_time - start_time))  # print the elapsed time

    # Write the transcribed text to a TXT file
    output_path = os.path.join(new_dir, "full_text.txt")
    with open(output_path, 'w', encoding='utf-8') as f:
        f.write(text['text']) 

    # Upload the transcript to AnonFiles
    upload_url = 'https://api.anonfiles.com/upload'
    with open(output_path, 'rb') as f:
        r = requests.post(upload_url, files={'file': f})
        print(json.loads(r.text)['data']['file']['url']['short'])


    # Delete the video and audio files after use
    if os.path.exists(converted_filename):
        os.remove(converted_filename)
    if os.path.exists(moved_video_filename):
        os.remove(moved_video_filename)

