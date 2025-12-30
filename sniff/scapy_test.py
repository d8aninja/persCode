from scapy.all import sniff, wrpcap, IP, TCP, UDP

def packet_handler(packet):
    """
    Handles each captured packet.

    Some useful options:

    sniff(iface="eth0", ...) - specify network interface
    sniff(filter="tcp port 80", ...) - BPF filter (e.g., HTTP traffic)
    sniff(timeout=30, ...) - capture for 30 seconds instead of count
    Remove count=10 to capture indefinitely (Ctrl+C to stop)

    :param packet: Description
    """
    if IP in packet:
        src_ip = packet[IP].src
        dst_ip = packet[IP].dst
        
        print(f"\n[+] New Packet:")
        print(f"    Source: {src_ip}")
        print(f"    Destination: {dst_ip}")
        
        if TCP in packet:
            print(f"    Protocol: TCP")
            print(f"    Src Port: {packet[TCP].sport} → Dst Port: {packet[TCP].dport}")
        elif UDP in packet:
            print(f"    Protocol: UDP")
            print(f"    Src Port: {packet[UDP].sport} → Dst Port: {packet[UDP].dport}")

# Capture packets
print("Starting packet capture...")
packets = sniff(prn=packet_handler, count=50)

# Save to PCAP file
output_file = "capture.pcap"
wrpcap(output_file, packets)
print(f"\n[✓] Saved {len(packets)} packets to {output_file}")