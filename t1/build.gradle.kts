import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    kotlin("jvm") version "1.9.23"

}


group = "me.singsongzep_"
version = "1.0-SNAPSHOT"

dependencies {
    implementation(project(":jvm"))
    testImplementation(kotlin("test"))
}

tasks.test {
    useJUnit()
}

tasks.withType<KotlinCompile> {
    kotlinOptions.jvmTarget = "1.8"
}

application {
    mainClass.set("MainKt")
}

repositories {
    mavenCentral()
}