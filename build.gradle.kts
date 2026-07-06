val moduleId by extra("fs_enhancer_extreme")
val moduleName by extra("FS Enhancer Extreme")
val verName by extra("v1.0.0")
val verType by extra("-Dev")
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