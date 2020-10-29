function prompt {
    $lastCommand = Get-History -Count 1
    $env:code = $LASTEXITCODE
    $env:cmdtime = [uint64]($lastCommand.EndExecutionTime - $lastCommand.StartExecutionTime).TotalMilliseconds
    $env:jobs = @(Get-Job).Count
    Start-Process -Wait -NoNewWindow -FilePath silver
    "$([char]0x1b)[0m"
}
$Env:VIRTUAL_ENV_DISABLE_PROMPT = 1
