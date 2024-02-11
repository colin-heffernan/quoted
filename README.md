# Quoted

Quoted is a simple program that displays a random quote from a JSON list in your .config.


# Usage

Place a file called `config.json` in the configuration directory. Here is the configuration
directory location on several common operating systems:
- Windows: `C:\Users\user\AppData\Roaming\quoted`
- MacOS: `/Users/user/Library/Application Support/quoted`
- Linux: `/home/user/.config/quoted`

A sample `config.json` may look like this:
```json
[
  ["Those that can, do. Those that can't, complain.", "Linus Torvalds"]
]
```

To add more quotes, simply add more entries to the top-level array in the form:
`["quote", "author"]`


# License

Quoted is licensed under the MIT License.
Quoted uses the Dirs library under the terms of the MIT License.
Quoted uses the Rand library under the terms of the MIT License.
Quoted uses the Serde library under the terms of the MIT License.
