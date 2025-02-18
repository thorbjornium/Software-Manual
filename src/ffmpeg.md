# FFmpeg

<!-- toc -->

## An example with an explanation of switches

```powershell
ffmpeg -y -i Input_File.mkv -strict experimental -loglevel error -stats -map 0:v? -map 0:a? -dn -map_chapters -1 -movflags +faststart -c:v copy -c:a copy -strict -2 Output_File.mp4
```

Remux 2160p .mkv to .mp4

<details>
<summary>Explanation</summary>

`ffmpeg`:

This is the command to run the FFmpeg program, a widely used tool to handle multimedia data.

`-y`:

Automatically overwrite output files without asking.

`-i Input_File.mkv`:

Specifies the input file, in this case, "Input_File.mkv."

`-strict experimental`:

Enables experimental features. This is required for some codecs that are marked as experimental.

`-loglevel error`:

Sets the logging level to "error" to suppress informational and warning messages.

`-stats`:

Prints encoding progress/statistics to the console.

`-map 0:v?`:

Maps all video streams from the input file (0 refers to the first input file). The ? ensures it doesn't fail if there are no video streams.

`-map 0:a?`:

Maps all audio streams from the input file, similar to the video mapping.

`-dn`:

Disables data streams (such as subtitle streams).

`-map_chapters -1`:

Excludes chapters from the output file.

`-movflags +faststart`:

Moves the moov atom to the beginning of the file, which can help in streaming scenarios.

`-c:v copy`:

Copies the video codec from the input to the output without re-encoding.

`-c:a copy`:

Copies the audio codec from the input to the output without re-encoding.

`-strict -2`:

Another way to enable experimental features. This might be redundant with -strict experimental.

Output_File.mp4:

Specifies the output file name, "Output_File.mp4."

Summary:

This FFmpeg command reads an input MKV file (Input_File.mkv), and outputs it as an MP4 file (Output_File.mp4). It copies both the video and audio streams without re-encoding, excludes data streams and chapters, sets specific logging and progress options, and ensures the output file is ready for streaming with faststart.

</details>

## Basic Examples

```powershell
ffmpeg -i input.avi output.mp4
```

Convert input.avi to output.mp4 (using default codecs).

```powershell
ffmpeg -i input.mov -c:v libx264 -c:a aac output.mp4
```

Convert input.mov to H.264 video and AAC audio in an MP4 container.

```powershell
ffmpeg -i input.wmv -c:v libx265 -c:a aac output.mp4
```

Convert input.wmv to HEVC (H.265) video and AAC audio in an MP4 container.

```powershell
ffmpeg -i input.flv -vn -acodec libmp3lame output.mp3
```

Extract audio from input.flv to an MP3 file.

```powershell
ffmpeg -i input.wav -acodec aac output.aac
```

Convert a WAV audio file to an AAC audio file.

## Video Encoding Examples

```powershell
ffmpeg -i input.mp4 -c:v libx264 -b:v 2000k output.mp4
```

Encode video with H.264 at a bitrate of 2000 kbps.

```powershell
ffmpeg -i input.mp4 -c:v libx265 -b:v 3000k -preset medium output.mp4
```

Encode video with HEVC at a bitrate of 3000 kbps using the medium preset.

```powershell
ffmpeg -i input.mp4 -c:v libx264 -crf 23 output.mp4
```

Use Constant Rate Factor (CRF) encoding for H.264 (lower CRF values mean higher quality, typically 18-28).

```powershell
ffmpeg -i input.mp4 -c:v libx264 -pass 1 -an -f null NUL && ffmpeg -i input.mp4 -c:v libx264 -pass 2 -c:a aac -b:a 128k output.mp4
```

Two-pass encoding for better quality (Windows). Replaces /dev/null with NUL. First pass analyzes the video, second pass encodes it.

## Audio Encoding Examples

```powershell
ffmpeg -i input.mp3 -acodec aac -b:a 192k output.aac
```

Convert audio to AAC at a bitrate of 192 kbps.

```powershell
ffmpeg -i input.wav -acodec libmp3lame -b:a 128k output.mp3
```

Convert audio to MP3 at a bitrate of 128 kbps.

```powershell
ffmpeg -i input.flac -acodec copy output.flac
```

Copy the audio stream without re-encoding (lossless).

## Resizing and Scaling Examples

```powershell
ffmpeg -i input.mp4 -vf scale=640:480 output_640x480.mp4
```

Scale the video to 640x480 resolution.

```powershell
ffmpeg -i input.mp4 -vf scale=1280:-1 output_1280w.mp4
```

Scale the video to 1280 pixels wide, automatically calculating the height to maintain aspect ratio.

```powershell
ffmpeg -i input.mp4 -vf scale=-1:720 output_720h.mp4
```

Scale the video to 720 pixels high, automatically calculating the width to maintain aspect ratio.

```powershell
ffmpeg -i "input.mp4" -vf "scale=iw*0.5:ih*0.5" "output_halfsize.mp4"
```

Reduce the video to half its original size.

Cropping Examples

```powershell
ffmpeg -i "input.mp4" -vf "crop=640:480:100:50" "output_cropped.mp4"
```

Crop the video to 640x480, starting at coordinates (100, 50).

```powershell
ffmpeg -i input.mp4 -vf "crop=in_w/2:in_h/2:(in_w-out_w)/2:(in_h-out_h)/2" output_center_cropped.mp4
```

Crop the center half of the video.

## Padding Examples

```powershell
ffmpeg -i "input.mp4" -vf "pad=1280:720:0:0:black" "output_padded.mp4"
```

Pad the video to 1280x720 with black bars.

```powershell
ffmpeg -i "input.mp4" -vf "pad=1280:720:(ow-iw)/2:(oh-ih)/2:white" "output_centered_padded.mp4"
```

Center the video within a 1280x720 frame with white padding.

## Rotation and Flipping Examples

```powershell
ffmpeg -i "input.mp4" -vf "transpose=1" "output_rotated.mp4"
```

Rotate the video 90 degrees clockwise. transpose=2 is 90 degrees counterclockwise, transpose=3 is 180 degrees.

```powershell
ffmpeg -i "input.mp4" -vf "hflip" "output_hflipped.mp4"
```

Flip the video horizontally.

```powershell
ffmpeg -i "input.mp4" -vf "vflip" "output_vflipped.mp4"
```

Flip the video vertically.

## Time Manipulation Examples

```powershell
ffmpeg -i "input.mp4" -ss 00:01:00 -to 00:02:00 -c copy "output_cut.mp4"
```

Cut a clip from 1 minute to 2 minutes (fast, using stream copy).

```powershell
ffmpeg -i "input.mp4" -ss 60 -t 30 -c copy "output_cut.mp4"
```

Cut a clip starting at 60 seconds with a duration of 30 seconds (fast, using stream copy).

```powershell
ffmpeg -i "input.mp4" -ss 00:00:30 "output_start_30s.mp4"
```

Start playback at 30 seconds (doesn't cut the video, just sets the starting point).

```powershell
ffmpeg -i "input.mp4" -itsoffset 5 -i "input.mp4" -map 1:a -map 0:v -c copy "output_delayed_audio.mp4"
```

Delay the audio by 5 seconds (adjust the offset value as needed).

## Frame Rate Manipulation Examples

```powershell
ffmpeg -i "input.mp4" -r 24 "output_24fps.mp4"
```

Change the frame rate to 24 fps.

```powershell
ffmpeg -i "input.mp4" -filter:v "minterpolate=mi_mode=mci,framerate=60" "output_60fps.mp4"
```

Increase the frame rate to 60 fps using motion interpolation (can introduce artifacts).

## Audio Manipulation Examples

```powershell
ffmpeg -i "input.mp4" -af "volume=0.5" "output_quieter.mp4"
```

Reduce the volume to 50%.

```powershell
ffmpeg -i "input.mp4" -af "volume=10dB" "output_louder.mp4"
```

Increase the volume by 10 dB.

```powershell
ffmpeg -i "input.mp4" -ac 2 "output_stereo.mp4"
```

Convert audio to stereo.

```powershell
ffmpeg -i "input.mp4" -ac 1 "output_mono.mp4"
```

Convert audio to mono.

```powershell
ffmpeg -i "input.mp4" -af "aresample=44100" "output_44100hz.mp4"
```

Change the sample rate to 44100 Hz.

## Image and Video Examples

```powershell
ffmpeg -i input.mp4 -s 1280x720 -c:a copy output.mp4
```

Change resolution of video

```powershell
ffmpeg -i input.mp4 -vn -ab 320k output.mp3
```

Removing video stream from a media file

```powershell
ffmpeg -i input.mp4 -r 1 -f image2 image-%2d.png
```

Extracting images from video  
`-r` - Set the frame rate. I.e. the number of frames to be extracted into images per second. The default value is 25.  
`-f` - Indicates the output format i.e, image format in our case.
image-%2d.png - Indicates how we want to name the extracted images. In this case, the names should start like image-01.png, image-02.png, image-03.png and so on. If you use %3d, then the name of images will start like image-001.png, image-002.png and so on.

---

```powershell
ffmpeg -i input.mp4 -filter:v "crop=w:h:x:y" output.mp4
```

Crop video  
`-filter:v` - Indicates the video filter.  
`crop` - Indicates crop filter.  
`w` - Width of the rectangle that we want to crop from the source video.  
`h` - Height of the rectangle.  
`x` - x coordinate of the rectangle that we want to crop from the source video.  
`y` - y coordinate of the rectangle.

Let us say you want to a video with a width of 640 pixels and a height of 480 pixels, from the position (200,150), the command would be:

```powershell
ffmpeg -i input.mp4 -filter:v "crop=640:480:200:150" output.mp4`
```

---

```powershell
ffmpeg -i input_file.mp4 -ss 00:01:30 -to 00:01:55 -c copy output.mp4
```

Clip sections of a video  
`-i input_file.mp4` - Specifies the input video file  
`-ss 00:01:30` - Sets the start time (1 minute and 30 seconds into the video)  
`-to 00:01:55` - Sets the end time of the video  
`-c copy` - Copies the streams without re-encoding (faster)  
`output.mp4` - Name of the output file

For a more accurate cut encoding can be used but will be slower.

```powershell
ffmpeg -ss 00:01:30 -i input_file.mp4 -t 00:00:30 -c:v libx264 -c:a aac output.mp4
```

---

## Stitch multiple videos together

First create a list of files.
**cmd:**

```cmd
(for %i in (*.mp4) do @echo file '%i') > files.txt
```

**Powershell:**

```Powershell
foreach ($i in Get-ChildItem .\*.mp4) {echo "file '$i'" >> files.txt}
```

**Bash:**

```bash
for f in *.mp4; do echo "file '$f'" >> videos.txt; done
```

or

```bash
printf "file '%s'\n" * > files.txt
```

Once your text file is ready it should contain all the names of files you want to concatenate in this format:

```powershell
file 'file2.mp4'
file 'file3.mp4'
file 'file1.mp4'
```

Now you can enter the following command to concatenate the files:

```powershell
ffmpeg -f concat -safe 0 -i files.txt -c copy output.mp4
```

With transcoding:

```powershell
ffmpeg -f concat -safe 0 -i input.txt -c:v libx264 -c:a aac output.mp4
```

`-f concat` - Specifies the concat format  
`-safe 0` - Allows ffmpeg to read files in the current directory  
`-i input.txt` - Input file containing the list of videos  
`-c copy` - Copies the codecs without re-encoding (faster)  
`-c:v libx264` - uses the x264 (H.264) video encoder  
`-c:a aac` - uses the AAC audio encoder  
`output.mp4` - Name of the output file

**Method 2:** Using the concat filter

```powershell
ffmpeg -i input_file.mp4 -i input2.mp4 -i input3.mp4 -filter_complex "[0:v][0:a][1:v][1:a][2:v][2:a]concat=n=3:v=1:a=1[outv][outa]" -map "[outv]" -map "[outa]" output.mp4
```

`-i input1.mp4 -i input2.mp4 -i input3.mp4` - Input video files  
`-filter_complex` - Applies complex filtering  
`concat=n=3:v=1:a=1` - Concatenates 3 inputs with 1 video and 1 audio stream each  
`[outv][outa]` - Labels for the output video and audio streams  
`-map "[outv]" -map "[outa]"` - Map the labeled streams to the output

---

```powershell
ffmpeg -i input.mp4 -aspect 16:9 output.mp4
```

Set aspect ratio. The commonly used aspect ratios are:

16:9  
4:3  
16:10  
5:4  
2:21:1  
2:35:1  
2:39:1

```powershell
ffmpeg -i "input.jpg" -c:v libx264 -loop 0 -t 10 "output.mp4"
```

Create a 10-second video from a still image (loop the image).

```powershell
ffmpeg -loop 1 -i "input.png" -c:v libx264 -t 5 -pix_fmt yuv420p "output.mp4"
```

Create a 5-second video from a still image (loop the image), specifying pixel format.

```powershell
ffmpeg -i "input.mp4" -vf "thumbnail" -frames:v 1 "output.jpg"
```

Create a thumbnail image from a video.

```powershell
ffmpeg -i "input.mp4" -vf "select=eq(n\,100)" -vframes 1 "output.png"
```

Extract a specific frame (frame 100) as an image.

```powershell
ffmpeg -i input.mp4 -vn -ar 44100 -ac 2 -ab 320k -f mp3 output.mp3
```

Extract and convert audio from media file

`-vn` - Indicates that we have disabled video recording in the output file.  
`-ar` - Set the audio frequency of the output file. The common values used are 22050, 44100, 48000 Hz.  
`-ac` - Set the number of audio channels.  
`-ab` - Indicates the audio bitrate.  
`-f` - Output file format. In our case, it's mp3 format.

## Encode .gif to .mp4 (ScreenToGif)

```powershell
ffmpeg -i example.gif -movflags faststart -pix_fmt yuv420p -vf "crop=trunc(iw/2)*2:trunc(ih/2)*2" video.mp4
```

`-movflags faststart` - This flag optimizes the video file for web playback by moving the moov atom (metadata for the video) to the beginning of the file. This allows the video to start playing before it is fully downloaded.

`-pix_fmt yuv420p` - Sets the pixel format to yuv420p, which is widely supported by most video players and is optimal for web playback.

`-vf` - This applies a video filter (-vf) to crop the input video.

`"crop=trunc(iw/2)*2:trunc(ih/2)*2` - Ensures the width (iw) and height (ih) of the cropped video are both even numbers, which is often a requirement for video encoding.

`trunc(iw/2)*2` - This calculates half of the input width, truncates the decimal, and multiplies it by 2 to get an even number.

`trunc(ih/2)*2` - Similarly, this does the same for the input height.

## Subtitle Examples

```powershell
ffmpeg -i "input.mp4" -vf "subtitles=subtitles.srt" "output_with_subs.mp4"
```

Add subtitles from a .srt file to the video (burn-in subtitles).

```powershell
ffmpeg -i "input.mp4" -i "subtitles.srt" -c copy -map 0 -map 1 "output_with_subs.mkv"
```

Add subtitles as a separate stream in an MKV container (subtitles can be turned on/off).

## Watermark Examples

```powershell
ffmpeg -i "input.mp4" -i "watermark.png" -filter_complex "overlay=10:10" "output_watermarked.mp4"
```

Add a watermark image to the video at position (10, 10).

```powershell
ffmpeg -i "input.mp4" -i "watermark.png" -filter_complex "overlay=(main_w-overlay_w-10):(main_h-overlay_h-10)" "output_bottom_right_watermarked.mp4"
```

Add a watermark to the bottom right corner.

## Complex Filtergraph Examples

```powershell
ffmpeg -i "input.mp4" -vf "split[main][tmp];[tmp]scale=640:480,deshake[shake];[main][shake]overlay=x=W-w-10:y=10" "output_deshaked_watermarked.mp4"
```

Deshake the video and add a watermark (requires the deshake filter, which may need to be enabled during compilation).

```powershell
ffmpeg -i "input.mp4" -vf "chromakey=0x3CB371:0.1:0.1" "output_chromakeyed.mp4"
```

Apply chromakey (green screen) effect. Adjust the color and similarity/blend values as needed.

## Streaming Examples (Basic)

**Note:** Streaming examples require a streaming server setup (e.g., using nginx with the RTMP module).

```powershell
ffmpeg -re -i "input.mp4" -c copy -f flv "rtmp://your_streaming_server/live/stream_key"
```

Stream a video to an RTMP server (replace with your server address and stream key).

## Miscellaneous Examples

```powershell
ffmpeg -i "concat:input1.ts|input2.ts|input3.ts" -c copy "output.ts"
```

Concatenate multiple Transport Stream (TS) files (alternative to the text file method). May not always work reliably.

```powershell
ffmpeg -i "input.mp4" -map 0:v -map 0:a -c copy "output.mkv"
```

Remux to MKV container (copy all video and audio streams).

```powershell
ffmpeg -i "input.mp4" -an "output_no_audio.mp4"
```

Remove audio from a video file.

```powershell
ffmpeg -i "input.mp4" -vn "output_audio_only.mp4"
```

Remove video from a video file (leaving only audio).

```powershell
ffmpeg -i "input.mp4" -f image2 "output\_%03d.png"
```

Extract each frame as a separate image (output001.png, output002.png, etc.).

```powershell
ffmpeg -framerate 24 -i "image%03d.png" -c:v libx264 "output.mp4"
```

Create a video from a sequence of images (image001.png, image002.png, etc.) at 24 fps.
