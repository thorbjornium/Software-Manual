# mpv User Guide (Windows) - Basic

<!-- toc -->

## Introduction

mpv is a free and open-source media player known for its flexibility, minimal GUI, and powerful command-line options. This guide covers basic usage on Windows. mpv relies heavily on keyboard shortcuts and configuration files for customization.

## Installation

1. Downloads can be found [here](https://mpv.io/installation/) and scripts [here](https://github.com/mpv-player/mpv/wiki/User-Scripts)

2. Extract the downloaded archive to a folder of your choice (e.g., `C:\mpv`).

3. **Optional (but Recommended): Add mpv to your PATH.** This allows you to run `mpv` from any command prompt or PowerShell window.

   - Search for "Edit the system environment variables" in the Windows Start Menu.
   - Click "Environment Variables...".
   - Under "System variables", find the "Path" variable and click "Edit...".
   - Click "New" and add the path to the mpv directory (e.g., `C:\mpv`).
   - Click "OK" on all windows.

## Basic Usage

### Playing Media

1. **Double-Click (Associate File Types):** The easiest way is to associate common video file types (e.g., `.mp4`, `.mkv`, `.avi`) with `mpv.exe`. Right-click on a video file, select "Open with", choose "mpv", and check the box that says "Always use this app to open .[file extension] files".

2. **Drag and Drop:** Drag a video file onto the `mpv.exe` icon or a running mpv window.

3. **Command Line/PowerShell:** Open a command prompt or PowerShell window and type:

   ```powershell
   mpv "path\to\your\video.mp4"
   ```

   Replace `"path\to\your\video.mp4"` with the actual path to your video file. If you added mpv to your PATH, you can run this command from any directory. Otherwise, you'll need to navigate to the `mpv` directory first.

### Basic Controls (Keyboard Shortcuts)

These are the most common and essential shortcuts

| Key(s)        | Action                                |
| ------------- | ------------------------------------- |
| Spacebar      | Pause/Play                            |
| Left Arrow    | Seek backward 5 seconds               |
| Right Arrow   | Seek forward 5 seconds                |
| Up Arrow      | Volume up                             |
| Down Arrow    | Volume down                           |
| q or Esc      | Quit mpv                              |
| f             | Toggle fullscreen                     |
| m             | Mute/Unmute                           |
| > (Shift + .) | Next frame                            |
| < (Shift + ,) | Previous frame                        |
| o             | Show playback statistics (OSD)        |
| i             | Show current filename (OSD)           |
| 1             | Cycle audio tracks                    |
| j             | Cycle subtitle tracks                 |
| v             | Toggle subtitle visibility            |
| 9             | Decrease subtitle delay               |
| 0             | Increase subtitle delay               |
| [             | Decrease playback speed               |
| ]             | Increase playback speed               |
| \             | Reset playback speed to normal (1.0x) |
| Ctrl + +      | Zoom in                               |
| Ctrl + -      | Zoom out                              |
| Ctrl + 0      | Reset zoom                            |

## On-Screen Display (OSD)

mpv has a minimal OSD that appears briefly when you perform actions like seeking or changing volume

## Configuration File (mpv.conf)

mpv is highly customizable through a configuration file named `mpv.conf`

1. **Location:** The `mpv.conf` file should be placed in the following directory

`%APPDATA%\mpv` (usually `C:\Users\[Your Username]\AppData\Roaming\mpv`)

Create the `mpv` directory if it doesn't exist

2.**Creating the File:** Create a new text file named `mpv.conf` in the `%APPDATA%\mpv` directory

3.**Example Configuration:** Here's a basic example

### Example `mpv.conf`

```txt
## Start in fullscreen
fullscreen=yes

## Volume at startup (0-100)
volume=70

### Prefer English audio tracks
alang=en

### Prefer English subtitles
slang=en

### Save position on exit (resumes playback)
save-position-on-quit

### Screenshot directory
screenshot-directory="C:\\Users\\[Your Username]\\Pictures\\mpv_screenshots"

### Example of rebinding a key
W=add volume 5

```

- Replace `[Your Username]` with your actual Windows username in the `screenshot-directory` line

- The `W=add volume 5` line rebinds the `W` key to increase the volume by 5%

  4.**Restart mpv:** Changes to `mpv.conf` take effect when you restart mpv

### Common Command-Line Options

You can also use command-line options to control mpv's behavior. These options override settings in `mpv.conf`

```powershell

mpv --fullscreen --volume=80 "path\to\your\video.mp4"

```

| Option               | Description                                              |
| -------------------- | -------------------------------------------------------- |
| --fullscreen         | Start in fullscreen mode.                                |
| --volume=\<value>    | Set the initial volume (0-100).                          |
| --loop               | Loop the video indefinitely.                             |
| --start=\<time>      | Start playback at a specific time (e.g., --start=01:30). |
| --no-border          | Remove window borders.                                   |
| --title=\<title>     | Set the window title.                                    |
| --audio-file=\<file> | Play an external audio file.                             |
| --sub-file=\<file>   | Load an external subtitle file.                          |

### Advanced Usage

mpv has many more advanced features, including

- **Video and Audio Filters:** Apply various filters to enhance or modify the video and audio output

- **Shaders:** Use shaders for advanced video processing

- **Lua Scripting:** Extend mpv's functionality with Lua scripts

- **Custom Keybindings:** Rebind any key to any mpv command

- **OSD Customization:** Customize the appearance of the OSD

## Advanced Commands (Configuration File)

Many mpv settings are best configured in the `mpv.conf` file (location varies by OS). Here are some examples:

- **`~/.config/mpv/mpv.conf` (Linux/macOS)**
- **`%APPDATA%\mpv\mpv.conf` (Windows)**

**Important Notes for Windows:**

- **Hardware Decoding:** Experiment with different hardware decoding options in your `mpv.conf` file. Common options for Windows include `hwdec=d3d11va`, `hwdec=dxva2-copy`, or `hwdec=auto`. The best option depends on your graphics card and drivers.
- **File Paths:** When using command-line arguments with file paths, be mindful of spaces in the path. Enclose the entire path in double quotes if it contains spaces (e.g., `mpv "C:\My Videos\movie.mp4"`).
- **Command Prompt/PowerShell:** You can run MPV commands from the Command Prompt or PowerShell. Make sure MPV's directory is in your system's `PATH` environment variable, or specify the full path to `mpv.exe`.

## Command-Line Examples (Windows)

```powershell
mpv file.mp4 - Play a file.
mpv --loop file.mp4 - Play a file in a loop.
mpv --start=1:30 file.mp4 - Start playback at 1 minute 30 seconds.
mpv --playlist=movies.txt - Play a playlist.  Ensure the playlist file uses Windows-style line endings (CRLF).
mpv --volume=80 file.mp4 - Set the volume to 80%.
mpv --speed=2.0 file.mp4 - Play at 2x speed.
mpv --no-audio file.mp4 - Play without audio.
mpv --no-video file.mp4 - Play without video (audio only).
mpv --geometry=50%+50% file.mp4 - Position the window at 50% horizontal and vertical.
mpv "C:\My Videos\movie with spaces.mp4" - Play a file with spaces in the filename.
```
