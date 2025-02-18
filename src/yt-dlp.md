# yt-dlp

<!-- toc -->

## yt-dlp User Guide (Windows) - Basic

## Introduction

yt-dlp is a command-line program to download videos from YouTube and other video sites. It's a fork of youtube-dl, with a focus on new features, fixes, and speed. This guide covers basic usage on Windows.

## Installation

1. Download yt-dlp.exe: Download the latest Windows executable (`yt-dlp.exe`) from the official GitHub releases page: [https://github.com/yt-dlp/yt-dlp/releases](https://github.com/yt-dlp/yt-dlp/releases). Look for the `yt-dlp.exe` file.

2. Optional (but Recommended): Add yt-dlp to your PATH. This allows you to run `yt-dlp` from any command prompt or PowerShell window.

- Search for "Edit the system environment variables" in the Windows Start Menu.
- Click "Environment Variables...".
- Under "System variables", find the "Path" variable and click "Edit...".
- Click "New" and add the directory where you saved `yt-dlp.exe` (e.g., `C:\yt-dlp`).
- Click "OK" on all windows.

3. Optional (but Recommended): Install FFmpeg. yt-dlp uses FFmpeg for merging audio and video, and for certain conversions. Download FFmpeg from [https://ffmpeg.org/download.html](https://ffmpeg.org/download.html). Download the Windows builds (usually a `.zip` file). Extract the contents, and add the `bin` directory inside the extracted folder to your PATH, similar to how you added yt-dlp. For example, if you extracted FFmpeg to `C:\ffmpeg`, you would add `C:\ffmpeg\bin` to your PATH.

## Basic Usage

### Downloading a Video

1. Open Command Prompt or PowerShell: Press `Win + R`, type `cmd` (for Command Prompt) or `powershell` (for PowerShell), and press Enter.

2. Run yt-dlp: Type the following command and press Enter:

```powershell
yt-dlp "URL_OF_THE_VIDEO"
```

Replace `"URL_OF_THE_VIDEO"` with the actual URL of the YouTube video or other supported video site. For example:

```powershell
yt-dlp https://www.youtube.com/watch?v=dQw4w9WgXcQ
```

If you added yt-dlp to your PATH, you can run this command from any directory. Otherwise, you'll need to navigate to the directory where you saved `yt-dlp.exe` first.

3 .Progress and Output: yt-dlp will display progress information in the console window as it downloads the video. The downloaded video file will be saved in the current directory by default.

## Common Options

yt-dlp has many options to customize the download process. Here are some of the most common

| Option                           | Description                                                                                                                                                                                                        |
| -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `-F`, `--list-formats`           | List all available formats for the video. Use this to find the format code you want to download (e.g., `137` for 1080p video without audio).                                                                       |
| `-f`, `--format <format>`        | Specify the format to download. Use the format code from `--list-formats`. You can specify multiple formats separated by `+` to merge video and audio (e.g., `-f 137+140` for 1080p video and high-quality audio). |
| `-o`, `--output <template>`      | Specify the output filename template. Use placeholders like `%title` (video title), `%id` (video ID), `%ext` (file extension). For example: `-o "%(title)s-%(id)s.%(ext)s"`.                                       |
| `-P`, `--paths <paths>`          | Specify the download path. For example: `-P "C:\Downloads"`.                                                                                                                                                       |
| `--merge-output-format <format>` | Specify the container format to use when merging video and audio (e.g., `--merge-output-format mp4`, `--merge-output-format mkv`). Requires FFmpeg.                                                                |
| `--extract-audio`                | Extract the audio from the video.                                                                                                                                                                                  |
| `--audio-format <format>`        | Specify the audio format to extract (e.g., `--audio-format mp3`, `--audio-format aac`). Requires FFmpeg.                                                                                                           |
| `--audio-quality <quality>`      | Specify the audio quality to extract (e.g., `--audio-quality 0` for best, `--audio-quality 5` for good).                                                                                                           |
| `--write-sub`                    | Download subtitles (if available).                                                                                                                                                                                 |
| `--write-auto-sub`               | Download automatically generated subtitles (if available).                                                                                                                                                         |
| `--sub-lang <languages>`         | Specify the subtitle language(s) to download (e.g., `--sub-lang en,es`).                                                                                                                                           |
| `--embed-subs`                   | Embed subtitles into the video file. Requires FFmpeg.                                                                                                                                                              |
| `--cookies <file>`               | Load cookies from a file. Useful for downloading age-restricted content or content that requires login. You can typically export cookies from your browser using a browser extension.                              |
| `--username <username>`          | Specify a username for sites that require login.                                                                                                                                                                   |
| `--password <password>`          | Specify a password for sites that require login.                                                                                                                                                                   |
| `--ignore-errors`                | Continue downloading even if some videos in a playlist or a batch fail.                                                                                                                                            |
| `--no-overwrites`                | Do not overwrite existing files.                                                                                                                                                                                   |
| `--update`                       | Update yt-dlp to the latest version.                                                                                                                                                                               |

## Examples

### Downloading the Best Available Video and Audio

```powershell
yt-dlp https://www.youtube.com/watch?v=dQw4w9WgXcQ
```

### Listing Available Formats

```powershell

yt-dlp -F https://www.youtube.com/watch?v=dQw4w9WgXcQ

```

### Downloading a Specific Format (Video and Audio) and Merging with FFmpeg

First, list the formats to find the video and audio format codes. Then

```powershell

yt-dlp -f 137+140 --merge-output-format mp4 https://www.youtube.com/watch?v=dQw4w9WgXcQ

```

This downloads format `137` (1080p video) and `140` (high-quality audio) and merges them into an MP4 file using FFmpeg

### Downloading Only Audio in MP3 Format

```powershell

yt-dlp --extract-audio --audio-format mp3 --audio-quality 0 https://www.youtube.com/watch?v=dQw4w9WgXcQ

```

This extracts the audio as an MP3 file with the best available quality

### Downloading Subtitles

```powershell

yt-dlp --write-sub --sub-lang en https://www.youtube.com/watch?v=dQw4w9WgXcQ

```

This downloads English subtitles (if available)

### Setting the Output Filename and Directory

```powershell

yt-dlp -o "%(title)s-%(id)s.%(ext)s" -P "C:\Downloads" https://www.youtube.com/watch?v=dQw4w9WgXcQ

```

This saves the video to `C:\Downloads` with a filename like `Video Title-dQw4w9WgXcQ.mp4`

### Configuration File (yt-dlp.conf)

yt-dlp can also be configured using a configuration file

1.Location: The `yt-dlp.conf` file should be placed in one of the following locations (yt-dlp will search in this order)

- The current working directory

- `%APPDATA%\yt-dlp` (usually `C:\Users\[Your Username]\AppData\Roaming\yt-dlp`). Create the `yt-dlp` directory if it doesn't exist

- `%LOCALAPPDATA%\yt-dlp` (usually `C:\Users\[Your Username]\AppData\Local\yt-dlp`)

- `~/.config/yt-dlp` (This is more relevant for Linux/macOS but might work in a Cygwin/MSYS2 environment)

2.Creating the File: Create a new text file named `yt-dlp.conf` in one of the above directories

3.Example Configuration:

```txt
# Always extract audio as mp3

--extract-audio

--audio-format mp3

--audio-quality 0

# Set a default output directory

-P "C:\Downloads"

# Set a default output filename template

-o "%(title)s-%(id)s.%(ext)s"

# Always download subtitles in English

--write-sub

--sub-lang en

# Use a proxy (replace with your proxy address)

 --proxy "http://127.0.0.1:8080"
```

- Uncomment the `--proxy` line and replace the address with your proxy server if needed

  4.Using the Configuration File: yt-dlp will automatically load the configuration file when it runs. You can still override options from the command line

## Updating yt-dlp

Use the `--update` option to update yt-dlp to the latest version

```powershell

yt-dlp --update

```

## Troubleshooting

- "yt-dlp is not recognized as an internal or external command": This means yt-dlp is not in your PATH. Double-check the installation instructions

- "FFmpeg not found": This means FFmpeg is not in your PATH, or yt-dlp cannot find it. Double-check the FFmpeg installation instructions

- Download errors: Try updating yt-dlp. Some sites change their structure frequently, requiring updates to yt-dlp. Also, check your internet connection

- Age-restricted content: You may need to pass cookies to yt-dlp using the `--cookies` option
