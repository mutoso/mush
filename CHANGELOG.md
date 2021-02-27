## v0.1.0 (2021-02-27)

### Feat

- ignore SIGINT (^C) so the shell doesn't close when the user terminates the program
- **fallback**: load bash aliases in fallback mode
- **history**: don't log if command starts with whitespace
- add "fallback mode", which send commands to bash instead of mush (activated with ctrl+t)
- **history**: add logging
- **cd**: move to home directory when cd is run with no arguments
- add username and hostname to prompt
- **cd**: add cd command
- add current working directory to prompt
- initial commit
