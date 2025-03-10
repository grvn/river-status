complete -c river-status -s o -l output -d 'Select the output to display information about.' -r
complete -c river-status -s s -l seat -d 'Select the seat to display information about.' -r
complete -c river-status -s a -l all -d 'Equivalent of -f -l -m -t --tag -u --view-tags'
complete -c river-status -s f -l focused -d 'Print information about the focused tags'
complete -c river-status -s l -l layout -d 'Print layout name of the focused tags.'
complete -c river-status -s m -l mode -d 'Print mode name.'
complete -c river-status -l no-output -d 'Force information about all outputs to be omitted from the output'
complete -c river-status -l no-seat -d 'Force information about all seats to be omitted from the output.'
complete -c river-status -s p -l pretty -d 'Pretty print JSON.'
complete -c river-status -s T -l tag -d 'Output the key *focusedTags* and numerical value representing which tag is focused for each output.'
complete -c river-status -s t -l title -d 'Print the title of the focused view.'
complete -c river-status -s u -l urgent -d 'Print information about urgent tags if there are any.'
complete -c river-status -l view-tags -d 'Prints the tags of all views.'
complete -c river-status -s w -l watch -d 'Continuously print information as it changes.'
complete -c river-status -s h -l help -d 'Print help (see more with \'--help\')'
complete -c river-status -s V -l version -d 'Print version'
