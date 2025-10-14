val moduleId by extra("ts_enhancer_extreme")
val moduleName by extra("TS Enhancer Extreme")
val verName by extra("v0.8.3")
val verType by extra("-Beta")
val verCode by extra(
    providers.exec { 
        commandLine("git", "rev-list", "HEAD", "--count") 
    }.standardOutput.asText.get().trim().toInt() + 52
)
val verHash by extra(
    providers.exec { 
        commandLine("git", "rev-parse", "--verify", "--short", "HEAD") 
    }.standardOutput.asText.get().trim()
)