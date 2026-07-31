import java.security.Signature
import java.security.KeyFactory
import java.security.MessageDigest
import java.security.spec.EdECPrivateKeySpec
import java.security.spec.NamedParameterSpec

import io.github.rctcwyvrn.blake3.Blake3

buildscript {
    dependencies {
        classpath("io.github.rctcwyvrn:blake3:1.3")
    }
}

plugins {
    id("base")
}

val moduleId:   String by rootProject.extra
val moduleName: String by rootProject.extra
val verName:    String by rootProject.extra
val verType:    String by rootProject.extra
val verCode:       Int by rootProject.extra
val verHash:    String by rootProject.extra

listOf(
    "debug",
    "release"
).forEach { variantName ->
    val variantCapped = variantName.replaceFirstChar { if (it.isLowerCase()) it.titlecase() else it.toString() }
    val variantLowered = variantName.lowercase()
    val moduleDir = layout.buildDirectory.dir("outputs/module/${variantLowered}")
    val moduleDirAsFile = moduleDir.get().asFile
    val zipFileName = "${moduleName}-${verName}-${verCode}-${verHash}-${variantName}.zip".replace(' ', '-')

    val prepareModuleFilesTask = tasks.register<Copy>("prepareModuleFiles${variantCapped}") {
        group = "module"
        description = "Prepares module files for ${variantName}."

        dependsOn(
            ":app:assemble${variantCapped}",
            ":fseed:bind${variantCapped}",
            ":fsees:bins${variantCapped}",
            ":fseeu:libu${variantCapped}"
        )

        doFirst {
            with(moduleDirAsFile) {
                deleteRecursively()
            }
        }

        into(moduleDir)
            from(project(":app").layout.buildDirectory.file("outputs/apk/${variantLowered}")) {
                include(
                    "app-${variantLowered}.apk"
                )
                rename(
                    "app-${variantLowered}.apk",
                    "provider.apk"
                )
            }
            from("${projectDir}/src") {
                include(
                    "module.base"
                )
                expand(
                    "moduleId" to "${moduleId}",
                    "moduleName" to "${moduleName}",
                    "versionName" to "${verName}${verType} (${verCode}-${verHash}-${variantLowered})",
                    "versionCode" to "${verCode}"
                )
            }
            from(moduleDir) {
                include(
                    "module.base"
                )
                rename(
                    "module.base",
                    "module.prop"
                )
            }
            from("${projectDir}/src") {
                exclude(
                    ".DS_Store",
                    "module.base"
                )
            }
            from(rootProject.file("README.md")) {
                rename(
                    "README.md",
                    "README4zh-Hans.md"
                )
            }
            from(
                rootProject.files(
                    "README4en-US.md"
                )
            )
        into("bin") {
            from(project(":fseed").file("target/aarch64-linux-android/${variantLowered}"))
            include("fseed")
            from(project(":fsees").file("target/aarch64-linux-android/${variantLowered}"))
            include("fsees")
        }
        into("lib") {
            from(project(":fseeu").file("target/aarch64-linux-android/${variantLowered}"))
            include("libutils.so")
        }
    }

    val signModuleFilesTask = tasks.register("signModule${variantCapped}") {
        group = "module"
        description = "Sign module files for ${variantName}."

        dependsOn(prepareModuleFilesTask)

        doFirst {
            fun sha256Sum() {
                fileTree(moduleDir) {
                    exclude("MANIFEST")
                }.visit {
                    if (isDirectory) return@visit

                    val mdInstance = MessageDigest.getInstance("SHA3-256")
                    file.forEachBlock(4096) { bytes, size ->
                        mdInstance.update(bytes, 0, size)
                    }

                    val sha256File = File(moduleDirAsFile, "MANIFEST/${file.relativeTo(moduleDirAsFile)}.sha256")
                    sha256File.parentFile.mkdirs()

                    sha256File.writeText(
                        mdInstance.digest().joinToString("") {
                            "%02x".format(it)
                        }
                    )
                }
            }

            val mistyFile = File(moduleDirAsFile, "mistylake")
            val privateKeyFile = project.file("private_key")
            if (privateKeyFile.exists()) {
                fun mistylakeSign() {
                    val BLAKE3Builder = StringBuilder()

                    listOf(
                        "bin/fseed",
                        "bin/fsees",
                        "lib/libutils.so",
                        "script/state.sh",
                        "script/util_functions.sh",
                        "action.sh",
                        "module.base",
                        "post-fs-data.sh",
                        "provider.apk",
                        "service.sh",
                        "uninstall.sh"
                    ).forEach {
                        println(it)

                        val mdInstance = Blake3.newInstance()
                        mdInstance.update(File(moduleDirAsFile, it))

                        BLAKE3Builder.append(
                            mdInstance.hexdigest()
                        )
                    }

                    val BLAKE3Hash = BLAKE3Builder.toString()

                    println(BLAKE3Hash)

                    val privateKeyBytes = privateKeyFile.readBytes()
                    val publicKeyBytes = project.file("public_key").readBytes()

                    val signInstance = Signature.getInstance("ed25519")
                    signInstance.initSign(
                        KeyFactory.getInstance("ed25519").generatePrivate(EdECPrivateKeySpec(NamedParameterSpec("ed25519"), privateKeyBytes))
                    )
                    signInstance.update(
                        BLAKE3Hash.toByteArray()
                    )

                    val finalSignBytes = signInstance.sign()

                    mistyFile.writeBytes(finalSignBytes.copyOfRange(0, 16))
                    mistyFile.appendBytes(publicKeyBytes.copyOfRange(0, 16))
                    mistyFile.appendBytes(finalSignBytes.copyOfRange(16, 48))
                    mistyFile.appendBytes(publicKeyBytes.copyOfRange(16, 32))
                    mistyFile.appendBytes(finalSignBytes.copyOfRange(48, 64))
                }

                mistylakeSign()

                sha256Sum()

                println("=== Guards the peace of Misty Lake ===")
            } else {
                println("no private_key found, this build will not be signed")

                mistyFile.createNewFile()

                sha256Sum()
            }
        }
    }

    val zipTask = tasks.register<Zip>("zip${variantCapped}") {
        group = "module"
        description = "Create module zip for ${variantCapped}."

        dependsOn(signModuleFilesTask)

        archiveFileName.set(zipFileName)
        destinationDirectory.set(layout.buildDirectory.file("outputs/${variantLowered}").get().asFile)
        from(moduleDir)
    }

    val pushTask = tasks.register<Exec>("push${variantCapped}") {
        group = "module"
        description = "Push module to device."

        dependsOn(zipTask)

        commandLine("adb", "push", zipTask.get().archiveFile.get().asFile, "/data/local/tmp")
    }

    tasks.register<Exec>("magisk${variantCapped}") {
        group = "module"
        description = "Installs module via Magisk."

        dependsOn(pushTask)

        commandLine("adb", "shell", "su", "-c", "magisk --install-module /data/local/tmp/${zipFileName}")
    }

    tasks.register<Exec>("ksud${variantCapped}") {
        group = "module"
        description = "Installs module via KernelSU."

        dependsOn(pushTask)

        commandLine("adb", "shell", "su", "-c", "ksud module install /data/local/tmp/${zipFileName}")
    }

    tasks.register<Exec>("apd${variantCapped}") {
        group = "module"
        description = "Installs module via APatch."

        dependsOn(pushTask)

        commandLine("adb", "shell", "su", "-c", "apd module install /data/local/tmp/${zipFileName}")
    }
}

tasks.register("zip") {
    group = "module"
    description = "Create module zip for Github Release."

    dependsOn(
        "zipDebug",
        "zipRelease"
    )
}