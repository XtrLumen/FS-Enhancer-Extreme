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

cd ${0%/*}
source "./libraries/util_functions.sh"
check

logs "启动后台服务"
$TSEEBIN/tseed --tseectl -start
initwait
[[ $($TSEEBIN/tseed --tseectl -state) == "true" ]] || logs "服务启动失败"
invoke "调用安全级别同步参数" "--securitypatchpropsync"
invoke "调用冲突模块排除参数" "--conflictmodcheck -b"
invoke "调用包名列表更新参数" "--packagelistupdate"
invoke "调用冲突软件排除参数" "--conflictappcheck"
invoke "调用引导属性修正参数" "--passpropstate"
invoke "调用安全启动修正参数" "--passvbhash"