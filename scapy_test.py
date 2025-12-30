from scapy.all import sniff, wrpcap, IP, TCP, UDP, Raw

def hexdump(data, length=16):
    """Display data in hex dump format"""
    for i in range(0, len(data), length):
        chunk = data[i:i+length]
        hex_part = ' '.join(f'{b:02x}' for b in chunk)
        ascii_part = ''.join(chr(b) if 32 <= b < 127 else '.' for b in chunk)
        print(f'    {i:04x}  {hex_part:<{length*3}}  {ascii_part}')

def packet_handler(packet):
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
        
        # Check for payload and display hex dump
        if Raw in packet:
            raw_data = packet[Raw].load
            print(f"    Payload ({len(raw_data)} bytes):")
            hexdump(raw_data)
            
            # Also try to show as text if it's readable
            try:
                decoded = raw_data.decode('utf-8', errors='ignore')
                if any(32 <= ord(c) < 127 for c in decoded[:100]):  # Check if somewhat readable
                    print(f"    Text preview: {decoded[:100]}...")
            except:
                pass

# Capture packets
print("Starting packet capture...")
packets = sniff(prn=packet_handler, count=50)

# Save to PCAP file
output_file = "capture.pcap"
wrpcap(output_file, packets)
print(f"\n[✓] Saved {len(packets)} packets to {output_file}")