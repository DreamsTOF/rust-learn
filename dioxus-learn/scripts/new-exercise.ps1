param([string]$Chapter,[int]$Number,[string]$Name,[ValidateSet("exercise","answer")][string]$Type)
$baseDir="c:\code\testruetlearn\dioxus-learn"
$suffix=if($Type-eq"answer"){"_answer"}else{""}
$fn="e{0:d3}_{1}{2}"-f$Number,$Name,$suffix
$td="$baseDir\$Chapter\$fn"
New-Item -ItemType Directory -Path "$td\src" -Force|Out-Null
$ct=@"
[package]
name = "$fn"
version.workspace = true
edition.workspace = true

[dependencies]
dioxus.workspace = true
dioxus-logger.workspace = true
"@
Set-Content -Path "$td\Cargo.toml" -Value $ct
if($Type-eq"answer"){
$mr=@"
use dioxus::prelude::*;
fn App() -> Element {
    rsx! { div { h1 { "Exercise $Number" } p { "Answer placeholder" } } }
}
fn main() { dioxus::launch(App); }
"@
}else{
$mr=@"
// TODO e${Number}: ${Name}
// Follow TODO comments to complete this exercise
use dioxus::prelude::*;
fn App() -> Element { todo!() }
fn main() { dioxus::launch(App); }
"@
}
Set-Content -Path "$td\src\main.rs" -Value $mr
Write-Host "$fn"
