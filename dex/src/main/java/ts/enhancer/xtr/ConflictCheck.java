/*
 * This file is part of TS-Enhancer-Extreme.
 *
 * TS-Enhancer-Extreme is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 *
 * TS-Enhancer-Extreme is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 * without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with TS-Enhancer-Extreme.  If not, see <https://www.gnu.org/licenses/>.
 *
 * Copyright (C) 2025 TheGeniusClub (Organization)
 * Copyright (C) 2025 XtrLumen (Developer)
 */

package ts.enhancer.xtr;

import java.io.File;
import java.nio.file.*;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

import static java.nio.file.StandardWatchEventKinds.*;

public class ConflictCheck implements Runnable {

    private static final String LOG_MODE;
    private static final File MONITOR_FILE;
    private static final Path MONITOR_PATH;
    private static final boolean IMG_EXISTS;
    private static final String ARG_TYPE;
    private static final long LOG_SUPPRESS_MS = 1500L;
    private static final Map<String, Long> LAST_LOG_TIMES = new ConcurrentHashMap<>();

    static {
        File apImg = new File("/data/adb/ap/modules.img");
        File ksuImg = new File("/data/adb/ksu/modules.img");
        if (apImg.exists() || ksuImg.exists()) {
            MONITOR_FILE = new File("/data/adb/modules");
            LOG_MODE = "触发执行冲突模块检查-OverlayFS";
            IMG_EXISTS = true;
            ARG_TYPE = "-o";
        } else {
            MONITOR_FILE = new File("/data/adb/modules_update");
            LOG_MODE = "触发执行冲突模块检查-MagicMount";
            IMG_EXISTS = false;
            ARG_TYPE = "-m";
        }
        MONITOR_PATH = MONITOR_FILE.toPath();
    }

    @Override
    public void run() {
        WatchService watcher = null;
        try {
            watcher = FileSystems.getDefault().newWatchService();
            MONITOR_PATH.register(watcher, ENTRY_CREATE, ENTRY_MODIFY, ENTRY_DELETE);

            while (true) {
                WatchKey key = watcher.take();
                boolean changed = false;

                for (WatchEvent<?> event : key.pollEvents()) {
                    changed = true;
                }

                if (changed) {
                    logOnce(LOG_MODE);

                    try {
                        ProcessBuilder pb = new ProcessBuilder("/data/adb/modules/ts_enhancer_extreme/binaries/tseed", "--conflictmodcheck", ARG_TYPE);
                        Process p = pb.start();
                        p.waitFor();
                    } catch (Exception ignored) {}
                }

                if (!key.reset()) {
                    break;
                }
            }
        } catch (Exception ignored) {
        } finally {
            if (watcher != null) {
                try {
                    watcher.close();
                } catch (Exception ignored) {}
            }
        }
    }

    private static void logOnce(String msg) {
        long now = System.currentTimeMillis();
        Long last = LAST_LOG_TIMES.get(msg);
        if (last == null || now - last >= LOG_SUPPRESS_MS) {
            Logger.logout(msg);
            LAST_LOG_TIMES.put(msg, now);
        }
    }
}