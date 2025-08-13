param(
    [Parameter(Mandatory = $true)]
    [string]$MacAddress
)

# Normalize MAC address to lowercase without separators
$NormalizedMac = $MacAddress.ToLower()

# Optional: Ping the broadcast address to populate ARP table
Write-Host "Scanning network to populate ARP table..."
$LocalIP = (Get-NetIPAddress -AddressFamily IPv4 -InterfaceAlias 'Ethernet','Wi-Fi' `
    | Where-Object {$_.IPAddress -notmatch '^169\.254'} | Select-Object -First 1).IPAddress
$Subnet = ($LocalIP -split '\.')[0..2] -join '.'
1..254 | ForEach-Object {
    Start-Job { param($ip) Test-Connection -Count 1 -Quiet $ip | Out-Null } -ArgumentList "$Subnet.$_"
} | Wait-Job | Remove-Job

# Get ARP table
$ArpTable = arp -a

# Search for the MAC address
$Match = $ArpTable | Where-Object { $_.ToLower() -match $NormalizedMac }

if ($Match) {
    # Extract IP address from the matching line
    if ($Match -match '(\d{1,3}(\.\d{1,3}){3})') {
        $IPAddress = $matches[1]
        Write-Host "IP address for MAC $MacAddress is $IPAddress"
    }
    else {
        Write-Host "MAC address found, but IP extraction failed."
    }
}
else {
    Write-Host "No IP found for MAC address $MacAddress"
}
