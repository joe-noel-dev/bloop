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

        // rustls-platform-verifier-android bundles a local Maven repo with the Android AAR that provides
        // the Kotlin/Java TrustManager bridge required for TLS certificate verification.
        // We locate it by searching the Cargo registry sources under CARGO_HOME so the path stays correct across machines.
        val cargoHome = providers.environmentVariable("CARGO_HOME")
            .orElse("${System.getProperty("user.home")}/.cargo")
        val rustlsAndroidMavenRepos = cargoHome.map { home ->
            java.io.File(home, "registry/src")
                .walkTopDown()
                .maxDepth(2)
                .filter { it.isDirectory && it.name.startsWith("rustls-platform-verifier-android-") }
                .mapNotNull { it.resolve("maven").takeIf { mavenDir -> mavenDir.exists() } }
                .toList()
        }
        rustlsAndroidMavenRepos.orNull?.forEach { repo ->
            maven { url = repo.toURI() }
        }

        mavenCentral()
    }
}

rootProject.name = "bloop-android"
include(":app")
