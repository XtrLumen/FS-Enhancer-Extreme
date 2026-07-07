plugins {
    id("com.android.application")
}

dependencies {
    implementation("org.bouncycastle:bcprov-jdk18on:1.80")
    implementation("com.google.guava:guava:33.4.0-android")
    implementation("co.nstant.in:cbor:0.9")
    compileOnly("androidx.annotation:annotation:1.9.1")
}

val verName: String by rootProject.extra
val verCode: Int by rootProject.extra

android {
    namespace = "io.github.xtrlumen.vbmeta"
    buildToolsVersion = "35.0.1"
    compileSdk = 35
    defaultConfig {
        minSdk = 29
        targetSdk = 34
        versionCode = verCode
        versionName = verName
    }

    buildFeatures {
        aidl = true
        buildConfig = true
    }

    buildTypes {
        release {
            isMinifyEnabled = true
            vcsInfo.include = false
            proguardFiles("proguard-rules.pro")
            signingConfig = android.signingConfigs.getByName("debug")
        }
    }

    packaging {
        resources {
            excludes += setOf("**")
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
