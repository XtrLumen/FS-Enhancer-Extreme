pluginManagement {
    repositories {
        google()
        mavenCentral()
    }
    plugins {
        id("com.android.application") version "8.7.3"
    }
}

dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
        google()
        mavenCentral()
    }
}

rootProject.name = "FS-Enhancer-Extreme"
include(
    ":app",
    ":fseed",
    ":fsees",
    ":fseev",
    ":module"
)