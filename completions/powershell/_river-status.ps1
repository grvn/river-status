
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'river-status' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'river-status'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'river-status' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'Select the output to display information about.')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Select the output to display information about.')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Select the seat to display information about.')
            [CompletionResult]::new('--seat', '--seat', [CompletionResultType]::ParameterName, 'Select the seat to display information about.')
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'Equivalent of -f -l -m -t --tag -u --view-tags')
            [CompletionResult]::new('--all', '--all', [CompletionResultType]::ParameterName, 'Equivalent of -f -l -m -t --tag -u --view-tags')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Print information about the focused tags')
            [CompletionResult]::new('--focused', '--focused', [CompletionResultType]::ParameterName, 'Print information about the focused tags')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'Print layout name of the focused tags.')
            [CompletionResult]::new('--layout', '--layout', [CompletionResultType]::ParameterName, 'Print layout name of the focused tags.')
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Print mode name.')
            [CompletionResult]::new('--mode', '--mode', [CompletionResultType]::ParameterName, 'Print mode name.')
            [CompletionResult]::new('--no-output', '--no-output', [CompletionResultType]::ParameterName, 'Force information about all outputs to be omitted from the output')
            [CompletionResult]::new('--no-seat', '--no-seat', [CompletionResultType]::ParameterName, 'Force information about all seats to be omitted from the output.')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Pretty print JSON.')
            [CompletionResult]::new('--pretty', '--pretty', [CompletionResultType]::ParameterName, 'Pretty print JSON.')
            [CompletionResult]::new('-T', '-T ', [CompletionResultType]::ParameterName, 'Output the key *focusedTags* and numerical value representing which tag is focused for each output.')
            [CompletionResult]::new('--tag', '--tag', [CompletionResultType]::ParameterName, 'Output the key *focusedTags* and numerical value representing which tag is focused for each output.')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'Print the title of the focused view.')
            [CompletionResult]::new('--title', '--title', [CompletionResultType]::ParameterName, 'Print the title of the focused view.')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'Print information about urgent tags if there are any.')
            [CompletionResult]::new('--urgent', '--urgent', [CompletionResultType]::ParameterName, 'Print information about urgent tags if there are any.')
            [CompletionResult]::new('--view-tags', '--view-tags', [CompletionResultType]::ParameterName, 'Prints the tags of all views.')
            [CompletionResult]::new('-w', '-w', [CompletionResultType]::ParameterName, 'Continuously print information as it changes.')
            [CompletionResult]::new('--watch', '--watch', [CompletionResultType]::ParameterName, 'Continuously print information as it changes.')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
