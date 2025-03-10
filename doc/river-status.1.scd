river-status(1)

# NAME

river-status - A status information client for river

# SYNOPSIS

*river-status* [_OPTIONS_]

# DESCRIPTION

*river-status* is a CLI tool that displays information about running instancies of river.
The output is always in JSON, but you can select to have it pretty printed for human readablility.

By default, it will display the names of each output and the current seat river is using.

*NOTE*: this tool uses the `river-status-unstable-v1` protocol which might be subject to significant breaking changes before river sees a stable 1.0 release.

# OPTIONS

*-a*, *--all*
	Equivalent of -f -l -m -t --tag -u --view-tags
*-f*, *--focused*
	Print information about the focused tags
*-h*, *--help*
	Print a help message and exit.
*-l*, *--layout*
	Print layout name of the focused tags.
*-m*, *--mode*
	Print mode name.
*--no-output*
	Force information about all outputs to be omitted from the output.
*--no-seat*
	Force information about all seats to be omitted from the output.
*-o*, *--output* {output}
	Select the output to display information about.
*-p*, *--pretty*
	Pretty print JSON.
*-s*, *--seat* {seat}
	Select the seat to display information about.
*--tag*
	Output the key *focusedTags* and numerical value representing which tag is
	focused for each output.
*-t*, *--title*
	Print the title of the focused view.
*-u*, *--urgent*
	Print information about urgent tags if there are any.
*--view-tags*
	Prints the tags of all views.
*-w*, *--watch*
	Continuously print information as it changes.

# SEE ALSO

_river_(1)
_riverctl_(1)

# AUTHORS

Developed and maintained by Rickard Hedlund <@>.

Source code is located at <https://github.com/grvn/river-status>.