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
FSEEMODDIR="$ADB/modules/fs_enhancer_extreme"
FSEECONFIGDIR="$ADB/fs_enhancer_extreme"
#TWO LEVEL#
FSEECONFIG="$FSEECONFIGDIR/config"
OLDLOG="$FSEECONFIGDIR/log.old"
LOGDIR="$FSEECONFIGDIR/log"
FSEELOG="$LOGDIR/log.log"
#OTHER#
isPostFsData=false
isServiceD=false
isService=false
logID="<Undefined>"
case "$(basename "$0")" in
  *"post-fs-data.sh"*)
    isPostFsData=true
    logID="<Post-Fs-Data>"
    ;;
  *".fsee_state.sh"*)
    isServiceD=true
    logID="<Service.D>"
    ;;
  *"service.sh"*)
    isService=true
    logID="<Service>"
    ;;
esac
##END##

##FUNCTIONS##
#MULTILINGUAL#
if [[ "$(getprop persist.sys.locale)" == *"zh"* || "$(getprop ro.product.locale)" == *"zh"* ]]; then
  LOCALE="CN"
else
  LOCALE="EN"
fi
println() {
  [ "$LOCALE" = "$1" ] && {
    shift
    echo "$@"
  }
}
print_cn() {
  println "CN" "$@"
}
print_en() {
  println "EN" "$@"
}
#OTHER#
fseed() {
  $FSEEMODDIR/bin/fseed "$@"
}
logout() {
  LEVEL=$1
  shift
  echo "$(date "+%m-%d %H:%M:%S.$(date +%3N)")  $$  $$ $LEVEL [FSEE]: $logID $@" >> "$FSEELOG"
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
  if fseed --isEnvironmentNormal; then
    if $isPostFsData; then
      logI "环境正常,继续执行"
      mv -f "$FSEEMODDIR/.webroot" "$FSEEMODDIR/webroot"
      if [[ ! "$APATCH" && ! "$KSU" ]]; then
        mv -f "$FSEEMODDIR/.action.sh" "$FSEEMODDIR/action.sh"
      else
        mv -f "$FSEEMODDIR/action.sh" "$FSEEMODDIR/.action.sh"
      fi
    fi
  else
    if $isPostFsData; then
      logE "环境异常,拦截执行"
      mv -f "$FSEEMODDIR/webroot" "$FSEEMODDIR/.webroot"
      mv -f "$FSEEMODDIR/action.sh" "$FSEEMODDIR/.action.sh"
    else
      exit
    fi
  fi
}
invoke() {
  if fseed "$@"; then
    logI "完毕"
  else
    logW "失败"
  fi
}
##END##