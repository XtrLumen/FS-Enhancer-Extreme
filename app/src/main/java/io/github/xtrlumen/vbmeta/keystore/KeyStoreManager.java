package io.github.xtrlumen.vbmeta.keystore;

public class KeyStoreManager {
    private static IAndroidKeyStore remoteKeyStore;

    public static IAndroidKeyStore getRemoteKeyStore() {
        return remoteKeyStore;
    }

    public static boolean isShizukuInstalled() {
        return false;
    }

    public static void requestPermission() {
    }

    public static void requestBinder(android.content.Context context) {
    }
}
