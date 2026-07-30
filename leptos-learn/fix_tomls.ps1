$files = Get-ChildItem -Path "C:\code\testruetlearn\leptos-learn\07_ssr" -Filter "Cargo.toml" -Recurse
foreach ($file in $files) {
    $content = [System.IO.File]::ReadAllText($file.FullName)
    $lines = [System.IO.File]::ReadAllLines($file.FullName)
    if ($lines.Count -le 1) {
        Write-Host "Fixing: $($file.FullName)"
        $fixed = $content -replace '\[package\]', "[package]`r`n"
        $fixed = $fixed -replace '\]name ', "]`r`nname "
        $fixed = $fixed -replace '\"version ', "`"`r`nversion "
        $fixed = $fixed -replace '\"edition ', "`"`r`nedition "
        $fixed = $fixed -replace '\"\[dependencies\]', "`"`r`n[dependencies]"
        $fixed = $fixed -replace '\](leptos)', "]`r`n`$1"
        [System.IO.File]::WriteAllText($file.FullName, $fixed, [System.Text.UTF8Encoding]::new($false))
    }
}
Write-Host "Done"
