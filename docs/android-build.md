# Android Build & Test Guide

## Prerequisites

- Android Studio (or Android SDK CLI)
- JDK 17
- Android SDK API 35 + Build Tools
- Emulator (API 35 recommended) or physical device

## Quick Start

```bash
# Set up Android SDK location (if not in ANDROID_HOME)
echo "sdk.dir=/Users/$USER/Library/Android/sdk" > android/local.properties

# Build APK
just apk-debug

# Install on device/emulator
just apk-install

# Build + install + launch
just apk-run
```

## Manual Commands

```bash
# Build debug APK
cd android && ./gradlew assembleDebug

# APK location
android/app/build/outputs/apk/debug/app-debug.apk

# Install on emulator
adb install -r android/app/build/outputs/apk/debug/app-debug.apk

# Launch app
adb shell am start -n com.yume/.MainActivity

# Watch logs
adb logcat -s "AndroidRuntime:E" "com.yume:*"

# Force stop
adb shell am force-stop com.yume
```

## Testing on Emulator

```bash
# 1. Start the Yume backend (from project root)
just dev
# Or: docker compose -f docker-compose.dev.yml up -d

# 2. Verify backend is running
curl http://localhost:3000/health
# → {"status":"ok","version":"0.1.0","environment":"development"}

# 3. Build and install APK
just apk-run

# 4. The app connects to http://10.0.2.2:3000 (emulator → host localhost)
# 5. Type a message and send — you should see streaming AI response
```

## Testing on Physical Device

```bash
# 1. Find device IP
adb shell ip route

# 2. Update ApiClient baseUrl in:
# android/app/src/main/java/com/yume/network/ApiClient.kt
# Change from http://10.0.2.2:3000 to http://<HOST_IP>:3000

# 3. Rebuild and install
just apk-run
```

## Troubleshooting

### "SDK location not found"
Create `android/local.properties`:
```
sdk.dir=/path/to/Android/sdk
```

### "LocalLifecycleOwner not present" crash
Fixed in Swing 0.2 — replaced `collectAsStateWithLifecycle()` with `collectAsState()`.

### App crashes on send
Backend not running. Start with `just dev` or `docker compose -f docker-compose.dev.yml up -d`.

### Emulator can't reach backend
- Emulator uses `10.0.2.2` to reach host localhost
- Verify backend listens on `0.0.0.0:3000`
- Check no firewall blocking port 3000
