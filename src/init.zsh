SILVER_START=$(date +%s%3N)
unsetopt prompt_subst

preexec() {
	SILVER_START=$(date +%s%3N)
}

precmd() {
	SILVER_START=$(date +%s%3N)
	PROMPT="$(code=$? jobs=$(jobs | wc -l) cmdtime=$(($(date +%s%3N)-$SILVER_START)) silver print "${SILVER[@]}") "
}
