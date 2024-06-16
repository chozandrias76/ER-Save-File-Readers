#!/bin/bash

# Default to dry-run
dry_run=true

# Parse arguments
for arg in "$@"; do
    case $arg in
        --dry=false)
        dry_run=false
        shift
        ;;
    esac
done

# Define the directory
directory="src/models/save_slot/face_data/attributes/"

# Function to convert snake_case to TitleCase
to_title_case() {
    echo "$1" | sed -r 's/(^|_)([a-z])/\U\2/g'
}

# Iterate over each file in the directory, assumes no mod.rs is present
for file in "$directory"*.rs; do
    filename=$(basename -- "$file")
    # Get the base name without extension
    base_name="${filename%.*}"
    # Convert the base name to TitleCase
    title_case_name=$(to_title_case "$base_name")
    # Replace underscores with nothing to form the final name
    final_name="${title_case_name//_/}"

    # Prepare the new content
    new_content=$(printf "use crate::impl_attribute;\n\nimpl_attribute!(%s);\n\nmod tests {\n  use super::*;\n  use crate::impl_read_ok_and_err_test;\n\n  impl_read_ok_and_err_test!(%s, ok: vec![1], err: vec![]);\n}" "$final_name" "$final_name")

    if $dry_run; then
        echo "Dry run: would update $file with:"
        echo "$new_content"
    else
        # Write the new contents to the file
        echo "$new_content" > "$file"
        echo "Updated $file"
    fi
done
