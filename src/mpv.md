# MPV Player: Keyboard Shortcuts and Scripts

<!-- toc -->

## Keyboard Shortcuts

### Playback Controls

| **Shortcut**       | **Action**                          |
|---------------------|-------------------------------------|
| Space            | Play/Pause                         |
| Enter            | Play/Pause                         |
| P                | Pause                              |
| . (Period)       | Frame-step (move forward 1 frame)  |
| , (Comma)        | Frame-back (move backward 1 frame) |
| → (Right Arrow)  | Seek forward 5 seconds             |
| ← (Left Arrow)   | Seek backward 5 seconds            |
| ↑ (Up Arrow)     | Seek forward 1 minute              |
| ↓ (Down Arrow)   | Seek backward 1 minute             |
| Ctrl + →         | Seek forward 10 seconds            |
| Ctrl + ←         | Seek backward 10 seconds           |
| [                | Decrease playback speed by 10%     |
| ]                | Increase playback speed by 10%     |
| {                | Halve playback speed               |
| }                | Double playback speed              |
| Backspace        | Reset playback speed to normal     |

### Volume and Audio

| **Shortcut**       | **Action**                          |
|---------------------|-------------------------------------|
| ↑ (Up Arrow)     | Increase volume                    |
| ↓ (Down Arrow)   | Decrease volume                    |
| 9                | Decrease volume                    |
| 0                | Increase volume                    |
| M                | Mute/Unmute                        |
| #                | Cycle through audio tracks         |
| Ctrl + +         | Increase audio delay               |
| Ctrl + -         | Decrease audio delay               |

### Video and Subtitles

| **Shortcut**       | **Action**                          |
|---------------------|-------------------------------------|
| F                | Toggle fullscreen                  |
| T                | Toggle stay-on-top                 |
| V                | Toggle subtitle visibility         |
| J                | Cycle through subtitle tracks      |
| Z                | Adjust subtitle delay forward      |
| X                | Adjust subtitle delay backward     |
| Ctrl + +         | Zoom in                            |
| Ctrl + -         | Zoom out                           |
| Alt + ↑          | Move subtitles up                  |
| Alt + ↓          | Move subtitles down                |

### Playlist and Files

| **Shortcut**       | **Action**                          |
|---------------------|-------------------------------------|
| >                | Next file in playlist              |
| <                | Previous file in playlist          |
| Ctrl + P         | Show playlist                      |
| Ctrl + L         | Load a file                        |

### Screenshots and Recording

| **Shortcut**       | **Action**                          |
|---------------------|-------------------------------------|
| S                | Take a screenshot (without subs)   |
| s                | Take a screenshot (with subs)      |
| Ctrl + S         | Take a screenshot (original size)  |
| Ctrl + R         | Start/stop recording               |

### Miscellaneous

| **Shortcut**       | **Action**                          |
|---------------------|-------------------------------------|
| Q                | Quit and save playback position    |
| Ctrl + Q         | Force quit                         |
| I                | Show file information              |
| O                | Show progress bar and time         |
| Tab              | Show on-screen stats               |
| ~ (Tilde)        | Show console                       |

---

## Popular MPV Scripts

MPV supports Lua and JavaScript scripts for advanced functionality. Here are some popular scripts:

### 1. **mpv-quality-menu**

- **Description**: Adds a menu to easily switch between video/audio quality options (e.g., 1080p, 720p) for streaming services like YouTube.
- **GitHub**: [mpv-quality-menu](https://github.com/christoph-heinrich/mpv-quality-menu)

### 2. **autoload**

- **Description**: Automatically loads and plays the next file in the directory.
- **GitHub**: [autoload](https://github.com/mpv-player/mpv/blob/master/TOOLS/lua/autoload.lua)

### 3. **thumbfast**

- **Description**: Generates and displays thumbnails while seeking.
- **GitHub**: [thumbfast](https://github.com/po5/thumbfast)

### 4. **mpv-webm**

- **Description**: Allows you to create WebM clips from videos.
- **GitHub**: [mpv-webm](https://github.com/ekisu/mpv-webm)

### 5. **mpv-scripts**

- **Description**: A collection of useful scripts, including:
  - **auto-profiles**: Automatically applies profiles based on media properties.
  - **reload**: Reloads the current file.
  - **stats**: Displays detailed playback statistics.
- **GitHub**: [mpv-scripts](https://github.com/mpv-player/mpv/wiki/User-Scripts)

### 6. **mpv-history**

- **Description**: Keeps a history of played files and allows you to resume playback.
- **GitHub**: [mpv-history](https://github.com/zenyd/mpv-scripts)

### 7. **mpv-playlistmanager**

- **Description**: A playlist manager with a graphical interface.
- **GitHub**: [mpv-playlistmanager](https://github.com/jonniek/mpv-playlistmanager)

### 8. **mpv-osc-tethys**

- **Description**: A modern on-screen controller (OSC) for MPV.
- **GitHub**: [mpv-osc-tethys](https://github.com/Zren/mpv-osc-tethys)

---

## How to Install Scripts

1. Download the script file (usually a `.lua` or `.js` file).
2. Place it in the `scripts` directory:
   - Windows: `%APPDATA%\mpv\scripts\`
   - Linux/macOS: `~/.config/mpv/scripts/`
3. Restart MPV to load the script.

---

## Custom Keybindings

You can add custom keybindings in your `input.conf` file (located in the same directory as `mpv.conf`). This file allows you to define your own keyboard shortcuts for MPV. Here’s how to set it up:

### Location of `input.conf`

- **Windows**: `%APPDATA%\mpv\input.conf`
- **Linux/macOS**: `~/.config/mpv/input.conf`

### Example Keybindings

Here are some examples of custom keybindings you can add to `input.conf`:

```ini
# Cycle through audio tracks
a cycle audio

# Cycle through subtitle tracks
s cycle sub

# Toggle mute
m cycle mute

# Take a screenshot (without subtitles)
Ctrl+s screenshot video

# Take a screenshot (with subtitles)
Ctrl+Shift+s screenshot

# Increase volume by 10
+ add volume 10

# Decrease volume by 10
- add volume -10

# Seek forward by 30 seconds
Shift+Right seek 30

# Seek backward by 30 seconds
Shift+Left seek -30

# Toggle fullscreen
f cycle fullscreen

# Quit and save playback position
q quit
```
