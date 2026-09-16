#!/bin/bash

# Check if brotli is installed
if ! command -v brotli &> /dev/null; then
    echo "Error: brotli is not installed. Please install it first."
    exit 1
fi

# Check if the correct number of arguments is provided
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <old_folder> <new_folder>"
    exit 1
fi

old_folder="$1"
new_folder="$2"

# Check if new folder exists
if [ ! -d "$new_folder" ]; then
    echo "Error: New folder '$new_folder' does not exist."
    exit 1
fi

output_dir="$new_folder"

# Check if old folder exists
if [ ! -d "$old_folder" ]; then
    echo "Old folder '$old_folder' does not exist. Compressing all files in new folder..."
    
    # Process all files in the new folder
    find "$new_folder" -type f | while read -r new_file; do
        # Get relative path from new_folder
        rel_path="${new_file#$new_folder/}"
        
        # Create corresponding directory structure in output
        output_file="$output_dir/$rel_path"
        # mkdir -p "$(dirname "$output_file")"
        
        # Compress the file
        echo "Compressing: $rel_path"
        brotli -c "$new_file" > "${output_file}.br"
    done
else
    echo "Comparing files between '$old_folder' and '$new_folder'..."
    
    # Process files in the new folder
    find "$new_folder" -type f | while read -r new_file; do
        # Get relative path from new_folder
        rel_path="${new_file#$new_folder/}"
        old_file="$old_folder/$rel_path"
        
        # Create corresponding directory structure in output
        output_file="$output_dir/$rel_path"
        # mkdir -p "$(dirname "$output_file")"
        
        if [ -f "$old_file" ]; then
            # File exists in both folders, copy the Brotli-compressed version
            echo "File exists in both folders: $rel_path"
            if [ -f "${old_file}.br" ]; then
                cp "${old_file}.br" "${output_file}.br"
                echo "  Copied compressed version from old folder"
            else
                # Compress the old file if not already compressed
                brotli -c "$old_file" > "${output_file}.br"
                echo "  Compressed old file and copied"
            fi
        else
            # File only exists in new folder, compress it
            echo "New file: $rel_path"
            brotli -c "$new_file" > "${output_file}.br"
            echo "  Compressed new file"
        fi
    done
fi

echo "Processing complete. Results are in '$output_dir'."