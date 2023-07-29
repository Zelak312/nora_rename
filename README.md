[![Rust](https://github.com/Zelak312/nora_rename/actions/workflows/rust_build_test.yml/badge.svg?branch=dev)](https://github.com/Zelak312/nora_rename/actions/workflows/rust_build_test.yml)

# Documentation

-   [What is nora?](#what-is-nora)
-   [How to install](#how-to-install)
-   [Usage](#usage)
-   [Example usage](#example-usage)

# What is Nora?

Nora is a command line utility for renaming files in batch using regex with advanced control.
It is using a custom language created only for this purpose.
Any suggestion is appricated (using github issues)

# How to install

## Using cargo

```
cargo install nora_rename
```

Other methods comming soon

# Usage

Command line usage

```
nora [OPTIONS] <INPUT> <OUTPUT>
```

## Options

| commands                 | description                                         |
| ------------------------ | --------------------------------------------------- |
| `-h` \| `--help`         | Print help information                              |
| `-s` \| `--skip`         | Skip the renaming preview and directly rename files |
| `-V` \| `--version`      | Print version information                           |
| `-p` \| `--pretty_print` | Pretty print the output for easier reading          |
| `-g` \| `--global`       | Removes the global match from the captures          |

## Input

Input is a regex expression.
Capture groups can be used (see here for using them in the output)

Named Captrure groups also works (see here for using them in the output)

## Output

The output expression is a little language easy to use

Two concepts are important, the unvariable parts and the interpreted parts

The unvariable part won't change when renaming while anything in the interpreted blocks will be interpreted
considering this example:

```
[#1].txt
```

The unvariable part is the `.txt`
An interpreted block starts with `[` and ends with `]` in this case the interpreted block is `[#1]` which contains `#1`

The interpreted block can have the following expressions

-   [For loop](#for-loop)
-   [Ternary expression](#ternary-expression)
-   [Math expression](#math-expression)
-   [String operation](#string-operation)
-   [Identifiers](#identifiers)
-   [String conversion](#string-conversion)
-   [Number conversion](#number-conversion)

---

# For loop

Example:

```
[for x in 0.#cap_count { #x }]
```

This will go from 0 to `#cap_count` which is evaluated as the number of caputre

`#x` can be used to get the caputr group with the value of x

if x is 0 it will get `#0` (first caputre group) etc

Note: This will often be used with the -g option because otherwise the whole caputres will also be included which will messed up things

# Ternary expression

Example:

```
[#1 == 10 ? 1 : 2]
```

Dependant on the condition, if it's true 1 will be returned otherwise 2 will be returned

The condition operator can be any of these `==`, `!=`, `<`, `<=`, `>`, `>=`

## Ternary second parameter skip

If you you want to do a ternary as a normal if statement to write something if it's true but nothing if not
Example:

```
[#1 == 10 ? "something" : ""]
```

The skip operator can be used to simplify this
Example:

```
[#1 == 10 ?> "something"]
```

# Math expression

Example:

```
[10 + 20]
```

currently supported operations are `+`, `-`, `*`, `/`, `**`, `//`, `(`, `)`

`**`: Power opertaor
`//`: Log operator

It is important to note that math expressions will only be interpreted as mathematical expressions when the left paramter is a number

For example:

```
["10" + 20]
```

This will give 1020 since it will convert the left to a string and do a concatenation

```
[10 + "20"]
```

This will give 30 since the left is a number and it will convert to string to a number automatically

# String operation

## Concatenation

Example:

```
["hello " + "world"]
```

It is important to note that concatenation of strings will only occur when the left paramter is a string

For example:

```
["10" + 20]
```

This will give 1020 since it will convert the left to a string and do a concatenation

```
[10 + "20"]
```

This will give 30 since the left is a number and it will convert to string to a number automatically

## Subtraction

Example:

```
["testes" - "te"]
```

This will give the ouput `stes` which removes the first `te` found

## Multiple Subtraction

Example:

```
["testes" -- "te"]
```

This will give the ouput `ss` which removes all `te` found

# Identifiers

Identifers are variables from the interpreter and the regex
Example:

```
[#1]
[foo]
[bar]
[#count]
```

These are all variables
variables starting with `#` are reserved for the interpreted (this means your capture groups shouldn't start with `#`)

## Using capture groups

when using a regex, the capture groups can be used in interpreted block like the following

```
[#0]
[#1]
...
[#n]
```

The number represent the capture groups in order.

it is important to note that `#0` is the whole regex capture so to use the first capture group, it will be `#1`

that is on is only true if the global parameter is false, if global is true, #0 will be the first capture group

## Using named capture groups

Using the named capture groups is similar to the normal capture group. The only difference is to ommit the `#` at the start

For example, if a named capture group is `test` it will be used as follows

```
[test]
```

Import Note: When using any capture group or named capture group identifiers, they are all strings by default.

To use mathematical expression when they are the left parameter of tha math epxression it is nessecary to [convert them to numbers](#number-conversion)

# String conversion

Example:

```
[string(#1)]
```

Transform the expression between the parenthese to a string

# Number conversion

Example:

```
[number(#1)]
```

Transform the expression between the parenthese to a number
The second argument can also control the number of decimal

```
[number(10.532, 2)]
```

Will result in 10.53

# Example Usage

## Rename files from (number).txt to (number).mkv

```
nora '(\d+)\..*' '[#1].mkv'
```

## Rename files from (number).txt to (number + 10).txt

```
nora '(\d+)\..*' '[number(#1) + 10].txt'
```

## Rename files from (number>.txt to (number + 10).txt only if (number) is 0 if not leave it as (number).txt

```
nora '(\d+)\..*' '[#1 == 0 ? number(#1) + 10 : #1].txt'
```

## Remove spaces in file name

for example if we have this file name `1 2 3 4 5.txt`

if we want to remove the spaces, we can use a loop and the -g option to remove the whole captures

with this regex `([^\s]+)\s?([^\s]+)?` it matches all but spaces so we can loop the number of capture group given by `#cap_count` to connect them back together

You can look here to see how the regex works https://www.debuggex.com/r/9bUMa4OHscE8TyvR

```
nora -g '([^\s]+)\s?([^\s]+)?' '[for x in 0.#cap_count { #x }]'
```

The loop will loop from 0 to #cap_count which will be the number of found captures

`#x` will get the content of the capture group for the value of x so if x is 0 it will be like doing `#0` etc

this will result in the file being renamed to `12345.txt`
