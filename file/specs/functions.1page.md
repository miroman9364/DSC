# Functions

## Overview

This document proposes an expression syntax and a set of function that can be used in resource configurations. This syntax is based on the [Azure Resource Manager template function expressions][ARM template syntax and expressions]. This syntax provides a way to extend JSON to support functions.

Resource providers can opt-in to support for this syntax. If a resource provider does not support this syntax, then the expression is treated as a string literal.

This document specifies some basic functions. Resource providers that support these functions, **must** support the syntax and behavior specified in this document. Resource providers can support additional functions, but they must be documented separately.

## Expression syntax basics

Complete details for the expression syntax are [defined by ARM][ARM template syntax and expressions]. This section provides a brief overview of the syntax.

Configuration expressions are an extension to JSON. Expressions start and end with brackets: `[` and `]`, respectively. The value of the expression is evaluated when the configuration operation is executed. An expression can return a string, integer, boolean, array, or object.

### Use functions

The following is an example of a configuration, where the `name` property value is defined by an expression that concatenates the `first` and `last` property values:

```json
{
  "first": "Alice",
  "last": "Smith",
  "name": "[concat(configuration().first, configuration().last)]"
}
```

Within the expression, there are a total of 3 function calls being made. The calls are to 2 different functions: one call to the `concat()` function, and two calls to the `configuration()` function. Both of these functions are part of the set of well-defined functions specified in this document.

Functions names are case-insensitive, function parameters are case-preserving. For example, `concat('John', ' ', 'Doe')` and `CONCAT('John', ' ', 'Doe')` are equivalent; both produce the string **`John Doe`**.

Functions that that provide access to the JSON path are case insensitive. For example, `configuration().myProperty` and `CONFIGURATION().MYPROPERTY` are equivalent.

Function parameters are case preserving, and the case sensitivity of parameters is determined by the function. For example, the `parameter()` function is case insensitive, the `environment()` function presumes case sensitive, and the `concat()` function is case agnostic.

### Passing string parameters

To pass a function a string parameter, quote the string using the opposite quote style used by the JSON value. For example, both of the following a valid ways of passing a string into a function:

```json
{
  "name": "[concat('John', ' ', 'Doe')]"
}
```

```json
{
  'name': '[concat("John", " ", "Doe")]'
}
```

Whereas the following two examples are invalid:

```json
{
  "name": "[concat("John", " ", "Doe")]"
}
```

```json
{
  'name': '[concat('John', ' ', 'Doe')]'
}
```

> TODO: Document escape sequences

## Well defined functions

The following functions are well defined. Resource providers that support these functions, **must** support the syntax and behavior specified in this document. Resource providers can support additional functions, but they must be documented separately.

### `configuration()`

```json
configuration()
```

The `configuration()` function returns the configuration object. The configuration object is the object that contains the resource configuration. The configuration object is the root object of the configuration. When using the configuration object to define configuration properties, only properties that are already defined can be referenced.

### `parameters()`

```json
parameters(parameterName)
```

The `parameters()` function returns the value of the specified parameter passed into the resource provider. The parameter name is case sensitive. The parameter name is the long name of the parameter. Dashes within the name of the parameter name can be omitted. For example, the `--top-directory` parameter can be referenced using `parameters('top-directory')`, `parameters('topDirectory')`, or `parameters('topdirectory')`.

### `osInfo()`

```json
osInfo()
```

The `osInfo()` function returns an object that contains information about the operating system. The object contains the properties returned by the `osinfo` resource. These include:

- id
- family
- version
- edition
- codename
- bitness
- architecture

### `environment()`

```json
environment(variableName)
```

Returns the value of the specified environment variable. The variable name is case sensitive on Linux and macOS, and case insensitive on Windows.

### ARM string functions

DSC reserves a number of function signatures for well-defined functions that are not currently specified. For example, most, if not the full set of [ARM template string functions] should eventually be supported for configurations.


[ARM template syntax and expressions]: https://learn.microsoft.com/en-us/azure/azure-resource-manager/templates/template-expressions

[ARM template string functions]: https://learn.microsoft.com/en-us/azure/azure-resource-manager/templates/template-functions#string-functions

[//]: # (cSpell:ignore bitness, osinfo,)
