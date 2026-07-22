tasks.register<Exec>("clean") {
    group = "rust"

    workingDir(projectDir)
    executable("./cargopp")
    args("clean")
}

listOf(
    "debug",
    "release"
).forEach { variantName ->
    val variantCapped = variantName.replaceFirstChar { if (it.isLowerCase()) it.titlecase() else it.toString() }
    val variantLowered = variantName.lowercase()

    tasks.register<Exec>("bind$variantCapped") {
        group = "rust"

        executable("./cargopp")
        if (variantLowered == "release") {
            args("buildRelease")
        } else {
            args("buildDebug")
        }
    }
}

tasks.register("bind") {
    group = "rust"

    dependsOn(
        "bindDebug",
        "bindRelease"
    )
}