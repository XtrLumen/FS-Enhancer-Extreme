val moduleId by extra("fs_enhancer_extreme")
val moduleName by extra("FS Enhancer Extreme")
val verName by extra("v1.0.0")
val verType by extra("-t")
val verCode by extra(
    providers.exec {
        commandLine("git", "rev-list", "HEAD", "--count")
    }.standardOutput.asText.get().trim().toInt()
)
val verHash by extra(
    providers.exec {
        commandLine("git", "rev-parse", "--verify", "--short", "HEAD")
    }.standardOutput.asText.get().trim()
)

tasks.register<Exec>("clean") {
    group = "rust"

    workingDir(projectDir)
    executable("cargo")
    args("clean")
}
listOf(
    project(":fseec"),
    project(":fsees"),
    project(":fseeu")
).forEach { project ->
    listOf(
        "debug",
        "release"
    ).forEach { variantName ->
        val variantCapped = variantName.replaceFirstChar {
            if (it.isLowerCase()) it.titlecase() else it.toString()
        }
        val variantLowered = variantName.lowercase()

        project.tasks.register<Exec>("build${variantCapped}") {
            group = "rust"

            environment("CARGO_TERM_COLOR", "always")
            executable("cargo").args("ndk", "build", "--target", "aarch64-linux-android")
            if (variantLowered == "release") {
                args("--release")
            }
        }
    }
}

val fseew = project(":fseew")
fseew.tasks.register<Delete>("clean") {
    group = "web"

    delete("dist")
}
listOf(
    "debug",
    "release"
).forEach { variantName ->
    val variantCapped = variantName.replaceFirstChar {
        if (it.isLowerCase()) it.titlecase() else it.toString()
    }
    val variantLowered = variantName.lowercase()

    fseew.tasks.register<Exec>("build${variantCapped}") {
        group = "web"

        environment("FORCE_COLOR", "1")
        executable("npm").args("run", "build")
        if (variantLowered == "debug") {
            args("--", "--mode", "development")
        }
    }
}