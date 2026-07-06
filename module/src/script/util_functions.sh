#
# This file is part of FS-Enhancer-Extreme.
#
# This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
#
# This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
# without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
# See the GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License along with this program;
# If not, see <https://www.gnu.org/licenses/>.
#
# Copyright (C) 2025-2026 XtrLumen
#

##VARIABLE##
#ZERO LEVEL#
ADB="/data/adb"
#ONE LEVEL#
SD="$ADB/service.d"
MODULESDIR="$ADB/modules"
FSEECONFIG="$ADB/fs_enhancer_extreme"
#TWO LEVEL#
FSMODDIR="$MODULESDIR/forgestore"
FSEEMODDIR="$MODULESDIR/fs_enhancer_extreme"
#THREE LEVEL#
MULTIPLETYPE="$FSEECONFIG/multiple.txt"
KERNELTYPE="$FSEECONFIG/kernel.txt"
FSEELOG="$FSEECONFIG/log/log.log"
TYPE="$FSEECONFIG/root.txt"
FSEEBIN="$FSEEMODDIR/bin"
#OTHER#
ORIGIN=$(basename "$0")
##END##

##FUNCTIONS##
#MULTILINGUAL#
[[ "$(getprop persist.sys.locale)" == *"zh"* || "$(getprop ro.product.locale)" == *"zh"* ]] && LOCALE="CN" || LOCALE="EN"
println() {
  [ "$LOCALE" = "$1" ] && {
    shift
    echo "$@"
  }
}
echo_cn() {
  println "CN" "$@"
}
echo_en() {
  println "EN" "$@"
}
#OTHER#
logout() { echo "$(date "+%m-%d %H:%M:%S.$(date +%3N)")  $$  $$ $1 System.out: [FSEE]$2" >> "$FSEELOG"; }
logs() { logout "$1" "<Service>$2"; }
logd() { logout "$1" "<Service.D>$2"; }
logp() { logout "$1" "<Post-Fs-Data>$2"; }
invoke() {
  case "$ORIGIN" in
    *"service.sh"*)
      class="logs"
      ;;
    *"post-fs-data.sh"*)
      class="logp"
      ;;
    *".fsee_state.sh"*)
      class="logd"
      ;;
  esac
  "$class" "I" "$1"
  if $FSEEBIN/fseed $2; then
    "$class" "I" "完毕"
  else
    "$class" "W" "失败"
  fi
}
check() {
  if [ "$(cat "$TYPE")" = "Multiple" ] || [ ! -d "$FSMODDIR" ] || [ -f "$FSMODDIR/disable" ] || sed -n '5p' "$FSMODDIR/module.prop" | grep -q -F "Enginex0"; then
    case "$ORIGIN" in
      *"post-fs-data.sh"*)
        logp "E" "环境异常,拦截执行"
        mv "$FSEEMODDIR/webroot" "$FSEEMODDIR/.webroot"
        mv "$FSEEMODDIR/action.sh" "$FSEEMODDIR/.action.sh"
        ;;
      *"service.sh"*)
        exit
        ;;
    esac
  else
    [[ "$ORIGIN" == *"post-fs-data.sh"* ]] && {
      logp "I" "环境正常,继续执行"
      mv "$FSEEMODDIR/.webroot" "$FSEEMODDIR/webroot"
      if [[ ! "$APATCH" && ! "$KSU" ]]; then
        mv "$FSEEMODDIR/.action.sh" "$FSEEMODDIR/action.sh"
      else
        mv "$FSEEMODDIR/action.sh" "$FSEEMODDIR/.action.sh"
      fi
    }
  fi
}
initwait() {
  until [ $(getprop sys.boot_completed) -eq 1 ]; do
    sleep 1s
  done
}
##END##

[[ "$ORIGIN" == *"post-fs-data.sh"* ]] && {
  rm -f "$MULTIPLETYPE"
  rm -f "$KERNELTYPE"
  rm -f "$FSEELOG"
  rm -f "$TYPE"
}