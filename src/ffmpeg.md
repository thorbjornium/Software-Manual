# FFmpeg

<!-- toc -->

## Remux 2160p .mkv to .mp4

```plaintext
ffmpeg -y -i Input_File.mkv -strict experimental -loglevel error -stats -map 0:v? -map 0:a? -dn -map_chapters -1 -movflags +faststart -c:v copy -c:a copy -strict -2 Output_File.mp4
```

## Encode .gif to .mp4  (ScreenToGif)

```plaintext
ffmpeg -i example.gif -movflags faststart -pix_fmt yuv420p -vf "crop=trunc(iw/2)*2:trunc(ih/2)*2" video.mp4  

```

## Extract and convert audio from media file

```plaintext
ffmpeg -i input.mp4 -vn -ar 44100 -ac 2 -ab 320k -f mp3 output.mp3
```

`-vn` - Indicates that we have disabled video recording in the output file.  
`-ar` - Set the audio frequency of the output file. The common values used are  22050, 44100, 48000 Hz.  
`-ac` - Set the number of audio channels.  
`-ab` - Indicates the audio bitrate.  
`-f` - Output file format. In our case, it's mp3 format.  

## Change resolution of video files

```plaintext
ffmpeg -i input.mp4 -s 1280x720 -c:a copy output.mp4  
```

## Removing audio stream from a video file

```plaintext
ffmpeg -i input.mp4 -an output.mp4  
```

## Removing video stream from a media file

```plaintext
ffmpeg -i input.mp4 -vn -ab 320k output.mp3  
```

## Extracting images from video

```plaintext
ffmpeg -i input.mp4 -r 1 -f image2 image-%2d.png  
```

`-r` - Set the frame rate. I.e the number of frames to be extracted into images per second. The default value is 25.  
`-f` - Indicates the output format i.e image format in our case.
image-%2d.png - Indicates how we want to name the extracted images. In this case, the names should start like image-01.png, image-02.png, image-03.png and so on. If you use %3d, then the name of images will start like image-001.png, image-002.png and so on.  

## Crop video

```plaintext
ffmpeg -i input.mp4 -filter:v "crop=w:h:x:y" output.mp4
```

`-filter:v` - Indicates the video filter.  
`crop` - Indicates crop filter.  
`w` - Width of the rectangle that we want to crop from the source video.  
`h` - Height of the rectangle.  
`x` - x coordinate of the rectangle that we want to crop from the source video.  
`y` - y coordinate of the rectangle.  

Let us say you want to a video with a width of 640 pixels and a height of 480 pixels, from the position (200,150), the command would be:

```plaintext
ffmpeg -i input.mp4 -filter:v "crop=640:480:200:150" output.mp4`  
```

## Clip sections of a video  

```plaintext
ffmpeg -i input_file.mp4 -ss 00:01:30 -to 00:01:55 -c copy output.mp4
```

`-i input_file.mp4` - Specifies the input video file  
`-ss 00:01:30` - Sets the start time (1 minute and 30 seconds into the video)  
`-to 00:01:55` - Sets the end time of the video  
`-c copy` - Copies the streams without re-encoding (faster)  
`output.mp4` - Name of the output file  

For a more accurate cut encoding can be used but will be slower.

```plaintext
ffmpeg -ss 00:01:30 -i input_file.mp4 -t 00:00:30 -c:v libx264 -c:a aac output.mp4  
```

## Stitch multiple videos together  

First create a list of files.

cmd:

```cmd
(for %i in (*.mp4) do @echo file '%i') > files.txt
```

Powershell:

```Powershell
foreach ($i in Get-ChildItem .\*.mp4) {echo "file '$i'" >> files.txt}  
```

Bash:

```bash
for f in *.mp4; do echo "file '$f'" >> videos.txt; done
```

or  

```bash
printf "file '%s'\n" * > files.txt
```

Once your text file is ready it should contain all the names of files you want to concatenate in this format:

```plaintext
file 'file2.mp4'
file 'file3.mp4'
file 'file1.mp4'
```

Now you can enter the following command to concatenate the files:

```plaintext
ffmpeg -f concat -safe 0 -i files.txt -c copy output.mp4  
```

With transcoding:

```plaintext
ffmpeg -f concat -safe 0 -i input.txt -c:v libx264 -c:a aac output.mp4  
```

`-f concat` - Specifies the concat format  
`-safe 0` - Allows ffmpeg to read files in the current directory  
`-i input.txt` - Input file containing the list of videos  
`-c copy` - Copies the codecs without re-encoding (faster)  
`-c:v libx264` - uses the x264 (H.264) video encoder  
`-c:a aac` - uses the AAC audio encoder  
`output.mp4` - Name of the output file  

Method 2: Using the concat filter  

```plaintext
ffmpeg -i input_file.mp4 -i input2.mp4 -i input3.mp4 -filter_complex "[0:v][0:a][1:v][1:a][2:v][2:a]concat=n=3:v=1:a=1[outv][outa]" -map "[outv]" -map "[outa]" output.mp4
```

`-i input1.mp4 -i input2.mp4 -i input3.mp4` - Input video files  
`-filter_complex` - Applies complex filtering  
`concat=n=3:v=1:a=1` - Concatenates 3 inputs with 1 video and 1 audio stream each  
`[outv][outa]` - Labels for the output video and audio streams  
`-map "[outv]" -map "[outa]"` - Maplaintext the labeled streams to the output  

## Set aspect ratio

```plaintext
ffmpeg -i input.mp4 -aspect 16:9 output.mp4
```

The commonly used aspect ratios are:

16:9  
4:3  
16:10  
5:4  
2:21:1  
2:35:1  
2:39:1  

## Loop an image with audio

```plaintext
ffmpeg -loop 1 -i inputimage.jpg -i inputaudio.mp3 -c:v libx264 -c:a aac -strict experimental -b:a 192k -shortest output.mp4
```

## Scale and crop an image  

Scaled and cropped to width: 3840 pixels and height: 2160 pixels

```plaintext
ffmpeg -i image_before.jpg -vf scale=3840:-1:flags=lanczos,crop=3840:2160 image_after.jpg
```

Scaled and cropped to height: 2160 pixels and width: 3840 pixels

```plaintext
ffmpeg -i image_before.jpg -vf scale=-1:2160:flags=lanczos,crop=3840:2160 image_after.jpg
```

## Links

[httplaintext://ostechnix.com/20-ffmpeg-commands-beginners/](httplaintext://ostechnix.com/20-ffmpeg-commands-beginners/)  

[httplaintext://www.ffmpeg.org/documentation.html](httplaintext://www.ffmpeg.org/documentation.html)  

[httplaintext://github.com/NapoleonWils0n/ffmpeg-scripts](httplaintext://github.com/NapoleonWils0n/ffmpeg-scripts)
