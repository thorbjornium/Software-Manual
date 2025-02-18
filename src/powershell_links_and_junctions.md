# Hard Links, Soft Links (Symbolic Links), and Junctions in PowerShell
<!-- toc -->
## Understanding the Concepts

## Hard Link

A hard link is essentially another name for the same file.  Both the original file and the hard link point to the same data on the disk. If you modify one, the other reflects the changes immediately because they are the same data.  You can't create hard links across different volumes/partitions.  Deleting the original file doesn't delete the data as long as at least one hard link remains

## Soft Link (Symbolic Link)

A soft link is a pointer to another file or directory.  It acts as a shortcut. If you modify the original file, the changes are reflected when you access the soft link. However, if you delete the original file, the soft link becomes broken (it points to a non-existent location).  Soft links *can* cross volume boundaries

## Junction

A junction (also called a directory symbolic link) is similar to a soft link, but it works specifically for directories.  It points to another directory on the *same* volume.  Like hard links, junctions continue to work even if the original directory is moved within the same volume.  They are transparent to applications; an application accessing the junction thinks it's accessing a real directory.  Junctions cannot cross volume boundaries

## PowerShell Cmdlets

The primary cmdlet for creating links in PowerShell is `New-Item`

## 1. Creating Hard Links

### Syntax

```powershell

New-Item -ItemType HardLink -Path "<link_path>" -Target "<target_file>"

```

### Example

```powershell
# Create a hard link named 'MyHardLink.txt' to the file 'OriginalFile.txt'

New-Item -ItemType HardLink -Path ".\MyHardLink.txt" -Target ".\OriginalFile.txt"

```

## Important Considerations for Hard Links

* The `-Path` specifies the name and location of the *new* hard link

* The `-Target` specifies the path to the *existing* file you want to create a hard link to

* Hard links can only be created to files, not directories

* Hard links must be on the same volume

## 2. Creating Soft Links (Symbolic Links)

### Syntax

```powershell

New-Item -ItemType SymbolicLink -Path "<link_path>" -Target "<target_path>"

```

### Example

```powershell
# Create a symbolic link named 'MySoftLink.txt' to the file 'OriginalFile.txt'

New-Item -ItemType SymbolicLink -Path ".\MySoftLink.txt" -Target ".\OriginalFile.txt"

# Create a symbolic link named 'MyDirectoryLink' to the directory 'OriginalDirectory'

New-Item -ItemType SymbolicLink -Path ".\MyDirectoryLink" -Target ".\OriginalDirectory"

```

## Key Points for Soft Links:

* `-Path`:  The path and name of the symbolic link

* `-Target`: The path to the file or directory the link will point to

* Soft links can point to files or directories

* Soft links can cross volume boundaries

## 3. Creating Junctions

### Syntax

```powershell

New-Item -ItemType Junction -Path "<junction_path>" -Target "<target_directory>"

```

### Example

```powershell
# Create a junction named 'MyJunction' that points to the directory 'OriginalDirectory'

New-Item -ItemType Junction -Path ".\MyJunction" -Target ".\OriginalDirectory"

```

## Important Details for Junctions

* `-Path`: The path and name of the junction (which will be a directory)

* `-Target`: The path to the *existing* directory the junction will point to

* Junctions can only point to directories

* Junctions must be on the same volume

## Checking Link Type

You can use `Get-Item` to inspect the link and determine its type

```powershell

$item = Get-Item ".\MySoftLink.txt" # Replace with your link path

$item.LinkType

```

The `$item.LinkType` property will return

* `HardLink`

* `SymbolicLink`

* `Junction`

* `NotALink` (if it's a regular file or directory)

## Removing Links

### Use `Remove-Item` to remove any of these links

```powershell

Remove-Item ".\MyHardLink.txt"

Remove-Item ".\MySoftLink.txt"

Remove-Item ".\MyJunction"

```

## Important Notes on Removal

* Removing a hard link *does not* delete the original file's data, unless it's the last hard link to that data

* Removing a soft link *does not* delete the original file or directory. It only removes the link itself

* Removing a junction *does not* delete the target directory or its contents. It only removes the junction point

## Error Handling

### It's a good practice to include error handling when creating links

```powershell

try {

New-Item -ItemType SymbolicLink -Path ".\MySoftLink.txt" -Target ".\NonExistentFile.txt" -ErrorAction Stop

Write-Host "Symbolic link created successfully."

}

catch {

Write-Host "Error creating symbolic link: $($_.Exception.Message)"

}

```

The `-ErrorAction Stop` parameter will cause the script to stop immediately if an error occurs, and the `catch` block will handle the error
