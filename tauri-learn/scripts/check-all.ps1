$json = cargo metadata --no-deps --format-version 1 | ConvertFrom-Json
$pkgs = @($json.packages | Where-Object { $_.name -match '^e\d\d-' -and $_.name -notmatch '^e0[1-8]' -and $_.name -notmatch '^e[4-5][0-9]' } | ForEach-Object { $_.name })
"验证 crate 数: $($pkgs.Count)"
$cargoArgs = @('check') + ($pkgs | ForEach-Object { @('-p', $_) })
& cargo @cargoArgs 2>&1 | Select-String -Pattern '^error|^warning|Finished'
"exit: $LASTEXITCODE"