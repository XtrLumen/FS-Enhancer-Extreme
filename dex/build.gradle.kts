plugins {
    id("com.android.application")
}

val verName: String by rootProject.extra
val verCode: Int by rootProject.extra

android {
    namespace = "ts.enhancer.xtr"
    compileSdk = 35
    defaultConfig {
        minSdk = 24
        targetSdk = 34
        versionCode = verCode
        versionName = verName
        multiDexEnabled = false
    }

    buildTypes {
        getByName("release") {
            isMinifyEnabled = true
            vcsInfo.include = false
            proguardFiles("proguard-rules.pro")
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_21
        targetCompatibility = JavaVersion.VERSION_21
    }

    lint {
        checkReleaseBuilds = false
    }
}

listOf("Debug", "Release").forEach { variantName ->
    val variantCapped = variantName.replaceFirstChar { if (it.isLowerCase()) it.titlecase() else it.toString() }
    val variantLowered = variantName.lowercase()

    tasks.register<Copy>("compileDex$variantCapped") {
        group = "build"
        
        dependsOn(
            ":dex:assemble$variantCapped"
        )

        into(layout.buildDirectory.dir("outputs/dex/$variantLowered"))
        from("$projectDir/build/intermediates/dex/$variantLowered/minifyReleaseWithR8") {
            include(
                "classes.dex"
            )
            rename(
                "classes.dex",
                "service.dex"
            )
        }
    }
}

tasks.register("compileDex") {
    group = "build"

    dependsOn(
        "compileDexDebug",
        "compileDexRelease"
    )
}