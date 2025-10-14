#
# This file is part of TS-Enhancer-Extreme.
#
# TS-Enhancer-Extreme is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
#
# TS-Enhancer-Extreme is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
# without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
# See the GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License along with TS-Enhancer-Extreme.
# If not, see <https://www.gnu.org/licenses/>.
#
# Copyright (C) 2025 TheGeniusClub (Organization)
# Copyright (C) 2025 XtrLumen (Developer)
#

TMPDIR_FOR_VERIFY="$TMPDIR/.vunzip"
mkdir -p "$TMPDIR_FOR_VERIFY"

extract() {
  unpack() {
    local zip=$1
    local file=$2
    local dir=$3
    local quiet=$4
    local skip=$5
    unzip -o "$zip" "$file" -d "$dir" >&2
    file_path="$dir/$file"
    if [ -f "$file_path" ]; then
      unzip -o "$zip" "MANIFEST/$file.sha256" -d "$TMPDIR_FOR_VERIFY" >&2
      hash_path="$TMPDIR_FOR_VERIFY/MANIFEST/$file.sha256"
      if [ -f "$hash_path" ]; then
        (echo "$(cat "$hash_path")  $file_path" | sha3sum -a 256 -c -s -) || {
          abort_cn "$file 被篡改!"
          abort_en "Failed to verify $file"
        }
      elif [ ! "$skip" = "-s" ]; then
        abort_cn "$file.sha256 不存在!"
        abort_en "$file.sha256 not exists"
      fi
    else
      abort_cn "$file 不存在!"
      abort_en "$file not exists"
    fi
    [ "$quiet" = "-q" ] || {
      print_cn "- $file 未篡改"
      print_en "- Verified $file" >&1
    }
  }
  if [[ "$2" == */\* ]]; then
    for files in $(unzip -l "$1" "$2" | awk 'NR>3 {print $4}' | grep -v '\.sha256$' | grep -v '/$' | grep -v '^$'); do
      unpack "$1" "$files" "$3" "$4" "$5"
    done
  else
    unpack "$@"
  fi
}

extract "$ZIPFILE" 'META-INF/com/google/android/*' "$TMPDIR_FOR_VERIFY" -q -s
extract "$ZIPFILE" 'verify.sh' "$TMPDIR_FOR_VERIFY" -q
extract "$ZIPFILE" 'customize.sh' "$TMPDIR_FOR_VERIFY" -q