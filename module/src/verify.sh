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
      unzip -o "$zip" "$file.sha512" -d "$TMPDIR_FOR_VERIFY" >&2
      hash_path="$TMPDIR_FOR_VERIFY/$file.sha512"
      if [ -f "$hash_path" ]; then
        (echo "$(cat "$hash_path")  $file_path" | sha512sum -c -s -) || {
          abort_cn "$file 被篡改!"
          abort_en "Failed to verify $file"
        }
      elif [ ! "$skip" = "-s" ]; then
        abort_cn "$file.sha512 不存在!"
        abort_en "$file.sha512 not exists"
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
    for files in $(unzip -l "$1" "$2" | awk 'NR>3 {print $4}' | grep -v '\.sha512$' | grep -v '/$' | grep -v '^$'); do
      unpack "$1" "$files" "$3" "$4" "$5"
    done
  else
    unpack "$@"
  fi
}

extract "$ZIPFILE" 'META-INF/com/google/android/*' "$TMPDIR_FOR_VERIFY" -q -s
extract "$ZIPFILE" 'verify.sh' "$TMPDIR_FOR_VERIFY" -q
extract "$ZIPFILE" 'customize.sh' "$TMPDIR_FOR_VERIFY" -q