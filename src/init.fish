function fish_prompt; env code=$status jobs=(count (jobs -p)) cmdtime={$CMD_DURATION}ms silver print $SILVER; echo -n ' '; end
