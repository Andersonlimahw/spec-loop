# Specloop Scenarios and Loops Creation Rule

To avoid accidentally overwriting existing scenario or loop files when running creation scripts, the scripts verify file presence:
- Overwriting `.specloop/scenarios/*.md` or `.specloop/loops/*.md` requires the `--force` flag.
- Without `--force`, scripts will exit cleanly with error code `1` and prompt the user to use `--force`.
