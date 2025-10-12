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

import java.io.*;
import java.util.*;

public class Logger {
    private static final File LOG_FILE = new File("/data/adb/ts_enhancer_extreme/log/log.log");
    private static final String prefix = " I System.out: [TSEE]<DEX>";

    public static synchronized void logout(String message) {
        try (BufferedWriter writer = new BufferedWriter(new OutputStreamWriter(
            new FileOutputStream(LOG_FILE, true), "UTF-8"))) {
            long t = System.currentTimeMillis();
            Calendar c = Calendar.getInstance();
            c.setTimeInMillis(t);
            String time = String.format("%02d-%02d %02d:%02d:%02d.%03d  %d  %d",
                c.get(Calendar.MONTH) + 1,
                c.get(Calendar.DAY_OF_MONTH),
                c.get(Calendar.HOUR_OF_DAY),
                c.get(Calendar.MINUTE),
                c.get(Calendar.SECOND),
                (int) (t % 1000),
                getPid(),
                getNativeTid()
            );
            writer.write(time + prefix + message);
            writer.newLine();
        } catch (Exception ignored) {
        }
    }

    private static long getNativeTid() {
        try {
            Class<?> processClass = Class.forName("android.os.Process");
            return (Integer) processClass.getMethod("myTid").invoke(null);
        } catch (Throwable e) {
            return Thread.currentThread().hashCode();
        }
    }

    private static int getPid() {
        try {
            Class<?> processClass = Class.forName("android.os.Process");
            return (Integer) processClass.getMethod("myPid").invoke(null);
        } catch (Throwable e) {
            try {
                java.io.BufferedReader reader = new java.io.BufferedReader(
                    new java.io.FileReader("/proc/self/stat"));
                String line = reader.readLine();
                reader.close();
                if (line != null) {
                    String[] parts = line.split(" ");
                    if (parts.length > 0) {
                        return Integer.parseInt(parts[0]);
                    }
                }
            } catch (Exception ex) {
                return -1;
            }
            return -1;
        }
    }
}