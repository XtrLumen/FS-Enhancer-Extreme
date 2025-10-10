pluginManagement {
    repositories {
        google()
        mavenCentral()
    }
    plugins {
        id("com.android.application") version "8.8.0"
        id("org.jetbrains.kotlin.android") version "2.1.0"
    }
}

rootProject.name = "TS-Enhancer-Extreme"
include(":app")
include(":dex")
include(":module")