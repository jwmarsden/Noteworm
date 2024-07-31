# Noteworm
I eat Notes for breakfast. 

```
Its a Very Hungry Noteworm.

Usage: noteworm [COMMAND]

Commands:
  backup  Backup Repository
  clean   Clean Repository
  report  Generate Reports for Repository
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

# About

This project is a tool that I use to backup and clean my Obsidian repositories. It does a one way syncronisation from my working folder (that I store/sync using cloud storage) to a separate repository that I also periodically backup in GitHub. 

# Commands Summary

## Backup

Trigger the backup command.

```
Backup Repository

Usage: noteworm backup [OPTIONS] --destination <DESTINATION>

Options:
  -s, --source <SOURCE>            Source Path (File Path) [default: .]
  -d, --destination <DESTINATION>  Destination Path (File Path)
  -t, --test                       
  -h, --help                       Print help
```
