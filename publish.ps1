param (
    [string]$local
)
clear
$apiKey = "PUT YOUR API KEY HERE"

if ($local -eq "local") {
    $server = "https://localhost:7086/api/Wasm/Upload/g6rmnkr2pglbymasff0mrl192tec8ki242joxzktsboso3agqi"
}
else {
    $server = "https://www.d1ag0n.com/api/Wasm/Upload/"
}
    
cargo build --target wasm32-unknown-unknown

if ($LastExitCode -eq 0) {
    if ($local -eq "local") {
        curl -k --fail -F "file=@./target/wasm32-unknown-unknown/debug/actr-rust-sample.wasm" $server
    } else {
        curl --fail -F "file=@./build/release.wasm" $server + $apiKey
    }
    Remove-Item -Force:$true -Confirm:$false -Recurse:$true ./target
} else {
    Write-Output "build failed"
}

if ($LastExitCode -eq 0) {
    Write-Output "$script.wasm uploaded to $server"
} 
