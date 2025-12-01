plugins {
    id("signing")
    id("maven-publish")
    id("java-library")
    id("io.github.gradle-nexus.publish-plugin") version "2.0.0"
}

java {
    group = "net.fornwall"
    version = "2025.0.0"
    sourceCompatibility = org.gradle.api.JavaVersion.VERSION_17
    withJavadocJar()
    withSourcesJar()
}

repositories {
    mavenCentral()
}

dependencies {
    testImplementation("org.junit.jupiter:junit-jupiter:5.14.1")
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")
}

tasks {
    test {
        useJUnitPlatform()
    }
}

// See https://docs.gradle.org/current/userguide/publishing_maven.html
// and https://github.com/igr/repo-url-parser/blob/master/build.gradle
//
// Setup signing credentials with as described in
// https://docs.gradle.org/current/userguide/signing_plugin.html
// with the following gradle properties:
//    signing.keyId=XXX
//    signing.password=YYY
//    signing.secretKeyRingFile=ZZZ
//
// Then publish to local (~/.m2/) and remote with:
//    ./gradlew publishToMavenLocal
//    ./gradlew -PsonatypeUsername=XXX -PsonatypePassword=YYY publishMavenJavaPublicationToMavenRepository
nexusPublishing {
    repositories {
        sonatype {
            nexusUrl.set(uri("https://ossrh-staging-api.central.sonatype.com/service/local/"))
            snapshotRepositoryUrl.set(uri("https://central.sonatype.com/repository/maven-snapshots/"))
        }
    }
}

publishing {
    publications {
        create<MavenPublication>("mavenJava") {
            from(components["java"])
            //artifactId = 'aoc'
            pom {
                name.set("AdventOfCode")
                description.set("Advent of Code solver")
                url.set("https://github.com/fornwall/advent-of-code")
                licenses {
                    license {
                        name.set("The MIT License")
                        url.set("https://opensource.org/licenses/MIT")
                    }
                }
                developers {
                    developer {
                        id.set("fornwall")
                        name.set("Fredrik Fornwall")
                        email.set("fredrik@fornwall.net")
                    }
                }
                scm {
                    connection.set("scm:git:git://github.com/fornwall/advent-of-code.git")
                    developerConnection.set("scm:git:ssh://git@github.com/fornwall/advent-of-code.git")
                    url.set("https://github.com/fornwall/advent-of-code/")
                }
            }
        }
    }
}

signing {
    sign(publishing.publications["mavenJava"])
}