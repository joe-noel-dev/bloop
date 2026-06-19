cmake_minimum_required(VERSION 3.22.1)
project("bloopjni")

# Prebuilt libbloop.so is staged by scripts/build-android.sh into
# <workspace-root>/target/android/jniLibs/<abi>/libbloop.so.
# This CMakeLists.txt lives at android/app/src/main/cpp/, which is five
# directory levels below the workspace root.
set(BLOOP_JNILIBS_DIR ${CMAKE_SOURCE_DIR}/../../../../../target/android/jniLibs)

add_library(bloop SHARED IMPORTED)
set_target_properties(bloop PROPERTIES
    IMPORTED_LOCATION ${BLOOP_JNILIBS_DIR}/${ANDROID_ABI}/libbloop.so
    IMPORTED_NO_SONAME TRUE)

add_library(bloopjni SHARED bloop_jni.c)
target_link_libraries(bloopjni bloop log)
