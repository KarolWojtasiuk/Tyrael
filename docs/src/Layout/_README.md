# File layout

Diablo 2 characters are stored in **.d2s** files, format of this file slightly differs from version to version.
Integers are stored in **little endian** order, and each file starts with version header.

## Version header
|Total Offset|Section Offset|Length|Field|
|-|-|-|-|
|0|0|4|[Signature](Fields.md#signature)|
|4|4|4|[Version](Fields.md#version)|

Rest of the save layout varies depending on save version.
For more information, visit dedicated pages for each save version.
