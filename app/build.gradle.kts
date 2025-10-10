import com.android.build.gradle.internal.tasks.CompileArtProfileTask
import org.jetbrains.kotlin.gradle.dsl.KotlinVersion

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

dependencies {
    implementation("org.bouncycastle:bcprov-jdk18on:1.80")
}

val verName: String by rootProject.extra
val verCode: Int by rootProject.extra

android {
    namespace = "io.github.xtrlumen.vbmeta"
    buildToolsVersion = "35.0.1"
    compileSdk = 34
    defaultConfig {
        minSdk = 24
        targetSdk = 34
        versionCode = verCode
        versionName = verName
    }

    signingConfigs {
        getByName("debug") {
            val storeFileProp = properties["storeFile"] as String?
            if (storeFileProp != null) {
                storeFile = file(storeFileProp)
                storePassword = properties["storePassword"] as String?
                keyAlias = properties["keyAlias"] as String?
                keyPassword = properties["keyPassword"] as String?
            }
        }
    }
    
    buildTypes {
        debug {
            versionNameSuffix = "-debug"
        }
        release {
            isMinifyEnabled = true
            isShrinkResources = true
            vcsInfo.include = false
            signingConfig = signingConfigs.getByName("debug")
            proguardFiles("proguard-rules.pro")
        }
    }
    
    kotlin {
        jvmToolchain(21)
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_21
        targetCompatibility = JavaVersion.VERSION_21
    }

    buildFeatures {
        viewBinding = false
        buildConfig = true
        aidl = false
    }

    packaging {
        resources {
            excludes += setOf("**")
        }
    }

    androidResources {
        generateLocaleConfig = false
    }

    lint {
        checkReleaseBuilds = false
        abortOnError = false
        disable.add("AppCompatResource")
    }
    
    dependenciesInfo {
        includeInApk = false
    }
}

tasks.withType(CompileArtProfileTask::class.java).configureEach {
    enabled = false
}

configurations.configureEach {
    exclude(group = "org.jetbrains.kotlin", module = "kotlin-stdlib-jdk7")
    exclude(group = "org.jetbrains.kotlin", module = "kotlin-stdlib-jdk8")
}