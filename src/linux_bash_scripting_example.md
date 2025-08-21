# Bash Scripting

<!-- toc -->

## Bash script checker

Check your script on [https://www.shellcheck.net/](https://www.shellcheck.net/)

## Objectives  

Create a bash script to perform user management tasks as outlined below:  

1. Create a new group. Each group must have a unique name. The script must check to ensure that no duplicate group names exist on the system. If a duplicate is found, an error needs to be reported, and the administrator must try another group name.  

2. Create a new user. Each user must have a unique name. The script must check to ensure that no duplicate usernames exist on the system. If a duplicate is found, an error needs to be reported and the administrator must try another username. The user will have a Bash login shell and belong to the group that was created in the previous step.  

3. Create a password for each user that is created.  

4. Ensure that the new user created is a member of the new group created.  

5. Create a directory at the root / of the file system with same name as the user created.  

6. Set the ownership of the directory to the user and group created.  

7. Set the permissions of the directory to full control for the owner and full control for the group created.  

8. Set the permissions to ensure that only the owner of a file can delete it from the directory.  

9. Ensure that the script is executable.  

## User Management Script Assignment Outline

| Objective | Description | Commands/Concepts Used |
| :------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **1. Create New Group** | Create a new group with a unique name. Check for duplicates and report errors if found. | `getent group`, `groupadd`, `$?` (exit status check) |
| **2. Create New User** | Create a new user with a unique name. Check for duplicates and report errors if found. User should have a Bash login shell and belong to the new group. | `id -u`, `useradd -m -s /bin/bash -g <groupname>`, `$?` |
| **3. Create Password for User** | Set a password for the newly created user. | `passwd`, \`echo "username:password" |
| **4. Ensure User is Member of Group** | Verify that the new user is correctly assigned to the new group. (This is typically handled by `useradd -g`). | `groups <username>` (for verification) |
| **5. Create User-Named Directory at `/`** | Create a directory at the root (`/`) of the file system, named after the user. | `mkdir`, `$?` |
| **6. Set Directory Ownership** | Assign ownership of the new directory to the created user and group. | `chown <user>:<group>`, `$?` |
| **7. Set Directory Permissions (Owner & Group)** | Grant full control (read, write, execute) to both the owner and the group for the new directory. | `chmod 770`, `$?` |
| **8. Set Sticky Bit on Directory** | Ensure that only the owner of a file can delete it from the directory, even if others have write permissions. | `chmod +t`, `$?` |
| **9. Script Executability** | The script file itself must be made executable. | `chmod +x <script_name.sh>` (performed manually once after script creation) |
| **Accept Any Username/Group Name** | The script should use variables for user and group names, not hardcoded values. | Bash variables, `read` command for input. |
| **Error Checking** | Implement checks for the success of each command using the exit status. | `$?` (checks the exit status of the most recently executed foreground command) |

---

## Script

```bash
#!/bin/bash
# Shebang: Specifies that this script should be executed with bash.

# --- Function to display error messages and exit ---
error_exit() {
    echo "ERROR: $1" >&2  # Print the error message to standard error (>&2).
    exit 1               # Exit the script with a non-zero status code (1) to indicate an error.
}

# --- Check if the script is run as root ---
# User management commands require root privileges.
if [[ $EUID -ne 0 ]]; then # Check if the Effective User ID (EUID) is not 0 (root).
   error_exit "This script must be run as root. Please use 'sudo'." # If not root, call error_exit and terminate.
fi

echo "--- User and Group Management Script ---" # Informative header for the script output.

# --- Prompt for Group Name ---
read -rp "Enter the new group name: " GROUP_NAME # Prompt the user and read the input into GROUP_NAME.

# --- Validate Group Name ---
if [[ -z "$GROUP_NAME" ]]; then # Check if the GROUP_NAME variable is empty (-z checks for empty string).
    error_exit "Group name cannot be empty." # If empty, report an error.
fi

# --- Check for Duplicate Group Name ---
# 'getent group "$GROUP_NAME"' attempts to retrieve information about the group.
# '&>/dev/null' redirects both standard output and standard error to /dev/null, suppressing command output.
# If 'getent' finds the group, its exit status is 0 (success).
if getent group "$GROUP_NAME" &>/dev/null; then
    error_exit "Group '$GROUP_NAME' already exists. Please choose a unique group name." # If group exists, report error.
fi

# --- Create New Group ---
echo "Creating group '$GROUP_NAME'..." # Inform the user about group creation.
groupadd "$GROUP_NAME" # Attempt to create the group.
if [[ $? -ne 0 ]]; then # Check the exit status of the *previous* command (groupadd). $? stores the exit status.
# Correction: The original script had a redundant 'groupadd "$GROUP_NAME"' in the if condition,
# which would attempt to create the group twice or cause a syntax error depending on strictness.
# Checking '$?' after the command is the correct way.
    error_exit "Failed to create group '$GROUP_NAME'." # If groupadd failed, report error.
fi
echo "Group '$GROUP_NAME' created successfully." # Confirm group creation.

# --- Prompt for User Name ---
read -rp "Enter the new user name: " USER_NAME # Prompt the user and read the input into USER_NAME.

# --- Validate User Name ---
if [[ -z "$USER_NAME" ]]; then # Check if the USER_NAME variable is empty.
    error_exit "User name cannot be empty." # If empty, report an error.
fi

# --- Check for Duplicate User Name ---
# 'id -u "$USER_NAME"' attempts to get the user ID.
# '&>/dev/null' suppresses command output.
# If 'id -u' finds the user, its exit status is 0.
if id -u "$USER_NAME" &>/dev/null; then
    error_exit "User '$USER_NAME' already exists. Please choose a unique user name." # If user exists, report error.
fi

# --- Create New User ---
echo "Creating user '$USER_NAME' and adding to group '$GROUP_NAME'..." # Inform the user about user creation.
# 'useradd': command to create a new user.
# '-m': Creates the user's home directory if it doesn't exist.
# '-s /bin/bash': Sets Bash as the user's default login shell.
# '-g "$GROUP_NAME"': Assigns the newly created group as the user's primary group.
useradd -m -s /bin/bash -g "$GROUP_NAME" "$USER_NAME" # Attempt to create the user.
if [[ $? -ne 0 ]]; then # Check the exit status of the 'useradd' command.
# Correction: The original script had a redundant 'useradd "$USER_NAME"' in the if condition.
    error_exit "Failed to create user '$USER_NAME'." # If useradd failed, report error.
fi
echo "User '$USER_NAME' created successfully." # Confirm user creation.

# --- Set Password for the New User ---
echo "Setting password for user '$USER_NAME'..." # Inform the user about password setting.
passwd "$USER_NAME" # Prompts for and sets the password for the new user interactively.
if [[ $? -ne 0 ]]; then # Check the exit status of the 'passwd' command.
# Correction: The original script had 'passwd "USER_NAME"' which uses a literal string, not the variable.
# It should check the exit status of 'passwd "$USER_NAME"'.
    error_exit "Failed to set password for user '$USER_NAME'. You might need to set it manually." # If passwd failed, report error.
fi
echo "Password set successfully for user '$USER_NAME'." # Confirm password set.

# --- Verify User is Member of Group (Optional but good practice) ---
# 'id -nG "$USER_NAME"': Lists all groups (names only) the user belongs to.
# 'grep -qw "$GROUP_NAME"': Searches for the exact group name quietly (-q) and as a whole word (-w).
# '!': Inverts the exit status, so the 'if' block runs if grep *fails* to find the group (i.e., user is NOT a member).
if ! id -nG "$USER_NAME" | grep -qw "$GROUP_NAME"; then
    echo "Warning: User '$USER_NAME' does not appear to be a member of group '$GROUP_NAME'." # Warn if not a member.
    echo "Attempting to add user to group (if not already done)..." # Inform about attempting to add.
    # 'usermod -aG "$GROUP_NAME" "$USER_NAME"': Adds the user to the specified supplementary group.
    # '-a': Appends the user to the group.
    # '-G': Specifies the supplementary group(s).
    usermod -aG "$GROUP_NAME" "$USER_NAME" # Attempt to add user to the group.
    if [[ $? -ne 0 ]]; then # Check the exit status of the 'usermod' command.
    # Correction: The original script had 'usermod "GROUP_NAME"' which is incorrect.
        error_exit "Failed to add user '$USER_NAME' to group '$GROUP_NAME'." # If usermod failed, report error.
    fi
fi
echo "User '$USER_NAME' is a member of group '$GROUP_NAME'." # Confirm user is in the group.

# --- Create User-Named Directory at Root ---
DIRECTORY_PATH="/$USER_NAME" # Define the path for the new directory.
echo "Creating directory '$DIRECTORY_PATH'..." # Inform the user about directory creation.
# 'mkdir -p "$DIRECTORY_PATH"': Creates the directory.
# '-p': Creates parent directories if they don't exist and doesn't report an error if the directory already exists.
mkdir -p "$DIRECTORY_PATH"
if [[ $? -ne 0 ]]; then # Check the exit status of the 'mkdir' command.
# Correction: The original script had 'mkdir -p "DIRECTORY_PATH"' which uses a literal string, not the variable.
    error_exit "Failed to create directory '$DIRECTORY_PATH'." # If mkdir failed, report error.
fi
echo "Directory '$DIRECTORY_PATH' created successfully." # Confirm directory creation.

# --- Set Ownership of the Directory ---
echo "Setting ownership of '$DIRECTORY_PATH' to '$USER_NAME:$GROUP_NAME'..." # Inform about ownership change.
# 'chown "$USER_NAME":"$GROUP_NAME" "$DIRECTORY_PATH"': Changes ownership to the specified user and group.
chown "$USER_NAME":"$GROUP_NAME" "$DIRECTORY_PATH"
if [[ $? -ne 0 ]]; then # Check the exit status of the 'chown' command.
# Correction: The original script had 'chown "USER_NAME"' which is incorrect.
    error_exit "Failed to set ownership for '$DIRECTORY_PATH'." # If chown failed, report error.
fi
echo "Ownership set successfully." # Confirm ownership change.

# --- Set Permissions of the Directory ---
echo "Setting permissions for '$DIRECTORY_PATH' to 770 (rwx for owner and group)..." # Inform about permissions.
# 'chmod 770 "$DIRECTORY_PATH"': Sets permissions.
# '770': Read, write, execute for owner (7); read, write, execute for group (7); no permissions for others (0).
chmod 770 "$DIRECTORY_PATH"
if [[ $? -ne 0 ]]; then # Check the exit status of the 'chmod' command.
# Correction: The original script had 'chmod 770 "DIRECTORY_PATH"' which uses a literal string.
    error_exit "Failed to set permissions for '$DIRECTORY_PATH'." # If chmod failed, report error.
fi
echo "Permissions set successfully to 770." # Confirm permissions set.

# --- Set Sticky Bit on the Directory ---
echo "Setting sticky bit on '$DIRECTORY_PATH'..." # Inform about sticky bit.
# 'chmod +t "$DIRECTORY_PATH"': Sets the sticky bit.
# The sticky bit on a directory means that only the owner of a file (or the directory owner/root)
# can rename or delete files within that directory, even if others have write permissions to the directory.
chmod +t "$DIRECTORY_PATH"
if [[ $? -ne 0 ]]; then # Check the exit status of the 'chmod' command.
# Correction: The original script had 'chmod +t "DIRECTORY_PATH"' which uses a literal string.
    error_exit "Failed to set sticky bit for '$DIRECTORY_PATH'." # If chmod failed, report error.
fi
echo "Sticky bit set successfully on '$DIRECTORY_PATH'." # Confirm sticky bit set.

echo "--- User and group management complete for '$USER_NAME' and '$GROUP_NAME'. ---" # Final success message.

```

### Script without comments

```bash
#!/bin/bash

error_exit() {
    echo "ERROR: $1" >&2
    exit 1
}

if [[ $EUID -ne 0 ]]; then
   error_exit "This script must be run as root. Please use 'sudo'."
fi

echo "--- User and Group Management Script ---"

read -rp "Enter the new group name: " GROUP_NAME

if [[ -z "$GROUP_NAME" ]]; then
    error_exit "Group name cannot be empty."
fi

if getent group "$GROUP_NAME" &>/dev/null; then
    error_exit "Group '$GROUP_NAME' already exists. Please choose a unique group name."
fi

echo "Creating group '$GROUP_NAME'..."
groupadd "$GROUP_NAME"
if [[ $? -ne 0 ]]; then
    error_exit "Failed to create group '$GROUP_NAME'."
fi
echo "Group '$GROUP_NAME' created successfully."

read -rp "Enter the new user name: " USER_NAME

if [[ -z "$USER_NAME" ]]; then
    error_exit "User name cannot be empty."
fi

if id -u "$USER_NAME" &>/dev/null; then
    error_exit "User '$USER_NAME' already exists. Please choose a unique user name."
fi

echo "Creating user '$USER_NAME' and adding to group '$GROUP_NAME'..."
useradd -m -s /bin/bash -g "$GROUP_NAME" "$USER_NAME"
if [[ $? -ne 0 ]]; then
    error_exit "Failed to create user '$USER_NAME'."
fi
echo "User '$USER_NAME' created successfully."

echo "Setting password for user '$USER_NAME'..."
passwd "$USER_NAME"
if [[ $? -ne 0 ]]; then
    error_exit "Failed to set password for user '$USER_NAME'. You might need to set it manually."
fi
echo "Password set successfully for user '$USER_NAME'."

if ! id -nG "$USER_NAME" | grep -qw "$GROUP_NAME"; then
    echo "Warning: User '$USER_NAME' does not appear to be a member of group '$GROUP_NAME'."
    echo "Attempting to add user to group (if not already done)..."
    usermod -aG "$GROUP_NAME" "$USER_NAME"
    if [[ $? -ne 0 ]]; then
        error_exit "Failed to add user '$USER_NAME' to group '$GROUP_NAME'."
    fi
fi
echo "User '$USER_NAME' is a member of group '$GROUP_NAME'."

DIRECTORY_PATH="/$USER_NAME"
echo "Creating directory '$DIRECTORY_PATH'..."
mkdir -p "$DIRECTORY_PATH"
if [[ $? -ne 0 ]]; then
    error_exit "Failed to create directory '$DIRECTORY_PATH'."
fi
echo "Directory '$DIRECTORY_PATH' created successfully."

echo "Setting ownership of '$DIRECTORY_PATH' to '$USER_NAME:$GROUP_NAME'..."
chown "$USER_NAME":"$GROUP_NAME" "$DIRECTORY_PATH"
if [[ $? -ne 0 ]]; then
    error_exit "Failed to set ownership for '$DIRECTORY_PATH'."
fi
echo "Ownership set successfully."

echo "Setting permissions for '$DIRECTORY_PATH' to 770 (rwx for owner and group)..."
chmod 770 "$DIRECTORY_PATH"
if [[ $? -ne 0 ]]; then
    error_exit "Failed to set permissions for '$DIRECTORY_PATH'."
fi
echo "Permissions set successfully to 770."

echo "Setting sticky bit on '$DIRECTORY_PATH'..."
chmod +t "$DIRECTORY_PATH"
if [[ $? -ne 0 ]]; then
    error_exit "Failed to set sticky bit for '$DIRECTORY_PATH'."
fi
echo "Sticky bit set successfully on '$DIRECTORY_PATH'."

echo "--- User and group management complete for '$USER_NAME' and '$GROUP_NAME'. ---"


```

## User Management Bash Script

Remember to save this as a `.sh` file (e.g., `user_manager.sh`) and make it executable using `chmod +x user_manager.sh` before running it.

This script is designed to be robust by checking for existing users and groups, handling potential errors, and setting the correct permissions and ownership.

To execute the script you would perform the following steps in a terminal:

1. **Save the script:** Copy the code above and save it to a file, e.g., `user_manager.sh`.
2. **Make it executable:**

```bash

chmod +x user_manager.sh

```

3. **Run with new unique user/group:**

```bash

sudo ./user_manager.sh 

```

(Then follow the prompts to enter a new unique user and group name, and set the password.)

4. **Show the error for duplicate user/group:**

```bash

sudo ./user_manager.sh

```

(Then enter a user or group name that you just created in the previous step, and observe the error message.)
