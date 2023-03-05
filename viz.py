import glob
from pathlib import Path
import subprocess

all_dot_file_paths = glob.glob("./src/test_out/*.dot")
for dot_file_path in all_dot_file_paths:
    svg_file_path = f"{Path(dot_file_path).parent}/{Path(dot_file_path).stem}.svg"
    with open(svg_file_path, 'w') as output_file:
        output = subprocess.getoutput(f"dot -Tsvg {dot_file_path}")
        output_file.write(output)
