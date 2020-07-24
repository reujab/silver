function fish_prompt
    test -n "$SILVER_LEFT"; or set -l SILVER_LEFT $SILVER
    env code=$status jobs=(count (jobs -p)) cmdtime={$CMD_DURATION} silver lprint $SILVER_LEFT
end
function fish_right_prompt
    env code=$status jobs=(count (jobs -p)) cmdtime={$CMD_DURATION} silver rprint $SILVER_RIGHT
end
