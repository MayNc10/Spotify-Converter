# Spotify Converter

This is just a simple tool for converting data downloaded from Spotify into a format that can be used to scrobble on Last.fm. Simple pass the program a path to a folder containing all the downloaded JSON, and an optional date to cap scrobbles at, and the parser will convert and split the JSON into smaller chunks that can be scrobbled (2600 songs per file). You can also run `spotify-converter -h` to get some info about the tool. 