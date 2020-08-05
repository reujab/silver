function fish_prompt
    env code=$status jobs=(count (jobs -p)) cmdtime={$CMD_DURATION} silver lprint
end
function fish_right_prompt
    env code=$status jobs=(count (jobs -p)) cmdtime={$CMD_DURATION} silver rprint
end
