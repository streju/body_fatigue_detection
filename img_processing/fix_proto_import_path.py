import os
import re

def modify_import_path(import_path, missing_import_dir):
    import_path_dirs = import_path.split('/')
    if missing_import_dir in import_path_dirs:
        return f'import public "{import_path}";'
    else:
        return f'import public "{missing_import_dir}/{import_path}";'


proto_directory = "proto/src/proto_generated"
missing_import = "proto_generated"

import_pattern = re.compile(r'^import\s+public\s+"(.+\.proto)";$', re.MULTILINE)

for root, _, files in os.walk(proto_directory):
    for file in files:
        if file.endswith(".proto"):
            file_path = os.path.join(root, file)
            with open(file_path, 'r') as f:
                content = f.read()
                
            modified_import = import_pattern.sub(
                lambda match: modify_import_path(match.group(1), missing_import), 
                content
            )
            with open(file_path, 'w') as f:
                f.write(modified_import)