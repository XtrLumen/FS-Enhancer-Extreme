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
SINGLETYPE="$FSEECONFIG/root.txt"
FSEELOG="$FSEECONFIG/log/log.log"
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
fseed() {
  $FSEEMODDIR/bin/fseed "$@"
}
logout() {
  case "$ORIGIN" in
    *"post-fs-data.sh"*)
      ID="<Post-Fs-Data>"
      ;;
    *"service.sh"*)
      ID="<Service>"
      ;;
    *".fsee_state.sh"*)
      ID="<Service.D>"
      ;;
  esac
  LEVEL=$1
  shift
  echo "$(date "+%m-%d %H:%M:%S.$(date +%3N)")  $$  $$ $LEVEL System.out: [FSEE]$ID$@" >> "$FSEELOG"
}
logI() {
  logout "I" "$@"
}
logW() {
  logout "W" "$@"
}
logE() {
  logout "E" "$@"
}
initwait() {
  until [ "$(getprop sys.boot_completed)" -eq 1 ]; do
    sleep 1s
  done
}
check() {
  if [ "$(cat "$SINGLETYPE")" = "Multiple" ] || [ ! -d "$FSMODDIR" ] || [ -f "$FSMODDIR/disable" ] || sed -n '5p' "$FSMODDIR/module.prop" | grep -q -F "Enginex0"; then
    if [[ $ORIGIN == *"post-fs-data.sh"* ]]; then
      logE "环境异常,拦截执行"
      mv "$FSEEMODDIR/webroot" "$FSEEMODDIR/.webroot"
      mv "$FSEEMODDIR/action.sh" "$FSEEMODDIR/.action.sh"
    else
      exit
    fi
  else
    [[ "$ORIGIN" == *"post-fs-data.sh"* ]] && {
      logI "环境正常,继续执行"
      mv "$FSEEMODDIR/.webroot" "$FSEEMODDIR/webroot"
      if [[ ! "$APATCH" && ! "$KSU" ]]; then
        mv "$FSEEMODDIR/.action.sh" "$FSEEMODDIR/action.sh"
      else
        mv "$FSEEMODDIR/action.sh" "$FSEEMODDIR/.action.sh"
      fi
    }
  fi
}
invoke() {
  COMMAND=$1
  shift
  logI "$@"
  if fseed $COMMAND; then
    logI "完毕"
  else
    logW "失败"
  fi
}
##END##

[[ "$ORIGIN" == *"post-fs-data.sh"* ]] && {
  rm -f "$MULTIPLETYPE"
  rm -f "$KERNELTYPE"
  rm -f "$SINGLETYPE"
  rm -f "$FSEELOG"
}