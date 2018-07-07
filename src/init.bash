PROMPT_COMMAND=bronze_prompt
bronze_prompt() {
	PS1="$(code=$? jobs=$(jobs -p | wc -l) bronze print "${BRONZE[@]}") "
}
