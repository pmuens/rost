section .multiboot_header
header_start:
    dd 0xe85250d6                                                   ; magic number
    dd 0                                                            ; architecture
    dd header_end - header_start                                    ; header length
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start)) ; checksum

    ; optional tags

    dw 0                                                            ; type
    dw 0                                                            ; flasg
    dd 8                                                            ; size
header_end:
