plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("org.jetbrains.kotlin.plugin.compose")
}

android {
    namespace = "com.yume"
    compileSdk = 35

    defaultConfig {
        applicationId = "com.yume"
        minSdk = 26
        targetSdk = 35
        versionCode = 1
        versionName = "0.1.0"
    }

    buildTypes {
        release {
            isMinifyEnabled = false
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlin {
        compilerOptions {
            jvmTarget.set(org.jetbrains.kotlin.gradle.dsl.JvmTarget.JVM_17)
        }
    }

    buildFeatures {
        compose = true
    }
}

dependencies {
    val composeBom = platform("androidx.compose:compose-bom:2024.06.00")
    implementation(composeBom)
    implementation("androidx.compose.ui:ui")
    implementation("androidx.compose.ui:ui-graphics")
    implementation("androidx.compose.ui:ui-tooling-preview")
    implementation("androidx.compose.material3:material3")
    implementation("androidx.compose.material:material-icons-extended")

    implementation("androidx.activity:activity-compose:1.9.0")
    implementation("androidx.lifecycle:lifecycle-viewmodel-compose:2.8.0")
    implementation("androidx.lifecycle:lifecycle-runtime-compose:2.8.0")

    // Networking (fallback when native library not available)
    implementation("com.squareup.okhttp3:okhttp:4.12.0")
    implementation("com.squareup.okhttp3:okhttp-sse:4.12.0")
    implementation("com.google.code.gson:gson:2.11.0")

    // JNA — needed for UniFFI-generated Kotlin bindings
    implementation("net.java.dev.jna:jna:5.15.0@aar")

    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-android:1.8.0")
    implementation("androidx.core:core-ktx:1.13.0")

    debugImplementation("androidx.compose.ui:ui-tooling")
}

// Copy native libraries from Rust build into jniLibs.
// Run `just ffi-android` first, then build the APK.
// The jniLibs directory is pre-created with .gitkeep.
tasks.register<Copy>("copyNativeLibs") {
    from("${rootProject.projectDir}/../target") {
        include("*/release/libyume_ffi.so")
    }
    into("src/main/jniLibs")
    eachFile {
        val srcPath = path
        path = when {
            srcPath.contains("aarch64") -> "arm64-v8a/libyume_ffi.so"
            srcPath.contains("armv7")   -> "armeabi-v7a/libyume_ffi.so"
            srcPath.contains("i686")    -> "x86/libyume_ffi.so"
            srcPath.contains("x86_64")  -> "x86_64/libyume_ffi.so"
            else                        -> return@eachFile
        }
    }
}
