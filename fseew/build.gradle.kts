tasks.register<Delete>("clean") {
    group = "web"

    delete("dist")
}

tasks.register<Exec>("build") {
    group = "web"

    executable("npm")
    args("run", "build")
}