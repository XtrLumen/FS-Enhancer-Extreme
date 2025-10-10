import java.util.TreeSet
import java.nio.ByteOrder
import java.nio.ByteBuffer
import java.security.Signature
import java.security.KeyFactory
import java.security.MessageDigest
import java.security.spec.EdECPrivateKeySpec
import java.security.spec.NamedParameterSpec
import org.apache.tools.ant.filters.FixCrLfFilter

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
        from("${rootProject.projectDir}/README4zh-Hans.md")
        from("${rootProject.projectDir}/README4zh-Hant.md")
        into("webroot") {
            from("${rootProject.projectDir}/webroot")
        }
    }

    val signModuleTask = tasks.register("signModule$variantCapped") {
        group = "module"
        dependsOn(prepareModuleFilesTask)
        
        val moduleOutputDir = moduleDir.get().asFile
        val privateKeyFile = File(project.projectDir, "private_key")
        val publicKeyFile = File(project.projectDir, "public_key")

        doLast {
            
            fun sha384Files() {
                moduleOutputDir.walkTopDown().forEach { file ->
                    if (file.isDirectory) return@forEach
                    if (file.name.endsWith(".sha384")) return@forEach
                    val md = MessageDigest.getInstance("SHA3-384")
                    file.forEachBlock(4096) { bytes, size ->
                        md.update(bytes, 0, size)
                    }
                    File(file.path + ".sha384").writeText(md.digest().joinToString("") { "%02x".format(it) })
                }
            }
            
            if (privateKeyFile.exists()) {
                val privateKey = privateKeyFile.readBytes()
                val publicKey = publicKeyFile.readBytes()
                val namedSpec = NamedParameterSpec("ed25519")
                val privKeySpec = EdECPrivateKeySpec(namedSpec, privateKey)
                val kf = KeyFactory.getInstance("ed25519")
                val privKey = kf.generatePrivate(privKeySpec);
                val sig = Signature.getInstance("ed25519")

                /* INFO:
                   bakacirno is the name of files that holds signed hash of all runtime files of TS Enhancer Extreme module, to ensure the runtime files hasn't been tampered with.
                */
                fun bakacirnoSign(name: String = "bakacirno") {
                    val set = TreeSet<File> { o1, o2 ->
                        o1.path.replace("\\", "/")
                            .compareTo(o2.path.replace("\\", "/"))
                    }

                    set.add(File(moduleOutputDir, "post-fs-data.sh"))
                    set.add(File(moduleOutputDir, "uninstall.sh"))
                    set.add(File(moduleOutputDir, "customize.sh"))
                    set.add(File(moduleOutputDir, "service.dex"))
                    set.add(File(moduleOutputDir, "service.apk"))
                    set.add(File(moduleOutputDir, "service.sh"))
                    set.add(File(moduleOutputDir, "banner.png"))
                    set.add(File(moduleOutputDir, "webui.apk"))
                    set.add(File(moduleOutputDir, "action.sh"))

                    File(moduleOutputDir, "libraries").walkTopDown().forEach { file ->
                        if (file.isFile) {
                            set.add(file)
                        }
                    }
                    File(moduleOutputDir, "binaries").walkTopDown().forEach { file ->
                        if (file.isFile) {
                            set.add(file)
                        }
                    }
                    File(moduleOutputDir, "webroot").walkTopDown().forEach { file ->
                        if (file.isFile) {
                            set.add(file)
                        }
                    }

                    val hashBuilder = StringBuilder()

                    set.forEach { file ->
                        val md = MessageDigest.getInstance("SHA3-512")
                        file.forEachBlock(4096) { bytes, size ->
                            md.update(bytes, 0, size)
                        }
                        val fileHash = md.digest()
                        hashBuilder.append(fileHash.joinToString("") { "%02x".format(it) })
                    }

                    println(hashBuilder.toString())

                    val signFile = File(moduleOutputDir, name)
                    signFile.writeText(hashBuilder.toString())
                }

                println("=== Guards the peace of Embodiment of Scarlet Devil ===")

                bakacirnoSign()

                sha384Files()
            } else {
                println("no private_key found, this build will not be signed")

                File(moduleOutputDir, "bakacirno").createNewFile()
                
                sha384Files()
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
    dependsOn("zipDebug", "zipRelease")
}