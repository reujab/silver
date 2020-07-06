SILVER_START=$(date +%s%3N)
unsetopt prompt_subst

preexec() {
  SILVER_START=$(date +%s%3N)
}

precmd() {
  PROMPT="$(code=$? jobs=$(jobs | wc -l) cmdtime=$(($(date +%s%3N)-$SILVER_START)) silver lprint ${SILVER_LEFT:-$SILVER})"
  ZLE_RPROMPT_INDENT=0
  RPROMPT="$(code=$? jobs=$(jobs | wc -l) cmdtime=$(($(date +%s%3N)-$SILVER_START)) silver rprint $SILVER_RIGHT)"
  SILVER_START=$(date +%s%3N)
}
