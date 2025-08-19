# User management

<!-- toc -->

## Case Scenario  

As the Linux Administrator for fast-growing company, you have been tasked with creating, modifying, and removing user accounts from the Linux server. The company has just hired 9 new employees to fill 3 newly designed departments. The departments that have been created are Engineering, Sales and IS. The server must be setup with the appropriate files, folders, users, groups and permissions to ensure a successful launch of the newly designed departments.

## Setting Up the Departments 💻

To set up the new departments, we'll follow a series of steps to create the necessary directories, users, and groups, and then configure the permissions to meet the specified security requirements.

| Department | Administrator User | Additional Users | Group | Directory |
| :--- | :--- | :--- | :--- | :--- |
| **Engineering** | `engadmin` | `enguser1`, `enguser2` | `engineering` | `/engineering` |
| **Sales** | `salsadmin` | `saluser1`, `saluser2` | `sales` | `/sales` |
| **IS** | `isadmin` | `isuser1`, `isuser2` | `is` | `/is` |

***

### Step 1: Create Directories and Groups

First, we need to create the directories and groups for each department.

**Commands:**

* `sudo mkdir /engineering /sales /is` (Creates the department directories at the root level).
* `sudo groupadd engineering` (Creates the `engineering` group).
* `sudo groupadd sales` (Creates the `sales` group).
* `sudo groupadd is` (Creates the `is` group).
* `sudo groupdel 'group'` (Removes group).

***

### Step 2: Create Users and Passwords

Next, we'll create the administrative and normal users for each department, ensuring they have a Bash login shell and are assigned to their respective primary groups.

**Commands:**  
`-m` create the user's home directory.  
`-g` name or ID of the primary group of the new account(group).  
Use `-mg` for both options.

* `sudo useradd -m -g engineering -s /bin/bash engadmin` (Creates the `engadmin` user with the primary group `engineering` and a home directory).
* `sudo useradd -m -g engineering -s /bin/bash enguser1`
* `sudo useradd -m -g engineering -s /bin/bash enguser2`
* `sudo useradd -m -g sales -s /bin/bash salsadmin` (Creates the `salsadmin` user with the primary group `sales` and a home directory).
* `sudo useradd -m -g sales -s /bin/bash saluser1`
* `sudo useradd -m -g sales -s /bin/bash saluser2`
* `sudo useradd -m -g is -s /bin/bash isadmin` (Creates the `isadmin` user with the primary group `is` and a home directory).
* `sudo useradd -m -g is -s /bin/bash isuser1`
* `sudo useradd -m -g is -s /bin/bash isuser2`
* `sudo userdel 'user'` (Removes user).

<br>

* `sudo passwd engadmin`
* `sudo passwd enguser1`
* `sudo passwd enguser2`
* `sudo passwd salsadmin`
* `sudo passwd saluser1`
* `sudo passwd saluser2`
* `sudo passwd isadmin`
* `sudo passwd isuser1`
* `sudo passwd isuser2`

After you run each command, the terminal will prompt you to input and confirm the new password for the specified user.

### Password Security Best Practices

When setting passwords, it's a good practice to enforce security policies to protect the system. Linux systems have a variety of tools to manage password policies, such as:

* **Password aging:** Forcing users to change their passwords after a certain period.
* **Complexity requirements:** Requiring passwords to meet specific criteria, like a minimum length and the inclusion of special characters.
* **Locking accounts:** Automatically locking accounts after a certain number of failed login attempts.

These policies can be configured in files like `/etc/login.defs` and `/etc/pam.d/common-password` to enhance overall system security.
***

### Step 3: Configure Directory Ownership and Permissions

This step ensures that the department directories have the correct ownership and permissions. We will set the department administrator as the owner, the department group as the group owner, and apply special permissions.

* `sudo chown engadmin:engineering /engineering` (Changes the ownership of the `/engineering` directory).
* `sudo chown salsadmin:sales /sales` (Changes the ownership of the `/sales` directory).
* `sudo chown isadmin:is /is` (Changes the ownership of the `/is` directory).

`engadmin:engineering`: This is the new ownership specification.

`engadmin`: The name before the colon is the new **user owner**. All files and directories have a user owner, and in this case, the /engineering directory's owner is being set to the engadmin user.

`:engineering`: The name after the colon specifies the new **group owner**. All files and directories also have a group owner. By setting the group owner to engineering, all users who are members of the engineering group will inherit the group's permissions for this directory.

`/engineering`: This is the target directory. This command will apply the ownership changes to the /engineering directory.

Now, we will set the permissions. The **`chmod`** command is used to set the permissions of files and directories. Permissions can be represented in either symbolic or octal (numeric) form. The octal representation is often clearer for complex permissions. A three-digit octal code represents the permissions for the owner, the group, and others, respectively. The octal values are derived from the sum of the permissions: read (`r`) is 4, write (`w`) is 2, and execute (`x`) is 1.

For example, `7` represents `rwx` (4+2+1), `5` represents `r-x` (4+0+1), and `6` represents `rw-` (4+2+0). The fourth octal digit, if present, is for special permissions.

To meet the requirements, we'll use a combination of standard and special permissions. The **sticky bit** (`t` or `1` in the first octal digit) is a special permission that, when applied to a directory, prevents users from deleting files they don't own, even if they have write permissions to the directory. This is exactly what the prompt requires. The first `2` in our command will set the **SGID** (Set Group ID) bit, which ensures that any new file created in the directory will inherit the group ownership of the directory (`engineering`, `sales`, or `is`).

**Commands:**

* `sudo chmod 2770 /engineering`
  * **2**: Sets the **SGID** bit. This means any new file or subdirectory created within `/engineering` will automatically belong to the `engineering` group.
  * **7**: `rwx` (read, write, execute) for the **owner** (`engadmin`).
  * **7**: `rwx` (read, write, execute) for the **group** (`engineering`).
  * **0**: No permissions for **others**.
* `sudo chmod 2770 /sales`
* `sudo chmod 2770 /is`
* `sudo chmod +t /engineering` (Adds the sticky bit to the `/engineering` directory, allowing only file owners to delete their own files).
* `sudo chmod +t /sales`
* `sudo chmod +t /is`

### Sticky bit, SUID & SGID  

Here's a breakdown of the differences between `1770`, `2770`, and `4770` permissions, with a focus on their special bits.

The numbers `1`, `2`, and `4` in the first position of the octal permission code represent special permission bits that affect how a file or directory behaves. The `770` part always means that the **owner** and the **group** have full read, write, and execute permissions (`rwx`), while **others** have no permissions (`---`).

***

### Special Permission Bit Differences

| Octal Code | Special Bit | Meaning and Impact |
| :--- | :--- | :--- |
| **`1770`** | **Sticky Bit (1)** 📌 | **Applies to Directories:** When set on a directory, users can create files and subdirectories within it, but they can **only delete or rename files they own** (or files owned by the directory owner). This is crucial for shared directories like `/tmp` to prevent users from deleting others' content. |
| **`2770`** | **Set Group ID (SGID) Bit (2)** 👥 | **Applies to Directories:** When set on a directory, any new files or subdirectories created within it will **inherit the group ownership of the parent directory**, rather than the primary group of the user who created them. This is very useful for collaborative projects where all files should belong to a specific project group.  <br> **Applies to Executable Files:** When set on an executable file, the process will run with the **effective group ID of the file's group owner**, not the primary group of the user executing it. |
| **`4770`** | **Set User ID (SUID) Bit (4)** 👤 | **Applies to Executable Files:** When set on an executable file, the process will run with the **effective user ID of the file's owner**, not the user who is executing it. A classic example is the `passwd` command, which allows a non-root user to change their password by temporarily assuming the root user's permissions to modify system files.  <br> **Applies to Directories:** The SUID bit on a directory is generally ignored by most modern Linux systems and doesn't have a standard or widely recognized behavior. |

***

In all these cases, the `770` indicates `rwx` (read, write, execute) for the user owner, `rwx` for the group owner, and `---` (no permissions) for others. The first digit is what changes the fundamental behavior regarding ownership or deletion within a directory, or execution privileges for a file.

***

### Step 4: Create Confidential Documents

Now we will create the required documents in each department's directory and set the appropriate ownership and permissions.

**Commands:**

* `sudo su engadmin -c "echo 'This file contains confidential information for the engineering department.' > /engineering/confidential.txt"` (Creates the file and writes the text as `engadmin`).
* `sudo su salsadmin -c "echo 'This file contains confidential information for the sales department.' > /sales/confidential.txt"` (Creates the file as `salsadmin`).
* `sudo su isadmin -c "echo 'This file contains confidential information for the IS department.' > /is/confidential.txt"` (Creates the file as `isadmin`).

<br>

`-c`: This option tells su to execute a single command (the one that follows) instead of starting a new interactive shell.

`"..."`: The double quotes are crucial. They enclose the command string that you want to execute as the engadmin user. This ensures that the entire command, including any pipes or redirections, is treated as a single unit and executed in the context of the new user's shell.

**Modify file permissions:**

* `sudo su engadmin -c "chmod 640 /engineering/confidential.txt"`
  * **6**: `rw-` (read and write) for the **owner** (`engadmin`).
  * **4**: `r--` (read-only) for the **group** (`engineering`).
  * **0**: No permissions for **others**.
* `sudo su salsadmin -c "chmod 640 /sales/confidential.txt"`
* `sudo su isadmin -c "chmod 640 /is/confidential.txt"`

***

### Deliverables: Verification Commands ✅

#### Verify User and Group Creation

To verify that the users and groups have been created, you can check the `/etc/passwd` and `/etc/group` files, respectively.

* `cat /etc/passwd | grep -E "engadmin|enguser1|enguser2|salsadmin|saluser1|saluser2|isadmin|isuser1|isuser2"`
  * This command searches the `/etc/passwd` file for the usernames we created. The output confirms their existence and provides details like their UID, GID, home directory, and shell.
* `cat /etc/group | grep -E "engineering|sales|is"`
  * This command searches the `/etc/group` file for the group names. The output confirms the groups exist and shows which users are members.

* `getent passwd` Displays a list of all users.
* `getent passwd isadmin isuser1 isuser2` Displays users if found.
* `-E` Enables extended regular expressions.

#### Verify User’s Group Assignment

To check a specific user's group assignments, you can use the **`id`** command.

* `id engadmin`
  * **Sample Output:** `uid=1001(engadmin) gid=1001(engineering) groups=1001(engineering)`
  * This shows that `engadmin`'s primary group (`gid`) is `engineering`.
* `id saluser1`
* `id isuser2`
  * You can run this command for each user to confirm their primary group.

#### Verify Directory Creation and Permissions

The **`ls -ld`** command is essential for checking directory permissions, ownership, and other details.

* `ls -ld /engineering /sales /is`
  * **Sample Output for `/engineering`**: `drwxrwsr-T 2 engadmin engineering 4096 Aug 19 12:00 /engineering`
  * **`d`**: Confirms it's a directory.
  * **`rwxrwsr-T`**:
    * `rwx`: Owner (`engadmin`) has read, write, and execute permissions.
    * `rws`: The `s` confirms the **SGID** bit is set, and the group has read, write, and execute permissions.
    * `r-T`: The `T` confirms the **sticky bit** is set for others' permissions.
  * **`engadmin engineering`**: Confirms the correct owner and group ownership.

#### Verify File Creation and Permissions

To verify the creation and permissions of the confidential documents, use the **`ls -l`** command.

* `sudo su engadmin -c "ls -l /engineering/confidential.txt"`
  * **Sample Output**: `-rw-r----- 1 engadmin engineering 62 Aug 19 12:05 /engineering/confidential_doc`
  * **`-rw-r-----`**: Confirms the permissions.
    * `rw-`: Owner (`engadmin`) has read and write access.
    * `r--`: Group (`engineering`) has read-only access.
    * `---`: Others have no permissions.
  * **`engadmin engineering`**: Confirms the correct ownership and group.
* `sudo su salsadmin -c "ls -l /sales/confidential_doc"`
* `sudo su isadmin -c "ls -l /is/confidential_doc"`

### Cleanup  

```bash

sudo userdel -r engadmin enguser1 enguser2 && \
sudo userdel -r salsadmin saluser1 saluser2 && \
sudo userdel -r isadmin isuser1 isuser2 && \
sudo groupdel engineering && \
sudo groupdel sales && \
sudo groupdel is && \
sudo rm -r /engineering && \
sudo rm -r /sales && \
sudo rm -r /is

```
