$files = Get-ChildItem -Path "C:\code\testruetlearn\leptos-learn" -Filter "Cargo.toml" -Recurse
$count = 0
foreach ($f in $files) {
    $c = [System.IO.File]::ReadAllText($f.FullName)
    if ($c -match '\[package\].*\[package\]') {
        Write-Host ("FIXING: " + $f.FullName)
        $idx = $c.LastIndexOf('[package]')
        $fixed = $c.Substring($idx)
        [System.IO.File]::WriteAllText($f.FullName, $fixed, [System.Text.UTF8Encoding]::new($false))
        $count++
    }
}
Write-Host ("Fixed " + $count + " files")
