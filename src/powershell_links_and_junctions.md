# Hard Links, Soft Links, and Junctions in PowerShell

## Hard Links

- **Definition**: A hard link is a directory entry that associates a name with a file on a file system. It allows multiple names to refer to the same file data on disk.

### Creating a Hard Link

```powershell
New-Item -ItemType HardLink -Path "C:\Path\To\HardLink.txt" -Target "C:\Path\To\OriginalFile.txt"
```

### Example

```powershell
# Create a hard link
New-Item -ItemType HardLink -Path "C:\Links\HardLink.txt" -Target "C:\Original\MyFile.txt"
```

## Soft Links (Symbolic Links)

- **Definition**: A soft link (or symbolic link) is a special type of file that points to another file or directory by its path.

### Creating a Soft Link

```powershell
New-Item -ItemType SymbolicLink -Path "C:\Path\To\SoftLink.txt" -Target "C:\Path\To\OriginalFile.txt"
```

### Example

```powershell
# Create a symbolic link
New-Item -ItemType SymbolicLink -Path "C:\Links\SoftLink.txt" -Target "C:\Original\MyFile.txt"
```

## Junctions

- **Definition**: A junction is a type of symbolic link specific to directories in NTFS file systems, used to create a link to another directory.

### Creating a Junction

```powershell
New-Item -ItemType Junction -Path "C:\Path\To\Junction" -Target "C:\Path\To\TargetDirectory"
```

### Example

```powershell
# Create a junction
New-Item -ItemType Junction -Path "C:\Links\MyJunction" -Target "C:\Original\MyDirectory"
```
