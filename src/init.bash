PROMPT_COMMAND=silver_prompt
silver_prompt() {
    PS1="$(code=$? jobs=$(jobs -p | wc -l) silver lprint)"
}
export VIRTUAL_ENV_DISABLE_PROMPT=1
