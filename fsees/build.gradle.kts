tasks.register<Exec>("clean") {
    group = "rust"

    workingDir(projectDir)
    executable("cargo")
    args("clean")
}

listOf(
    "debug",
    "release"
).forEach { variantName ->
    val variantCapped = variantName.replaceFirstChar { if (it.isLowerCase()) it.titlecase() else it.toString() }
    val variantLowered = variantName.lowercase()

    tasks.register<Exec>("bins$variantCapped") {
        group = "rust"

        executable("cargo")
        args("ndk", "build", "--target", "aarch64-linux-android")
        if (variantLowered == "release") {
            args("--release")
        }
    }
}

tasks.register("bins") {
    group = "rust"

    dependsOn(
        "binsDebug",
        "binsRelease"
    )
}