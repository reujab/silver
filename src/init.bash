PROMPT_COMMAND=silver_prompt
silver_prompt() {
	PS1="$(code=$? jobs=$(jobs -p | wc -l) silver print "${SILVER[@]}") "
}
