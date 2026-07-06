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
fseed --rootdetect
check

invoke --conflictmodcheck "移除冲突模块"
[ -f "$SD/.fsee_state.sh" ] || {
  logI "复制状态检测脚本到自启文件夹"
  mkdir -p "$SD"
  cp -f "$FSEEMODDIR/script/state.sh" "$SD/.fsee_state.sh"
  chmod +x "$SD/.fsee_state.sh"
}