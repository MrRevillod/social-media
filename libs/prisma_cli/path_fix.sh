#!/bin/bash

FILE="./libs/common/src/prisma.rs"

if [[ ! -f "$FILE" ]]; then
    echo "Error: El archivo $FILE no existe."
    exit 1
fi

original_line='pub static DATAMODEL_STR : & '\''static str = include_str ! ("/social-net/libs/common/prisma/schema.prisma") ;'
replace_line='pub static DATAMODEL_STR : & '\''static str = include_str ! ("../prisma/schema.prisma") ;'

content=$(cat "$FILE")
new_content="${content/"$original_line"/"$replace_line"}"

echo "$new_content" | sudo tee "$FILE" > /dev/null

echo "Línea reemplazada en $FILE"