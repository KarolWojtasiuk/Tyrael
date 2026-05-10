# Architecture

Tyrael is split into layers with different responsibility and structure.

## Layers

### Raw Data

This layer represents raw save data split into sections and data is not validated in any way except section headers.
Data structure depends on save version, there is no abstraction here, just raw bytes.
This layer is responsible for writing and reading the save file.

Modifying data at this stage is strongly discouraged and will most likely result in save corruption.

Conversion to [Data](#data) layer is lossless.

### Data

This layer represents save file extracted to primitive fields.
Values are not validated in any way beyond the type system, if a value fits the field type, it is valid.
Data structure is fully independent of save version, it is a universal save representation.

Modifying data at this stage is not recommended except in unusual cases and may still result in save corruption.

Conversion to [Raw Data](#raw-data) layer can be lossy if target version does not support all fields.  
Conversion to [Info](#info) layer is lossless but may fail when used with incompatible [data repository](#data-repository).

### Info

This layer uses [data repository](#data-repository) to interpret field values.
At this stage all fields are always valid and human-readable.

Modifying data at this stage is recommended and should never corrupt the save file if used with valid [data repository](#data-repository).

Conversion to [Data](#data) layer is lossless.

## Data repository

Data structure that represents game data, source of information for all field values.
Can be generated from game files, and should be backwards compatible between game versions.

For debugging/experimenting purposes there is also `RawDataRepository`, which skips all validations and may still result in save corruption.
