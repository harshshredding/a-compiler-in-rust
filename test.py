import glob
import subprocess
from pathlib import Path
from parser import parse_file

# test_source_file_paths = glob.glob('test_cases/test_source_files/*.src')
# for test_source_file in test_source_file_paths:
#     file_name = Path(test_source_file).name
#     file = Path(test_source_file).stem
#     print(f"tokenizing {file_name}")
#     subprocess.run(['cargo', 'run', test_source_file, f"test_cases/test_tokens_files/{file}.tokens"])


test_token_file_paths = glob.glob('test_cases/test_tokens_files/*.tokens')
for tokens_path in test_token_file_paths:
    tokens_file_name = Path(tokens_path).stem
    print(f"Parsing {tokens_file_name}")
    output_derivation_path = f'test_cases/test_syntax_derivations/{tokens_file_name}.derivation'
    try:
        parse_file(input_tokens_file=tokens_path, output_derivation_file=output_derivation_path)
    except KeyError:
        print(f"SYNTAX ERROR in {Path(tokens_path).name}. Please check {output_derivation_path} for info.")





