-keep class io.github.xtrlumen.vbmeta.Entry
-assumenosideeffects class android.util.Log {
    public static int e(...);
}

-repackageclasses
-overloadaggressively
-allowaccessmodification