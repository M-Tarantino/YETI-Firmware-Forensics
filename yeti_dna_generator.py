import sqlite3
import os

DB_NAME = "yeti_dna.db"

# Curated list of signatures that usually require separate tools
SIGNATURES = [
    # --- The "Sasquatch" Set (Modified SquashFS) ---
    ("SquashFS_Modified_shsq", "73687371", "Proprietary_FS"), # TP-Link / Atheros
    ("SquashFS_Modified_qshs", "71736873", "Proprietary_FS"), # Broadcom
    ("SquashFS_Modified_tqsh", "74717368", "Proprietary_FS"), # Rare TP-Link variants
    ("SquashFS_Modified_sqlz", "73716c7a", "Proprietary_FS"), # LZMA-based Squash
    
    # --- The "Unblob" Set (Complex Containers) ---
    ("TRX_Header", "48445230", "Container"),                # Broadcom TRX
    ("U-Boot_uImage", "27051956", "Bootloader"),           # Standard IoT Bootloader
    ("NETGEAR_Header", "2a23245e", "Container"),            # Netgear .chk
    ("UBI_Image", "55424923", "Filesystem"),               # Flash memory FS
    
    # --- The "Binwalk" Set (Standard Identifiers) ---
    ("ELF_Binary", "7f454c46", "Executable"),
    ("LZMA_Stream", "5d000080", "Compression"),
    ("Gzip_Stream", "1f8b08", "Compression"),
]

def generate_dna():
    if os.path.exists(DB_NAME): os.remove(DB_NAME)
    conn = sqlite3.connect(DB_NAME)
    cursor = conn.cursor()
    cursor.execute("CREATE TABLE signatures (id INTEGER PRIMARY KEY, name TEXT, magic TEXT, category TEXT)")
    cursor.executemany("INSERT INTO signatures (name, magic, category) VALUES (?, ?, ?)", SIGNATURES)
    conn.commit()
    print(f"[+] DNA Database Generated with {len(SIGNATURES)} proprietary-ready signatures.")
    conn.close()

if __name__ == "__main__":
    generate_dna()