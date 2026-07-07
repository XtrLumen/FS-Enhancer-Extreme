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

cd ${0%/*}
source "./script/util_functions.sh"
rm   -rf "$OLDLOG"
mv    -f "$LOGDIR" "$OLDLOG"
mkdir -p "$LOGDIR"
touch    "$FSEELOG"
logI "轮换日志目录结束"
logI "清理临时文件"
rm -f "$FSEECONFIG/multiple.txt"
rm -f "$FSEECONFIG/kernel.txt"
rm -f "$FSEECONFIG/root.txt"
fseed --rootdetect
check

logI "移除冲突模块"
invoke --conflictmodcheck
if [ ! -f "$ADB/service.d/.fsee_state.sh" ]; then
  logI "配置状态检测脚本"
  mkdir -p "$ADB/service.d"
  cp -f "$FSEEMODDIR/script/state.sh" "$ADB/service.d/.fsee_state.sh"
  chmod +x "$ADB/service.d/.fsee_state.sh"
fi