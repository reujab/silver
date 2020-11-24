function prompt {
    $lastCommand = Get-History -Count 1
    $env:code = $LASTEXITCODE
    $env:cmdtime = [uint64]($lastCommand.EndExecutionTime - $lastCommand.StartExecutionTime).TotalMilliseconds
    $env:jobs = @(Get-Job).Count
    Start-Process -Wait -NoNewWindow silver lprint
    "$([char]0x1b)[0m"
}
$env:SILVER_SHELL = "@SILVER_SHELL@"
$env:VIRTUAL_ENV_DISABLE_PROMPT = 1
