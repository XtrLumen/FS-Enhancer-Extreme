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

    tasks.register<Exec>("libu$variantCapped") {
        group = "rust"

        executable("cargo")
        args("ndk", "--target", "aarch64-linux-android", "build")
        if (variantLowered == "release") {
            args("--release")
        }
    }
}

tasks.register("libu") {
    group = "rust"

    dependsOn(
        "libuDebug",
        "libuRelease"
    )
}