# Fields

### Magic value
Unknown value with no information available on the internet, generally has a constant value for a given save version.

## Version header
### Signature
Must always have value `0xAA55AA55` for the file to be a valid Diablo 2 save file.

### Version
Version of the save file, this is separate from the game version, but one game version is always using single save version.

Known save versions:
|Save version|Game|Game version|
|-|-|-|
|71|Diablo 2|1.00 - 1.06b|
|87|Diablo 2 LoD|1.07|
|89|Diablo 2 LoD|1.08|
|92|Diablo 2 LoD|1.09 - 1.09d|
|96|Diablo 2 LoD|1.10 - 1.14d|
|105|Diablo 2 Resurrected RotW|3.0.92198|

This table is missing older Resurrected versions because new Blizzard sucks and you can't easily change game version.

## Checksum header
### File size
Save file size in bytes.

### Checksum
Save file checksum, if value is invalid, Diablo will not load that character.
To calculate checksum set this field to 0 and use algorithm.
TODO: algorithm

## Character information
### Character name
Character name padded with null bytes at the end.  
Rules:
- Must have at least 2 characters
- Must have at most 15 characters
- Must not contain invalid characters (valid: a-z, A-Z, -, _)
- Must not start or end with dash (-) or underscore (_)
- Must not contain dash (-) and underscore (_) at the same time
- Must not contain multiple dashes (-) or underscores (_)
- Must end with null bytes
- Must not contain any other character than null after first null byte

### Character status
Flags indicating character status.
|Bit|Name|Value|
|-|-|-|
|0|Unknown|Always 0|
|1|Unknown|Always 0|
|2|Hardcore|0=Normal 1=Hardcore|
|3|Dead|0=Alive 1=Dead|
|4|Unknown|Always 0|
|5|Expansion|0=Classic 1=Expansion|
|6|Unknown|Always 0|
|7|Unknown|Always 0|

### Character progression
Value indicating number of bosses killed. Used for determining character title and which difficulty you can play.
Values slightly differ between Classic and Expansion mode.

#### Classic
In classic mode value is incremented after every act boss.

|Value|Meaning|Title|Hardcore Title|
|-|-|-|-|
|0|Before killing Andariel on Normal difficulty||
|1|After killing Andariel on Normal difficulty||
|2|After killing Duriel on Normal difficulty||
|3|After killing Mephisto on Normal difficulty||
|4|After killing Diablo on Normal difficulty|Sir / Dame|Count / Countess|
|5|After killing Andariel on Nightmare difficulty|Sir / Dame|Count / Countess|
|6|After killing Duriel on Nightmare difficulty|Sir / Dame|Count / Countess|
|7|After killing Mephisto on Nightmare difficulty|Sir / Dame|Count / Countess|
|8|After killing Diablo on Nightmare difficulty|Lord / Lady|Duke / Duchess|
|9|After killing Andariel on Hell difficulty|Lord / Lady|Duke / Duchess|
|10|After killing Duriel on Hell difficulty|Lord / Lady|Duke / Duchess|
|11|After killing Mephisto on Hell difficulty|Lord / Lady|Duke / Duchess|
|12|After killing Diablo on Hell difficulty|Baron / Baroness|King / Queen|

#### Expansion
In expansion mode value is incremented after Andarial/Duriel/Mephisto by 1 and after Baal by 2 so it skips one value for every difficulty.

|Value|Meaning|Title|Hardcore Title|
|-|-|-|-|
|0|Before killing Andariel on Normal difficulty||
|1|After killing Andariel on Normal difficulty||
|2|After killing Duriel on Normal difficulty||
|3|After killing Mephisto on Normal difficulty||
|5|After killing Baal on Normal difficulty|Slayer|Destroyer|
|6|After killing Andariel on Nightmare difficulty|Slayer|Destroyer|
|7|After killing Duriel on Nightmare difficulty|Slayer|Destroyer|
|8|After killing Mephisto on Nightmare difficulty|Slayer|Destroyer|
|10|After killing Baal on Nightmare difficulty|Champion|Conqueror|
|11|After killing Andariel on Hell difficulty|Champion|Conqueror|
|12|After killing Duriel on Hell difficulty|Champion|Conqueror|
|13|After killing Mephisto on Hell difficulty|Champion|Conqueror|
|15|After killing Baal on Hell difficulty|Patriarch / Matriarch|Guardian|

### Active weapon set
Value indicating active weapon set.  
- 0 = Weapon Set 1  
- 1 = Weapon Set 2 (Expansion only)
