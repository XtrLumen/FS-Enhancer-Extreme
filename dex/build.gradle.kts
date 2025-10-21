plugins {
    id("com.android.application")
}

android {
    namespace = "ts.enhancer.xtr"
    compileSdk = 35
    defaultConfig {
        minSdk = 24
        targetSdk = 34
        multiDexEnabled = false
    }

    buildTypes {
        getByName("release") {
            isMinifyEnabled = false
            proguardFiles("proguard-rules.pro")
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_21
        targetCompatibility = JavaVersion.VERSION_21
    }

    lint {
        abortOnError = false
        checkReleaseBuilds = false
    }
}

listOf("Debug", "Release").forEach { variantName ->
    val variantCapped = variantName.replaceFirstChar { if (it.isLowerCase()) it.titlecase() else it.toString() }
    val variantLowered = variantName.lowercase()

    tasks.register("compileDex$variantCapped") {
        group = "build"
        
        dependsOn(
            "desugar${variantCapped}FileDependencies",
            "compile${variantCapped}JavaWithJavac",
            "dexBuilder${variantCapped}",
            "mergeExtDex${variantCapped}",
            "mergeDex${variantCapped}"
        )

        doLast {
            val src = file(layout.buildDirectory.dir("intermediates/dex/$variantLowered/mergeDex$variantCapped/classes.dex").get())
            val dst = file(layout.buildDirectory.dir("outputs/dex/$variantLowered/classes.dex").get())

            if (src.exists()) {
                dst.parentFile.mkdirs()
                src.copyTo(dst, overwrite = true)
            }
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