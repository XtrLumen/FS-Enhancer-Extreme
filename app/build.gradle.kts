plugins {
    id("com.android.application")
}

dependencies {
    compileOnly("androidx.annotation:annotation:1.9.1")
    implementation("com.google.guava:guava:33.4.0-android")
    implementation("org.bouncycastle:bcprov-jdk18on:1.80")
    implementation("co.nstant.in:cbor:0.9")
}

val verName: String by rootProject.extra
val verCode: Int by rootProject.extra

android {
    namespace = "io.github.xtrlumen.vbmeta"
    buildToolsVersion = "36.0.0"
    compileSdk = 36
    defaultConfig {
        minSdk = 29
        targetSdk = 29
        versionCode = verCode
        versionName = verName
    }

    buildFeatures {
        buildConfig = true
    }

    buildTypes {
        debug {
            versionNameSuffix = "-d"
        }
        release {
            isMinifyEnabled = true
            vcsInfo.include = false
            proguardFiles("proguard-rules.pro")
            signingConfig = android.signingConfigs.getByName("debug")
        }
    }

    packaging {
        resources {
            excludes += setOf(
                "org/**"
            )
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_21
        targetCompatibility = JavaVersion.VERSION_21
    }

    lint {
        checkReleaseBuilds = false
    }

    dependenciesInfo {
        includeInApk = false
    }

    tasks.withType<JavaCompile> {
        options.compilerArgs.add("-Xlint:deprecation")
        options.compilerArgs.add("-Xlint:unchecked")
        options.compilerArgs.add("-Xdiags:verbose")
    }
}
