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

##FUNCTIONS##
#MULTILINGUAL#
if [[ "$(getprop persist.sys.locale)" == *"zh"* || "$(getprop ro.product.locale)" == *"zh"* ]]; then
  LOCALE="CN"
else
  LOCALE="EN"
fi
operate() {
  [ "${LOCALE}" = "${1}" ] && {
    shift
    local operation="${1}"
    shift
    case "${operation}" in
      ui_print)
        ui_print "${@}"
        ;;
      abort_verify)
        ui_print "***********************************************"
        ui_print "! ${@}"
        print_cn "! 这个ZIP文件已损坏,请重新下载"
        print_en "! This zip may be corrupted, please try downloading again"
        abort    "***********************************************"
        ;;
    esac
  }
}
print_cn() {
  operate "CN" "ui_print" "${@}"
}
print_en() {
  operate "EN" "ui_print" "${@}"
}
abort_cn() {
  operate "CN" "abort_verify" "${@}"
}
abort_en() {
  operate "EN" "abort_verify" "${@}"
}
##END##

##VARIABLE##
#PUBLIC#
SKIPUNZIP=1
#ZERO LEVEL#
ADB="/data/adb"
#ONE LEVEL#
MODULESDIR="${ADB}/modules"
#TWO LEVEL#
FSEECONFIG="${ADB}/fs_enhancer_extreme/config"
#CHECK ENVIRONMENT#
MIN_RELEASE=10
RELEASE=$(grep_get_prop ro.build.version.release)
#PRINT INFORMATION#
MODULE_VER=$(grep_prop version "${TMPDIR}/module.prop")
#EXTRACT MODULE FILES#
FILES="
bin/*
lib/*
script/*
action.sh
mistylake
module.base
module.prop
post-fs-data.sh
provider.apk
service.sh
uninstall.sh
"
#POST PROCESS#
NES="
${MODPATH}/bin/fseed
${MODPATH}/bin/fsees
"
SYS="
com.android.vending
com.google.android.gsf
com.google.android.gms
"
USR="
me.bmax.apatch
com.android.patch
me.garfieldhan.apatch.next
"
##END##

##PRE PROCESS##
#CHECK INTEGRITY#
unzip -o "${ZIPFILE}" 'verify.sh' -d "${TMPDIR}" >/dev/null
[ -f "${TMPDIR}/verify.sh" ] || {
  abort_cn "无法提取 verify.sh!"
  abort_en "Unable to extract verify.sh"
}
source "${TMPDIR}/verify.sh"
#CHECK ENVIRONMENT#
[ ${BOOTMODE} ] || {
  ui_print "***********************************************"
  print_cn "! 不受支持的安装环境 Recovery"
  print_cn "! 请从 KernelSU, APatch 或 Magisk 应用安装"
  print_en "! Install from recovery is not supported"
  print_en "! Please install from KernelSU, APatch or Magisk app"
  abort    "***********************************************"
}
[ ${RELEASE} -lt ${MIN_RELEASE} ] && {
  ui_print "***********************************************"
  print_cn "! 不受支持的安卓版本 ${RELEASE}"
  print_cn "! 最低支持的安卓版本 ${MIN_RELEASE}"
  print_en "! Unsupported android version: ${RELEASE}"
  print_en "! Minimal supported android version is ${MIN_RELEASE}"
  abort    "***********************************************"
}
[ -f "${ADB}/.overlayfs_enable" ] || {
  [ -f "${ADB}/ksu/mount_system" ] && cat "${ADB}/ksu/mount_system" | grep -q "OVERLAYFS"
} && {
  ui_print "***********************************************"
  print_cn "! 不受支持的挂载系统 OverlayFS"
  print_cn "! 由于冲突模块排除功能在此模式无法正常工作"
  print_cn "! 请切换到 Magic Mount 挂载系统或元模块挂载系统后再次安装"
  print_en "! Unsupported mount system: OverlayFS"
  print_en "! Conflict module exclusion cannot work in this mode"
  print_en "! Please switch to Magic Mount mount system or Meta Module mount system before installing again"
  abort    "***********************************************"
}
#PRINT INFORMATION#
if [ "${KSU}" ]; then
  print_cn "- KernelSU版本号: ${KSU_KERNEL_VER_CODE}(kernel) ${KSU_VER_CODE}(ksud)"
  print_cn "- KernelSU版本: ${KSU_VER}"
  print_en "- KernelSU version code: ${KSU_KERNEL_VER_CODE}(kernel) ${KSU_VER_CODE}(ksud)"
  print_en "- KernelSU version: ${KSU_VER}"
elif [ "${APATCH}" ]; then
  print_cn "- APatch版本号: ${APATCH_VER_CODE}"
  print_cn "- APatch版本: ${APATCH_VER}"
  print_en "- APatch version code: ${APATCH_VER_CODE}"
  print_en "- APatch version: ${APATCH_VER}"
elif [ "${MAGISK_VER}" ]; then
  print_cn "- Magisk版本号: ${MAGISK_VER_CODE}"
  print_cn "- Magisk版本: ${MAGISK_VER}"
  print_en "- Magisk version code: ${MAGISK_VER_CODE}"
  print_en "- Magisk version: ${MAGISK_VER}"
fi
print_cn "- 正在安装模块: FS-Enhancer-Extreme ${MODULE_VER}"
print_en "- Install module FS-Enhancer-Extreme ${MODULE_VER}"
sleep 1s
#DELETE OLD FILES#
print_cn "- 删除旧版文件"
print_en "- Delete older version files"
rm -f "${ADB}/service.d/.fsee_state.sh"
##END##

##EXTRACT MODULE FILES##
print_cn "- 提取模块文件"
print_en "- Extracting module files"
for FILE in ${FILES}; do
  extract "${ZIPFILE}" "${FILE}" "${MODPATH}"
done
##END##

##POST PROCESS##
print_cn "- 赋予必要权限"
print_en "- Setting permission"
chcon u:object_r:shell_data_file:s0 "${MODPATH}/provider.apk"
for NE in ${NES}; do
  chmod +x "${NE}"
done
mkdir -p "${FSEECONFIG}"
print_cn "- 提取密钥文件"
print_en "- Extract keybox file"
extract "${ZIPFILE}" 'keybox.xml' "${FSEECONFIG}"
if [ ! -f "${FSEECONFIG}/usr.txt" ] || [ ! -f "${FSEECONFIG}/sys.txt" ]; then
  print_cn "- 创建排除列表"
  print_en "- Extract default exclusion list"
  [ -f "${FSEECONFIG}/sys.txt" ] || {
    echo "$SYS" | grep -v '^$' > "${FSEECONFIG}/sys.txt"
  }
  [ -f "${FSEECONFIG}/usr.txt" ] || {
    echo "$USR" | grep -v '^$' > "${FSEECONFIG}/usr.txt"
  }
fi
[[ "$(grep_get_prop ro.product.brand)" == "OnePlus" ]] && {
  grep -qx "com.oplus.engineermode" "${FSEECONFIG}/sys.txt" || echo "com.oplus.engineermode" >> "${FSEECONFIG}/sys.txt"
  grep -qx "com.coloros.sceneservice" "${FSEECONFIG}/sys.txt" || echo "com.coloros.sceneservice" >> "${FSEECONFIG}/sys.txt"
}
##END##

print_cn "- 安装完毕"
print_en "- Install Done"