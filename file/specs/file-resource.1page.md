# File Resource

## Overview

The file resource is used to manage the configuration file system objects and their content on Windows, Linux, and macOS systems.

This resource can get and test file system paths for directories, files and symbolic links. It can be used to verify the path exists, the file object type, and the resolved path of the file object.

In addition, the file resource can also be used to test the content of a files and symbolic links to files. It can be used to verify the file content contains (or _does not_ contain) a specific value, or matches a regular expression. It can also validate a file exactly matches specific content. For matching operations, it can verify the expected number of matches. For regular expression matches, it can verify the content of the capture groups of the matches.

## Examples

In the following examples, assume the following file system structure:

```text
root
├── other
│   └── target.file
└── parent
    └── child
        ├── example1.file
        └── other.file -> root/other/target.file   
```

Furthermore, assume the file `example1.file` contains the following content:

```text
All good dogs go to heaven.
There are no bad dogs.
```

Finally, assume the file `target.file` contains the following content:

```text
Hello, World!
```

### Example 1

This snippet shows how you can get a specific file object, check if it does or doesn't exist, and get the current properties. By default, the _well-known_ `_exist` property has a default of `true`.

```shell
@"
{
  'path' :'root/parent/child/example1.file'
}
"@ | dsc get -r microsoft/file
```

The result shows the actual state and includes all the properties of the file object.

```json
{
  "actualState": {
    "_exists": true,
    "path": "root/parent/child/example1.file",
    "type": "file",
    "resolvedPath": "root/parent/child/example1.file"
  }
}
```

### Example 2

This snippet shows how you can get a specific file object of a specific type, if it exists.

```shell
@"
{
  'path': 'root/parent/../other',
  'type': 'directory'
}
"@ | dsc get -r microsoft/file
```

The result shows the actual state. The input path is a relative directory, the actual state shows the input path and the resolved path.

```json
{
  "actualState": {
    "_exists": true,
    "path": "root/parent/../other",
    "type": "directory",
    "resolvedPath": "root/other"
  }
}
```

### Example 3

This example asserts that the file object does not exist.

```shell
@"
{
  '_exist': false,
  'path': 'root/parent/child/other.file',
  'type': 'file'
}
"@ | dsc get -r microsoft/file
```

The actual state shows the file exists, is a symlink, and gives the resolve link path.

```json
{
  "actualState": {
    "_exists": true,
    "path": "root/parent/child/other.file",
    "type": "symlink",
    "resolvedPath": "root/other/target.file"
  }
}
```

### Example 4

This snippet shows how to use the `like` operation to find text within the file content. This examples uses the default value of `true` for the well-known `_exist` property.

```shell
@"
{
  'path' :'root/parent/child/example1.file',
  'find': [
    {
      'like': 'good'
    }
  ]
}
"@ | dsc get -r microsoft/file
```

The result shows the actual state and includes all the properties of the file object, and the find operation result.

```json
{
  "actualState": {
    "_exists": true,
    "path": "root/parent/child/example1.file",
    "type": "file",
    "resolvedPath": "root/parent/child/example1.file",
    "find": [
      {
        "like": "good",
        "count": 1
      }
    ]
  }
}
```

### Example 5

This example asserts the text **dogs** occurs 2 times.

```shell
@"
{
  'path' :'root/parent/child/example1.file',
  'find': [
    {
      'like': 'dogs',
      'count': 2
    }
  ]
}
"@ | dsc get -r microsoft/file
```

The result shows the actual state matches the assertion.

```json
{
  "actualState": {
    "_exists": true,
    "path": "root/parent/child/example1.file",
    "type": "file",
    "resolvedPath": "root/parent/child/example1.file",
    "find": [
      {
        "like": "dogs",
        "count": 2
      }
    ]
  }
}
```

### Example 6

This shows how to use the `content` operation to asserts a file contains exact content.

```shell
@"
{
  'path' :'root/parent/child/other.file',
  'find': [
    {
      'content': 'Hello',
    }
  ]
}
"@ | dsc get -r microsoft/file
```

The result shows the actual state does not matches the assertion.

```json
{
  "actualState": {
    "_exists": true,
    "path": "root/parent/child/other.file",
    "type": "symlink",
    "resolvedPath": "root/other/target.file",
    "find": [
      {
        "content": "Hello, World!"
      }
    ]
  }
}
```

Support for regular expression matching will be added in a future release. The plan is to support ECMA 2018 (JavaScript) regular expression syntax.

By default, regular expressions will not support backtrace, as this is an expensive operation. The `--es2018` option can be used to enable full ES2018 regular expression syntax, including backtrace. _This will change performance from linear to non-linear time._

In addition to specifying the regex for the search, support for the following options will get added over time:

- `ignoreCase` If `true` then the search is case insensitive.
- `global` If `true` then the search will find all matches; otherwise, only the first match is found.
- `multiline` If `true` then `^` and `$` will match the start and end of a line, respectively. If `false` then `^` and `$` will match the start and end of the file, respectively.

Also, regular expression capture groups could be added eventually.