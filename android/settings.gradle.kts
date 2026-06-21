pluginManagement {
    repositories {
        google()
        mavenCentral()
        gradlePluginPortal()
    }
}
plugins {
    id("org.gradle.toolchains.foojay-resolver-convention") version "0.10.0"
}

dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
        google()
        mavenCentral()

        // rustls-platform-verifier bundles a local Maven repo with the Android AAR that provides
        // the Kotlin/Java TrustManager bridge required for TLS certificate verification.
        // We locate it via cargo metadata so the path stays correct across machines.
        val cargoHome = providers.environmentVariable("CARGO_HOME")
            .orElse("${System.getProperty("user.home")}/.cargo")
        val rustlsAndroidMaven = cargoHome.map { home ->
            java.io.File(home, "registry/src")
                .walkTopDown()
                .maxDepth(2)
                .firstOrNull { it.isDirectory && it.name.startsWith("rustls-platform-verifier-android-") }
                ?.resolve("maven")
                ?.takeIf { it.exists() }
                ?.toURI()
                ?.toString()
        }
        rustlsAndroidMaven.orNull?.let { maven { url = uri(it) } }
    }
}

rootProject.name = "bloop-android"
include(":app")
