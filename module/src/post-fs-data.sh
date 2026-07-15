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
logI "轮换日志结束"
logI "重置描述文件"
cp -f "$FSEEMODDIR/module.base" "$FSEEMODDIR/module.prop"
[ -f "$ADB/service.d/.fsee_state.sh" ] || {
  logI "配置描述文件刷新脚本"
  mkdir -p "$ADB/service.d"
  cp -f "$FSEEMODDIR/script/state.sh" "$ADB/service.d/.fsee_state.sh"
  chmod +x "$ADB/service.d/.fsee_state.sh"
}
logI "收集运行环境"
invoke envcollect
envcheck

logI "处理冲突模块"
invoke modcheck