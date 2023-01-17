#!/usr/bin/env python3
import os
import shutil

GIR_FILES=[
"Soup-3.0", "WebKit2-4.1", "WebKit2WebExtension-4.1", "JavaScriptCore-4.1",
]

dest_dir = os.path.abspath("./")

for file in GIR_FILES:
    src = f"/usr/share/gir-1.0/{file}.gir"
    dest = f"{dest_dir}/{file}.gir"
    try:
        shutil.copy(src, dest)
    except FileNotFoundError:
        print(f"gir file not found {file}")
