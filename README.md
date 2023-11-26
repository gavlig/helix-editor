## ⚠️THIS IS A FORK THAT IS NOT FUNCTIONAL ON ITS OWN⚠️

SEE https://github.com/helix-editor/helix if you're looking for Helix Editor  
SEE https://github.com/gavlig/kodiki if you're looking for Kodiki  

### Non exhaustive list of changes:

- Added an alternative rendering pipeline to be used from external applications
- Added word selection by double click
- Added selecting with mouse click and pressed shift
- Added smart tab indent that matches expected indentation in current line (mimic vscode)
- Added avoiding of showing the same completion if it was closed by user in the same cursor position
- Added formatting to symbol picker
- Added "current symbol under cursor" field in the status panel
- Added initial focus on first completion item to apply it on first press of enter
- Added lsp symbols caching for external rendering and less frequent calls to lsp server
- Added versioning for document symbols and diagnostics for external renderning and syncing
- Added dark theme indicator for external logic
- Modified scrolling and cursor positioning because requirements are different from a terminal application
- Allowed enabling inlay hints from external application

### Hotkey changes

- Added selecting with shift+left/right/up/down/home/end in normal mode
- Added word deleting with control+del/backspace in normal mode
- Added control+/ for commenting
- Removed control+c for commenting
- Removed control+f/b for scrolling
- Added control+f for search
- Added control+c/v for copy/pasting in normal mode
- Added control+tab for buffer picker. Might be not the best choice but works for a migrant from vs-like IDE-s
- Added alt+left/right for jumping forward/backward in jump list
- Added control+space for completion suggestions
- Added control+shift+space for signature help
- Added control+f for search