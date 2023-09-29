# File Resource

## Overview

The file resource allows you to manage files on Windows, Linux, and macOS systems.

## Parameters

`--destination-path` The path to a file object that you want to enure is either **Present** or **Absent**.

`--source-path` If specified the path that is the source file object to copy.

`--content` The content that you want to enure is either **Present** or **Absent** within the file object. This parameter is only valid if the file object is a file. If the file object is a directory, then this parameter is ignored. If the file object is a link to a file, and `--traverse-links` is true, the the content of the link target is validated; otherwise, this parameter is ignored.

> We should support either POSIX or JavaScript regular expressions. (I'm not even sure they are not the same.)

> We should support multiple encodings. In addition to UTF-8, UTF-16, UTF-32, ASCII, etc, we should support binary in the form of Base64 encoded strings.

`--top-directory` Actions can only be performed on file objects that are within the top directory. Any path that resolves to a value outside of the top directory results in an error. Specify this parameter to prevent any accidental changes outside of the top directory. This is an optional parameter; on Linux and macOS, this defaults to the `/` root directory, and on Windows, this defaults to the root of the `%SystemDrive%`, e.g., `C:\`.

> Should there be separate destination and source top directories?

> We should support the use of `{{top-directory}}` for substitutions within the configuration.

`--recursive` If true, and the file object is a directory, then the action is performed recursively on all files and directories within the directory.

> How should we handle recursion of links when `--traverse-links` is true? Even with the `--preserve-top-directory` and `--no-preserve-top-directory` parameters, resolving and recursively processing links is a potential problem. Recommend that `--traverse-links`  is only applied to the `--destination-path` and `--source-path`, but not to the recursively discovered children of these file objects. For example, if the source of a copy is a link to a directory, which itself contains links to other directories, and traverse links is true, would the user expect a shallow copy of the resolved source, or a deep copy of all resolved file objects?

`--traverse-links` If true, traverse links and act on the target of the link; otherwise, the act on the link object directly. Default: false.

The absolute path for links is always resolved to determine if the link resolves to the top directory. Changes to the top directory and recursion of the top directory, either directly or through a link can be fine tuned using the `--preserve-top-directory` and `--no-preserve-top-directory` parameters.

`--type` The type of the file object, one of: `file`, `directory`, `symbolic-link`.

`--owner` The name of the file object owner.

`--preserve-top-directory` If true, and the file object resolves to the top-directory directory, recursive actions will fail. Default: false.

`--no-preserve-top-directory` If false, the action will fail if the path is the root directory. Default: true.

### Linux and macOS only parameters

The following parameters are only supported on Linux and macOS.

`--type` In addition to the basic file object types, Linux and macOS also supports the `hard-link` type.

`--owner` In addition to using a user name, the owner can be the unique user identifier.

`--group` The name or unique user identifier of the group that owns the file object.

`--permissions` Specify the file object permissions. Permissions can be specified in either a symbolic, e.g., `u=rw,g=r,o=r`, or octal, e.g., `644` format.

The format of a symbolic mode is [**ugoa**...][[**+-=**][_perms_...]...], where _perms_ is either zero or more letters from the set **rwx**, or a single letter from the set **ugo**. Multiple symbolic modes can be given, separated by commas.

A combination of the letters **ugoa** controls which users' access to the file will be changed: the user who owns it (**u**), other users in the file's group (**g**), other users not in the file's group (**o**), or all users (**a**). If none of these are given, the effect is as if **a** were given, but bits that are set in the umask are not affected.

The operator **+** causes the selected file mode bits to be added to the existing file mode bits of each file; **-** causes them to be removed; and **=** causes them to be added and causes unmentioned bits to be removed.

The letters **rwx** select file mode bits for the affected users: read (**r**), write (**w**), and execute (**x**) (or search for directories). Instead of one or more of these letters, you can specify exactly one of the letters **ugo**: the permissions granted to the user who owns the file (**u**), the permissions granted to other users who are members of the file's group (**g**), and the permissions granted to users that are in neither of the two preceding categories (**o**).

A numeric mode is from one to four octal digits (0-7), derived by adding up the bits with values 4, 2, and 1. Omitted digits are assumed to be leading zeros. The first digit selects the set user ID (4) and set group ID (2) and restricted deletion or sticky (1) attributes. The second digit selects permissions for the user who owns the file: read (4), write (2), and execute (1); the third selects permissions for other users in the file's group, with the same values; and the fourth for other users not in the file's group, with the same values.

If the file object is a symbolic link or a hard link, and `--traverse-links` is true, then the permissions are applied to the link target. If `--traverse-links` s false, then no action is taken. Links do not support permissions. All action results to permissions on links will return the requested values. Any TEST action on link permissions will always succeed.

> We will not support `setuid`, `setgid`, or `seteuid` with this parameter at this time. This can be revisited if there is a customer demand. The `--copy-permissions` argument provides a workaround for most use cases.

> We will not support the `X` bit, for execute/search only if the file is a directory or already has execute permission for some user. This flag sets the execute/search on **ugo** if and only if the execute/search bit is already set for one of user, group or other. This can be revisited if there is a customer demand.

`--stick` Specify the sticky bit (restricted deletion bit) of the file object. If the file object type is a symbolic link, and `--traverse-links` is true, then the sticky bit is applied to the link target; otherwise, no action is taken and the result always succeeds.

`--copy-permissions` This specifies the path to a file or directory to copy the file mode permissions from. This will copy all file mode permissions of the source, including the set-user-ID, set-group-ID, and sticky bit. The `--copy-permissions` behavior on links is the same as the `--permissions` behavior.

### Windows only parameters

The following parameters are only supported on Windows.

`--type` In addition to the basic file object types, Windows also supports the `junction-point`

`--owner` On windows, the `--owner` can be a comma separated list of one or more users and groups. Users and groups can be specified by name, or by unique identifier. If the user or group is not found, the action will fail.

`--attributes` The desired state of the attributes for the targeted file object. Valid values are one or more of `archive`, `hidden`, `readonly`, and `system`. The values are case-insensitive. The values are comma separated. The values are additive, so if you specify `--attributes hidden,system`, the file object will be hidden and a system file. If the file object type is a symbolic link, and `--traverse-links` is true, then the attributes are applied to the link target; otherwise, the attributes are applied to the link.

> How does Windows handle attributes on soft links?

> Are attributes applied to a junction point equivalent to setting the attribute on the inode, in which case, if `--traverse-links`` is false and the target is a junction point, and the operation is SET, then the action should fail?

[//]: # (cSpell:ignore setuid, setgid, seteuid,)
