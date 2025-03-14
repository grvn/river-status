
use builtin;
use str;

set edit:completion:arg-completer[river-status] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'river-status'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'river-status'= {
            cand -o 'Select the output to display information about.'
            cand --output 'Select the output to display information about.'
            cand -s 'Select the seat to display information about.'
            cand --seat 'Select the seat to display information about.'
            cand -a 'Equivalent of -f -l -m -t --tag -u --view-tags'
            cand --all 'Equivalent of -f -l -m -t --tag -u --view-tags'
            cand -f 'Print information about the focused tags'
            cand --focused 'Print information about the focused tags'
            cand -l 'Print layout name of the focused tags.'
            cand --layout 'Print layout name of the focused tags.'
            cand -m 'Print mode name.'
            cand --mode 'Print mode name.'
            cand --no-output 'Force information about all outputs to be omitted from the output'
            cand --no-seat 'Force information about all seats to be omitted from the output.'
            cand -p 'Pretty print JSON.'
            cand --pretty 'Pretty print JSON.'
            cand --sleep 'optional delay (in milliseconds) between calls to river for status updates. This option is a no-op without `--watch`.'
            cand -T 'Output the key *focusedTags* and numerical value representing which tag is focused for each output.'
            cand --tag 'Output the key *focusedTags* and numerical value representing which tag is focused for each output.'
            cand -t 'Print the title of the focused view.'
            cand --title 'Print the title of the focused view.'
            cand -u 'Print information about urgent tags if there are any.'
            cand --urgent 'Print information about urgent tags if there are any.'
            cand --view-tags 'Prints the tags of all views.'
            cand -w 'Continuously print information as it changes.'
            cand --watch 'Continuously print information as it changes.'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
    ]
    $completions[$command]
}
