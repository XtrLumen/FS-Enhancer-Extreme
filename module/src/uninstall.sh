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

rm -rf /data/adb/ts_enhancer_extreme/
rm -f /data/adb/service.d/.tsee_state.sh
find /data/adb/service.d -empty -delete
rm -f /data/adb/modules/tricky_store/action.sh
[ -d /data/adb/tricky_store ] && {
  mv /data/adb/tricky_store/config_backup /data/adb/tsconfig_backup
  rm -rf /data/adb/tricky_store/
  mv /data/adb/tsconfig_backup /data/adb/tricky_store
  [ ! -f /data/adb/tricky_store/keybox.xml ] && cat /data/adb/ts_enhancer_extreme/keybox.xml > /data/adb/tricky_store/keybox.xml
}