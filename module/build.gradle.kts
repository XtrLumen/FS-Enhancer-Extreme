import java.util.TreeSet
import java.nio.ByteOrder
import java.nio.ByteBuffer
import java.security.Signature
import java.security.KeyFactory
import java.security.MessageDigest
import io.github.rctcwyvrn.blake3.Blake3
import java.security.spec.EdECPrivateKeySpec
import java.security.spec.NamedParameterSpec
import org.apache.tools.ant.filters.FixCrLfFilter

plugins {
    id("base")
}

buildscript {
    dependencies {
        classpath("io.github.rctcwyvrn:blake3:1.3")
    }
}

val moduleId: String by rootProject.extra
val moduleName: String by rootProject.extra
val verName: String by rootProject.extra
val verType: String by rootProject.extra
val verCode: Int by rootProject.extra
val verHash: String by rootProject.extra

listOf("debug", "release").forEach { variantName ->
    val variantLowered = variantName.lowercase()
    val variantCapped = variantName.replaceFirstChar { if (it.isLowerCase()) it.titlecase() else it.toString() }

    val moduleDir = layout.buildDirectory.dir("outputs/module/$variantLowered")
    val zipFileName = "$moduleName-$verName-$verCode-$verHash-$variantName.zip".replace(' ', '-')

    val prepareModuleFilesTask = tasks.register<Sync>("prepareModuleFiles$variantCapped") {
        group = "module"
        dependsOn(
            ":app:assemble$variantCapped",
            ":dex:compileDex$variantCapped"
        )
        outputs.upToDateWhen {false}
        into(moduleDir)
        from(project(":app").layout.buildDirectory.file("outputs/apk/$variantLowered")) {
            include(
                "app-$variantLowered.apk"
            )
            rename(
                "app-$variantLowered.apk",
                "service.apk"
            )
        }
        from(project(":dex").layout.buildDirectory.file("outputs/dex/$variantLowered")) {
            include(
                "classes.dex"
            )
            rename(
                "classes.dex",
                "service.dex"
            )
        }
        from("$projectDir/src") {
            include(
                "module.prop"
            )
            expand(
                "moduleId" to "$moduleId",
                "moduleName" to "$moduleName",
                "versionName" to "$verName$verType ($verCode-$verHash-$variantLowered)",
                "versionCode" to "$verCode"
            )
        }
        from("$projectDir/src") {
            exclude(
                ".DS_Store",
                "module.prop"
            )
        }
        from("${rootProject.projectDir}/README.md") {
            rename(
                "README.md",
                "README4en-US.md"
            )
        }
        from(
            "${rootProject.projectDir}/README4zh-Hans.md",
            "${rootProject.projectDir}/README4zh-Hant.md"
        )
        into("webroot") {
            from("${rootProject.projectDir}/webroot")
        }
    }

    val signModuleTask = tasks.register("signModule$variantCapped") {
        group = "module"
        dependsOn(prepareModuleFilesTask)

        val moduleOutputDir = moduleDir.get().asFile
        val publicKeyFile = File(project.projectDir, "public_key")
        val privateKeyFile = File(project.projectDir, "private_key")

        doLast {
            fun sha256Sum() {
                moduleOutputDir.walkTopDown().forEach { file ->
                    if (file.isDirectory) return@forEach
                    if (file.name.endsWith(".sha256")) return@forEach
                    val md = MessageDigest.getInstance("SHA3-256")
                    file.forEachBlock(4096) { bytes, size ->
                        md.update(bytes, 0, size)
                    }
                    File(file.path + ".sha256").writeText(md.digest().joinToString("") { "%02x".format(it) })
                }
            }

            if (privateKeyFile.exists()) {
                val publicKey = publicKeyFile.readBytes()
                val privateKey = privateKeyFile.readBytes()
                val sigType = Signature.getInstance("ed25519")
                fun mistySign() {
                    val set = TreeSet<File> { o1, o2 ->
                        o1.path.replace("\\", "/")
                            .compareTo(o2.path.replace("\\", "/"))
                    }

                    listOf(
                        "post-fs-data.sh",
                        "uninstall.sh",
                        "customize.sh",
                        "service.dex",
                        "service.apk",
                        "service.sh",
                        "banner.png",
                        "webui.apk",
                        "action.sh"
                    ).forEach { fileName ->
                        set.add(File(moduleOutputDir, fileName))
                    }

                    listOf("bin", "lib", "webroot").forEach { dirName ->
                        File(moduleOutputDir, dirName).walkTopDown().forEach { file ->
                            if (file.isFile) {
                                set.add(file)
                            }
                        }
                    }

                    set.forEach { file ->
                        val relativePath = file.absolutePath.substring(moduleOutputDir.absolutePath.length)
                        println(relativePath.replace("\\", "/"))
                    }

                    val BLAKE3Builder = StringBuilder()

                    set.forEach { file ->
                        val hasher = Blake3.newInstance()
                        val buffer = ByteArray(4096)
                        file.inputStream().use { input ->
                            var bytesRead: Int
                            while (input.read(buffer).also { bytesRead = it } != -1) {
                                if (bytesRead == buffer.size) {
                                    hasher.update(buffer)
                                } else {
                                    hasher.update(buffer.copyOf(bytesRead))
                                }
                            }
                        }
                        val fileHash = hasher.digest()
                        BLAKE3Builder.append(fileHash.joinToString("") { "%02x".format(it) })
                    }

                    val BLAKE3Hash = BLAKE3Builder.toString()

                    println(BLAKE3Hash)

                    sigType.initSign(KeyFactory.getInstance("ed25519").generatePrivate(EdECPrivateKeySpec(NamedParameterSpec("ed25519"), privateKey)))
                    sigType.update(BLAKE3Hash.toByteArray())

                    File(moduleOutputDir, "cirno").writeBytes(sigType.sign())

                    File(moduleOutputDir, "misty").writeBytes(publicKey)
                }

                mistySign()

                sha256Sum()

                println("=== Guards the peace of Misty Lake ===")
            } else {
                println("no private_key found, this build will not be signed")

                listOf(
                    "cirno",
                    "misty"
                ).forEach { emptyFile ->
                    File(moduleOutputDir, emptyFile).createNewFile()
                }

                sha256Sum()
            }
        }
    }

    tasks.register<Zip>("zip$variantCapped") {
        group = "module"
        dependsOn(signModuleTask)
        archiveFileName.set(zipFileName)
        destinationDirectory.set(layout.buildDirectory.file("outputs/$variantLowered").get().asFile)
        from(moduleDir)
    }
}

tasks.register("zip") {
    group = "module"
    dependsOn(
        "zipDebug",
        "zipRelease"
    )
}