plugins {
    id("com.android.application")
}

android {
    namespace = "ts.enhancer.xtr"
    compileSdk = 34
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
}

dependencies {
    implementation("io.github.rctcwyvrn:blake3:1.3")
}

val variants = listOf("Debug", "Release")

variants.forEach { variantCapped ->
    val variantLowered = variantCapped.lowercase()

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
                println("Copied classes.dex to $dst")
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