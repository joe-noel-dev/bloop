#include <jni.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

// ---------------------------------------------------------------------------
// Forward declarations matching the Rust #[no_mangle] extern "C" exports in
// core/src/ffi/mod.rs
// ---------------------------------------------------------------------------

typedef struct BloopContext BloopContext;

typedef enum {
    BloopErrorCode_Success = 0,
    BloopErrorCode_InvalidRequest,
    BloopErrorCode_ErrorPostingRequest,
} BloopErrorCode;

typedef void (*BloopResponseCallback)(void *context, const uint8_t *data, size_t size);

extern BloopContext *bloop_init(BloopResponseCallback callback, void *context);
extern BloopErrorCode bloop_add_request(BloopContext *ctx, const uint8_t *request, size_t size);
extern void bloop_shutdown(BloopContext *ctx);

// ---------------------------------------------------------------------------
// JNI callback glue
//
// bloop_init takes a plain C function pointer.  When the Rust core calls that
// function it may be on any thread managed by the Tokio runtime, so we must
// attach the thread to the JVM before calling back into Kotlin.
// ---------------------------------------------------------------------------

typedef struct {
    JavaVM *jvm;
    jobject callback_obj;   // global ref to the Kotlin ResponseCallback instance
    jmethodID on_response;  // cached method ID for ResponseCallback.onResponse([B)V
} JniCallbackContext;

static void jni_response_callback(void *ctx, const uint8_t *data, size_t size) {
    JniCallbackContext *jni_ctx = (JniCallbackContext *)ctx;
    JNIEnv *env = NULL;
    int attached = 0;

    jint result = (*jni_ctx->jvm)->GetEnv(jni_ctx->jvm, (void **)&env, JNI_VERSION_1_6);
    if (result == JNI_EDETACHED) {
        (*jni_ctx->jvm)->AttachCurrentThread(jni_ctx->jvm, &env, NULL);
        attached = 1;
    }

    if (!env) {
        return;
    }

    jbyteArray byte_array = (*env)->NewByteArray(env, (jsize)size);
    if (byte_array) {
        (*env)->SetByteArrayRegion(env, byte_array, 0, (jsize)size, (const jbyte *)data);
        (*env)->CallVoidMethod(env, jni_ctx->callback_obj, jni_ctx->on_response, byte_array);
        (*env)->DeleteLocalRef(env, byte_array);
    }

    if (attached) {
        (*jni_ctx->jvm)->DetachCurrentThread(jni_ctx->jvm);
    }
}

// Holds both the opaque Rust context and the JNI callback resources so that
// shutdown can release everything in one shot.
typedef struct {
    BloopContext *bloop_ctx;
    JniCallbackContext *jni_callback;
} AndroidBloopContext;

// ---------------------------------------------------------------------------
// JNI entry points
// Class:  com.joenoel.bloop.core.BloopJNI
// ---------------------------------------------------------------------------

JNIEXPORT jlong JNICALL
Java_com_joenoel_bloop_core_BloopJNI_bloopInit(JNIEnv *env, jclass cls, jobject callback) {
    JavaVM *jvm = NULL;
    (*env)->GetJavaVM(env, &jvm);

    jclass callback_class = (*env)->GetObjectClass(env, callback);
    jmethodID on_response = (*env)->GetMethodID(env, callback_class, "onResponse", "([B)V");
    if (!on_response) {
        return 0L;
    }

    JniCallbackContext *jni_ctx = (JniCallbackContext *)malloc(sizeof(JniCallbackContext));
    if (!jni_ctx) {
        return 0L;
    }
    jni_ctx->jvm = jvm;
    jni_ctx->callback_obj = (*env)->NewGlobalRef(env, callback);
    jni_ctx->on_response = on_response;

    BloopContext *bloop_ctx = bloop_init(jni_response_callback, jni_ctx);
    if (!bloop_ctx) {
        (*env)->DeleteGlobalRef(env, jni_ctx->callback_obj);
        free(jni_ctx);
        return 0L;
    }

    AndroidBloopContext *android_ctx = (AndroidBloopContext *)malloc(sizeof(AndroidBloopContext));
    if (!android_ctx) {
        bloop_shutdown(bloop_ctx);
        (*env)->DeleteGlobalRef(env, jni_ctx->callback_obj);
        free(jni_ctx);
        return 0L;
    }
    android_ctx->bloop_ctx = bloop_ctx;
    android_ctx->jni_callback = jni_ctx;

    return (jlong)(intptr_t)android_ctx;
}

JNIEXPORT jint JNICALL
Java_com_joenoel_bloop_core_BloopJNI_bloopAddRequest(
        JNIEnv *env, jclass cls, jlong ctx_handle, jbyteArray request) {
    if (!ctx_handle) {
        return BloopErrorCode_ErrorPostingRequest;
    }

    AndroidBloopContext *android_ctx = (AndroidBloopContext *)(intptr_t)ctx_handle;
    jsize size = (*env)->GetArrayLength(env, request);
    jbyte *bytes = (*env)->GetByteArrayElements(env, request, NULL);

    BloopErrorCode code = bloop_add_request(
            android_ctx->bloop_ctx, (const uint8_t *)bytes, (size_t)size);

    (*env)->ReleaseByteArrayElements(env, request, bytes, JNI_ABORT);
    return (jint)code;
}

JNIEXPORT void JNICALL
Java_com_joenoel_bloop_core_BloopJNI_bloopShutdown(JNIEnv *env, jclass cls, jlong ctx_handle) {
    if (!ctx_handle) {
        return;
    }

    AndroidBloopContext *android_ctx = (AndroidBloopContext *)(intptr_t)ctx_handle;
    bloop_shutdown(android_ctx->bloop_ctx);
    (*env)->DeleteGlobalRef(env, android_ctx->jni_callback->callback_obj);
    free(android_ctx->jni_callback);
    free(android_ctx);
}
