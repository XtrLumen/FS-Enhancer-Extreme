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

rm -rf /data/adb/fs_enhancer_extreme/
rm -f /data/adb/service.d/.fsee_state.sh
find /data/adb/service.d -empty -delete
rm -f /data/adb/modules/tricky_store/action.sh
[ -d /data/adb/tricky_store ] && {
  cp -f /data/adb/tricky_store/keybox_backup/keybox.xml /data/adb/tricky_store/keybox.xml
  rm -rf /data/adb/tricky_store/keybox_backup
  [ ! -f /data/adb/tricky_store/keybox.xml ] && cp -f /data/adb/fs_enhancer_extreme/keybox.xml /data/adb/tricky_store/keybox.xml
}