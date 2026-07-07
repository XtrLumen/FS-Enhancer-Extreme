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
check

logI "启动后台服务"
fseed --fseectl -start
initwait
[[ $(fseed --fseectl -state) == "true" ]] || logE "服务启动失败"
logI "更新目标文件"
invoke --packagelistupdate
logI "卸载冲突软件"
invoke --conflictappcheck
logI "同步安全补丁级别到属性"
invoke --securitypatchpropsync
logI "伪装引导程序状态为锁定"
invoke --passpropstate
logI "修正已验证启动哈希属性"
invoke --passvbhash