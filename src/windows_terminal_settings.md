# Settings

<!-- toc -->

Open settings: <kbd>Ctrl</kbd> + <kbd>,</kbd>

To open command palette press: <kbd>Ctrl</kbd> + <kbd>,</kbd>
All default keyboard shortcuts can be found in this menu. Some are not bound.
Click on the JSON file in the lower left corner to edit bound and unbound shortcuts. To view all default
settings <kbd>Alt</kbd> + click the JSON file.

## Terminal Icons

Install Terminal icons:

{{#tabs global="install"}}
{{#tab name="Winget"}}

```Powershell
Install-Module -Name Terminal-Icons -Repository PSGallery
```  

{{#endtab}}
{{#tab name="Scoop"}}

```Powershell
scoop bucket add extras
scoop install terminal-icons
```  

{{#endtab}}
{{#endtabs}}

Add `Import-Module -Name Terminal-Icons` to your PowerShell profile.

More info [here](https://github.com/devblackops/Terminal-Icons)
