import subprocess
import re
import sys

def find_ip_from_mac(mac_address):
    # Normalize MAC address to lowercase without separators for easier matching
    normalized_mac = mac_address.lower()

    # Run ARP command depending on OS
    try:
        if sys.platform.startswith("win"):
            arp_output = subprocess.check_output(["arp", "-a"], text=True)
        else:
            arp_output = subprocess.check_output(["arp", "-n"], text=True)
    except Exception as e:
        print(f"Error running arp: {e}")
        return None

    # Search for the MAC address in the output
    for line in arp_output.splitlines():
        if normalized_mac in line.lower():
            ip_match = re.search(r"\b(?:[0-9]{1,3}\.){3}[0-9]{1,3}\b", line)
            if ip_match:
                return ip_match.group(0)

    return None

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python find_ip_from_mac.py <MAC_ADDRESS>")
        sys.exit(1)

    mac = sys.argv[1]
    ip = find_ip_from_mac(mac)

    if ip:
        print(f"IP address for {mac} is {ip}")
    else:
        print(f"No IP found for MAC address {mac}")
