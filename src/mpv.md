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

## All Keyboard Shortcuts

### Keyboard Control

- LEFT and RIGHT
  - Seek backward/forward 5 seconds. Shift+arrow does a 1 second exact seek (see <tt class="docutils literal"><span class="pre">--hr-seek</span></tt>).
- UP and DOWN
  - Seek forward/backward 1 minute. Shift+arrow does a 5 second exact seek (see <tt class="docutils literal"><span class="pre">--hr-seek</span></tt>).
- Ctrl+LEFT and Ctrl+RIGHT
  - Seek to the previous/next subtitle. Subject to some restrictions and might not always work; see <tt class="docutils literal"><span class="pre">sub-seek</span></tt> command.
- Ctrl+Shift+LEFT and Ctrl+Shift+RIGHT
  - Adjust subtitle delay so that the previous or next subtitle is displayed now. This is especially useful to sync subtitles to audio.
- [ and ]
  - Decrease/increase current playback speed by 10%.
- { and }
  - Halve/double current playback speed.
- BACKSPACE
  - Reset playback speed to normal.
- Shift+BACKSPACE
  - Undo the last seek. This works only if the playlist entry was not changed. Hitting it a second time will go back to the original position. See <tt class="docutils literal"><span class="pre">revert-seek</span></tt> command for details.
- Shift+Ctrl+BACKSPACE
  - Mark the current position. This will then be used by <tt class="docutils literal">Shift+BACKSPACE</tt> as revert position (once you seek back, the marker will be reset). You can use this to seek around in the file and then return to the exact position where you left off.
- &lt; and &gt;
  - Go backward/forward in the playlist.
- ENTER
  - Go forward in the playlist.
- Shift+HOME and Shift+END
  - Go to the first/last playlist entry.
- p and SPACE
  - Pause (pressing again unpauses).
- .
  - Step forward. Pressing once will pause, every consecutive press will play one frame and then go into pause mode again.

- ,
  - Step backward. Pressing once will pause, every consecutive press will play one frame in reverse and then go into pause mode again.
- q
  - Stop playing and quit.
- Q
  - Like <tt class="docutils literal">q</tt>, but store the current playback position. Playing the same file later will resume at the old playback position if possible. See [RESUMING PLAYBACK](https://mpv.io/manual/master/#resuming-playback).
- / and \*
  - Decrease/increase volume.
- KP\_DIVIDE and KP\_MULTIPLY
  - Decrease/increase volume.
- 9 and 0
  - Decrease/increase volume.
- m
  - Mute sound.
- \_
  - Cycle through the available video tracks.

- \#

  - Cycle through the available audio tracks.
- E
  - Cycle through the available Editions.
- f
  - Toggle fullscreen (see also <tt class="docutils literal"><span class="pre">--fs</span></tt>).
- ESC
  - Exit fullscreen mode.
- T
  - Toggle stay-on-top (see also <tt class="docutils literal"><span class="pre">--ontop</span></tt>).
- w and W
  - Decrease/increase pan-and-scan range. The <tt class="docutils literal">e</tt> key does the same as <tt class="docutils literal">W</tt> currently, but use is discouraged. See <tt class="docutils literal"><span class="pre">--panscan</span></tt> for more information.
- o and P
  - Show progression bar, elapsed time and total duration on the OSD.
- O
  - Toggle OSD states between normal and playback time/duration.
- v
  - Toggle subtitle visibility.
- j and J
  - Cycle through the available subtitles.
- z and Z
  - Adjust subtitle delay by -/+ 0.1 seconds. The <tt class="docutils literal">x</tt> key does the same as <tt class="docutils literal">Z</tt> currently, but use is discouraged.
- l
  - Set/clear A-B loop points. See <tt class="docutils literal"><span class="pre">ab-loop</span></tt> command for details.
- L
  - Toggle infinite looping.
- Ctrl++ and Ctrl+-
  - Adjust audio delay (A/V sync) by +/- 0.1 seconds.
- Ctrl+KP\_ADD and Ctrl+KP\_SUBTRACT
  - Adjust audio delay (A/V sync) by +/- 0.1 seconds.
- G and F
  - Adjust subtitle font size by +/- 10%.
- u
  - Switch between applying only <tt class="docutils literal"><span class="pre">--sub-ass-*</span></tt> overrides (default) to SSA/ASS subtitles, and overriding them almost completely with the normal subtitle style. See <tt class="docutils literal"><span class="pre">--sub-ass-override</span></tt> for more info.
- V
  - Cycle through which video data gets used for ASS rendering. See <tt class="docutils literal"><span class="pre">--sub-ass-use-video-data</span></tt> for more info.
- r and R
  - Move subtitles up/down. The <tt class="docutils literal">t</tt> key does the same as <tt class="docutils literal">R</tt> currently, but use is discouraged.
- s
  - Take a screenshot.
- S
  - Take a screenshot, without subtitles. (Whether this works depends on VO driver support.)
- Ctrl+s
  - Take a screenshot, as the window shows it (with subtitles, OSD, and scaled video).
- HOME
  - Seek to the beginning of the file.
- PGUP and PGDWN
  - Seek to the beginning of the previous/next chapter. In most cases, "previous" will actually go to the beginning of the current chapter; see <tt class="docutils literal"><span class="pre">--chapter-seek-threshold</span></tt>.
- Shift+PGUP and Shift+PGDWN
  - Seek backward or forward by 10 minutes. (This used to be mapped to PGUP/PGDWN without Shift.)
- b
  - Activate/deactivate debanding.
- d
  - Cycle the deinterlacing filter.
- A
  - Cycle aspect ratio override.
- Ctrl+h
  - Toggle hardware video decoding on/off.
- Alt+LEFT, Alt+RIGHT, Alt+UP, Alt+DOWN
  - Move the video rectangle (panning).
- Alt++ and Alt+-
  - Change video zoom.
- Alt+KP\_ADD and Alt+KP\_SUBTRACT
  - Change video zoom.
- Alt+BACKSPACE
  - Reset the pan/zoom settings.
- F8
  - Show the playlist and the current position in it.
- F9
  - Show the list of audio and subtitle streams.
- Ctrl+v
  - Append the file or URL in the clipboard to the playlist. If nothing is currently playing, it is played immediately. Only works on platforms that support the <tt class="docutils literal">clipboard</tt> property.
- i and I
  - Show/toggle an overlay displaying statistics about the currently playing file such as codec, framerate, number of dropped frames and so on. See [STATS](https://mpv.io/manual/master/#stats) for more information.

- ?
  - Toggle an overlay displaying the active key bindings. See [STATS](https://mpv.io/manual/master/#stats) for more information.
- DEL
  - Cycle OSC visibility between never / auto (mouse-move) / always
- `
  - Show the console. (ESC closes it again. See [CONSOLE](https://mpv.io/manual/master/#console).)

(The following keys are valid only when using a video output that supports the corresponding adjustment.)

- 1 and 2
  - Adjust contrast.
- 3 and 4
  - Adjust brightness.
- 5 and 6
  - Adjust gamma.
- 7 and 8
  - Adjust saturation.
- Alt+0 (and Command+0 on macOS)
  - Resize video window to half its original size.
- Alt+1 (and Command+1 on macOS)
  - Resize video window to its original size.
- Alt+2 (and Command+2 on macOS)
  - Resize video window to double its original size.
- Command + f (macOS only)
  - Toggle fullscreen (see also <tt class="docutils literal"><span class="pre">--fs</span></tt>).

(The following keybindings open a menu in the console that lets you choose from a list of items by typing part of the desired item, by clicking the desired item, or by navigating them with keybindings: <tt class="docutils literal">Down</tt> and <tt class="docutils literal">Ctrl+n</tt> go down, <tt class="docutils literal">Up</tt> and <tt class="docutils literal">Ctrl+p</tt> go up, <tt class="docutils literal">Page down</tt> and <tt class="docutils literal">Ctrl+f</tt> scroll down one page, and <tt class="docutils literal">Page up</tt> and <tt class="docutils literal">Ctrl+b</tt> scroll up one page.)

In track menus, selecting the current tracks disables it.

- g-p
  - Select a playlist entry.
- g-s
  - Select a subtitle track.
- g-S
  - Select a secondary subtitle track.
- g-a
  - Select an audio track.
- g-v
  - Select a video track.
- g-t
  - Select a track of any type.
- g-c
  - Select a chapter.
- g-e
  - Select an MKV edition or DVD/Blu-ray title.
- g-l
  - Select a subtitle line to seek to. This currently requires <tt class="docutils literal">ffmpeg</tt> in <tt class="docutils literal">PATH</tt>, or in the same folder as mpv on Windows.
- g-d
  - Select an audio device.
- g-h
  - Select a file from the watch history. Requires <tt class="docutils literal"><span class="pre">--save-watch-history</span></tt>.
- g-w
  - Select a file from watch later config files (see [RESUMING PLAYBACK](https://mpv.io/manual/master/#resuming-playback)) to resume playing. Requires <tt class="docutils literal"><span class="pre">--write-filename-in-watch-later-config</span></tt>.
- g-b
  - Select a defined input binding.
- g-r
  - Show the values of all properties.
- g-m, MENU, Ctrl+p
  - Show a menu with miscellaneous entries.

See [SELECT](https://mpv.io/manual/master/#select) for more information.

(The following keys are valid if you have a keyboard with multimedia keys.)

- PAUSE
  - Pause.
- STOP
  - Stop playing and quit.
- PREVIOUS and NEXT
  - Seek backward/forward 1 minute.
- ZOOMIN and ZOOMOUT
  - Change video zoom.

If you miss some older key bindings, look at <tt class="docutils literal"><span class="pre">etc/restore-old-bindings.conf</span></tt> in the mpv git repository.

### Mouse Control

- Left double click
  - Toggle fullscreen on/off.
- Right click
  - Toggle pause on/off.
- Forward/Back button
  - Skip to next/previous entry in playlist.
- Wheel up/down
  - Decrease/increase volume.
- Wheel left/right
  - Seek forward/backward 10 seconds.
- Ctrl+Wheel up/down
  - Change video zoom.
  