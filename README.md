# workbench-android
Workbench project for running a `NativeActivity` in C# that calls a rust library using `winit`



## Instructions

### Add build targets

64-bit ARM (ARMv7)

```
rustup target add aarch64-linux-android armv7-linux-androideabi
```

For running an Android emulator on x64:

```
rustup target add x86_64-linux-android
```



### Install cargo ndk

```
cargo install cargo-ndk
```



### Install Android NDK

Install NDK in Androids SDK Manager (in Visual Studio)

Run powershell as administrator and set PATH variable

```
[Environment]::SetEnvironmentVariable("ANDROID_NDK_HOME", "C:\Program Files (x86)\Android\android-sdk\ndk-bundle", "Machine")
```

This might need a restart to take affect. Set variable in local session:

```
$env:ANDROID_NDK_HOME = [Environment]::GetEnvironmentVariable("ANDROID_NDK_HOME", "Machine")
```



### Build rust library

Under `crates\workbench_bs_android\` execute:

```
cargo ndk -t aarch64-linux-android -P 21 build --release
```

or for the x64 emulator

```
cargo ndk -t x86_64-linux-android -P 21 build --release
```



## Problem

winit EventLoop does not receive touch events. Resize and keyboard events are working.

Found in device logs:

`InputDispatcher	Not sending touch gesture to 37aacfc ActivityRecordInputSink com.benginestudios.WorkbenchBR/android.app.NativeActivity because it has config NO_INPUT_CHANNEL`

```
Time	Device Name	Type	PID	Tag	Message
02-12 08:56:41.731	Samsung SM-X710	Warning	2669	InputDispatcher	Not sending touch gesture to 37aacfc ActivityRecordInputSink com.benginestudios.WorkbenchBR/android.app.NativeActivity because it has config NO_INPUT_CHANNEL

Time	Device Name	Type	PID	Tag	Message
02-12 08:56:35.828	Samsung SM-X710	Warning	2669	InputDispatcher	index=10, TouchWindow=37aacfc ActivityRecordInputSink com.benginestudios.WorkbenchBR/android.app.NativeActivity, touchableRegion=[-14400,-25600][17600,25600], inputConfig=NO_INPUT_CHANNEL | NOT_FOCUSABLE, index=9, FocusWindow=349e408 com.benginestudios.WorkbenchBR/android.app.NativeActivity, touchableRegion=<empty>, inputConfig=0x0
```

`Error 2669  InputDispatcher Error while processing MotionEvent for [Gesture Monitor] secinputdev`

`frame=[1600,0][1600,0]`

```
      9: name=349e408 com.benginestudios.WorkbenchBR/android.app.NativeActivity, id=784, displayId=0, inputConfig=0x0, samsungFlags=0x0, alpha=1, frame=[1600,0][1600,0], globalScale=1, applicationInfo.name=ActivityRecord{213164506 u0 com.benginestudios.WorkbenchBR/android.app.NativeActivity t1037}, applicationInfo.token=0xb400007a08733800, touchableRegion=<empty>, pTouchableRegion=<empty>, 1HXY=[0, 0, 0], ownerPid=3982, ownerUid=10444, dispatchingTimeout=10000ms, token=0xb400007a0a21da40, touchOcclusionMode=BLOCK_UNTRUSTED, canOccludePresentation
      transform (ROT_270) (ROTATE TRANSLATE)
          0.0000  1.0000  0.0000
          -1.0000  0.0000  1600.0000
          0.0000  0.0000  1.0000
          
      10: name=37aacfc ActivityRecordInputSink com.benginestudios.WorkbenchBR/android.app.NativeActivity, id=778, displayId=0, inputConfig=NO_INPUT_CHANNEL | NOT_FOCUSABLE, samsungFlags=0x0, alpha=1, frame=[1600,0][1600,0], globalScale=0, applicationInfo.name=, applicationInfo.token=<null>, touchableRegion=[-14400,-25600][17600,25600], pTouchableRegion=<empty>, 1HXY=[0, 0, 0], ownerPid=2669, ownerUid=1000, dispatchingTimeout=0ms, token=0x0, touchOcclusionMode=BLOCK_UNTRUSTED
      transform (ROT_270) (ROTATE TRANSLATE)
          0.0000  1.0000  0.0000
          -1.0000  0.0000  1600.0000
          0.0000  0.0000  1.0000
```



```
Time	Device Name	Type	PID	Tag	Message
02-12 08:56:06.431	Samsung SM-X710	Error	2669	InputDispatcher	Error while processing MotionEvent for [Gesture Monitor] secinputdev
  DispatchEnabled: true
  DispatchFrozen: false
  InputFilterEnabled: false
  FocusedDisplayId: 0
  FocusedApplications:
    displayId=0, name='ActivityRecord{213164506 u0 com.benginestudios.WorkbenchBR/android.app.NativeActivity t1037}', dispatchingTimeout=10000ms
  FocusedWindows:
    displayId=0, name='349e408 com.benginestudios.WorkbenchBR/android.app.NativeActivity'
  FocusRequests:
    displayId=0, name='349e408 com.benginestudios.WorkbenchBR/android.app.NativeActivity' result='OK'
  Pointer Capture Requested: false
  Current Window with Pointer Capture: None
  TouchStatesByDisplay:
    0 :     Windows:
        0 : name='[Gesture Monitor] secinputdev', targetFlags=SPLIT, forwardingWindowToken=0x0, mDeviceStates=7:[touchingPointers=[Pointer(id=0, FINGER)], downTimeInTarget=6358305510000, hoveringPointers=[], pilferingPointerIds=<none>]
        1 : name='[Gesture Monitor] PalmMotion', targetFlags=SPLIT, forwardingWindowToken=0x0, mDeviceStates=7:[touchingPointers=[Pointer(id=0, FINGER)], downTimeInTarget=6358305510000, hoveringPointers=[], pilferingPointerIds=<none>]
  CursorStatesByDisplay: <no displays touched by cursor>
  Display: 0
    logicalSize=2560x1600
        transform (ROT_270) (ROTATE TRANSLATE)
            0.0000  1.0000  0.0000
            -1.0000  0.0000  1600.0000
            0.0000  0.0000  1.0000
    Windows:
      0: name=Embedded{View root of ReusableWindowDecorViewHost#0}, id=63, displayId=0, inputConfig=NOT_VISIBLE | NOT_FOCUSABLE | TRUSTED_OVERLAY, samsungFlags=0x0, alpha=1, frame=[1600,0][1600,0], globalScale=0, applicationInfo.name=, applicationInfo.token=<null>, touchableRegion=[-48400,-50000][51600,50000], pTouchableRegion=<empty>, 1HXY=[0, 0, 0], ownerPid=3391, ownerUid=10055, dispatchingTimeout=10000ms, token=0xb400007a12653590, touchOcclusionMode=BLOCK_UNTRUSTED
      transform (ROT_270) (ROTATE TRANSLATE)
          0.0000  1.0000  0.0000
          -1.0000  0.0000  1600.0000
          0.0000  0.0000  1.0000
      1: name=stylus-handwriting-event-receiver-0, id=142, displayId=0, inputConfig=NOT_FOCUSABLE | NOT_TOUCHABLE | TRUSTED_OVERLAY | SPY | INTERCEPTS_STYLUS, samsungFlags=0x0, alpha=1, frame=[1600,0][1600,0], globalScale=1, applicationInfo.name=stylus-handwriting-event-receiver-0, applicationInfo.token=<null>, touchableRegion=[-14400,-25600][17600,25600], pTouchableRegion=<empty>, 1HXY=[0, 0, 0], ownerPid=2669, ownerUid=1000, dispatchingTimeout=10000ms, token=0xb4000078ab4f3f00, touchOcclusionMode=BLOCK_UNTRUSTED
      transform (ROT_270) (ROTATE TRANSLATE)
          0.0000  1.0000  0.0000
          -1.0000  0.0000  1600.0000
          0.0000  0.0000  1.0000
      2: name=[Gesture Monitor] secinputdev, id=770, displayId=0, inputConfig=NOT_FOCUSABLE | TRUSTED_OVERLAY | SPY, samsungFlags=0x0, alpha=1, frame=[1600,0][1600,0], globalScale=1, applicationInfo.name=[Gesture Monitor] secinputdev, applicationInfo.token=<null>, touchableRegion=[-14400,-25600][17600,25600], pTouchableRegion=<empty>, 1HXY=[0, 0, 0], ownerPid=2669, ownerUid=1000, dispatchingTimeout=10000ms, token=0xb400007a088896b0, touchOcclusionMode=BLOCK_UNTRUSTED
      transform (ROT_270) (ROTATE TRANSLATE)
          0.0000  1.0000  0.0000
          -1.0000  0.0000  1600.0000
          0.0000  0.0000  1.0000
      3: name=[Gesture Monitor] PalmMotion, id=110, displayId=0, inputConfig=NOT_FOCUSABLE | TRUSTED_OVERLAY | SPY, samsungFlags=0x0, alpha=1, frame=[1600,0][1600,0], globalScale=1, applicationInfo.name=[Gesture Monitor] PalmMotion, applicationInfo.token=<null>, touchableRegion=[-14400,-25600][17600,25600], pTouchableRegion=<empty>, 1HXY=[0, 0, 0], ownerPid=2669, ownerUid=1000, dispatchingTimeout=10000ms, token=0xb40000795eb7e280, touchOcclusionMode=BLOCK_UNTRUSTED
      transform (ROT_270) (ROTATE TRANSLATE)
          0.0000  1.0000  0.0000
          -1.0000  0.0000  1600.0000
          0.0000  0.0000  1.0000
      4: name=7fc68b2 TaskbarWindow, id=140, displayId=0, inputConfig=NOT_FOCUSABLE | TRUSTED_OVERLAY | WATCH_OUTSIDE_TOUCH | SLIPPERY, samsungFlags=0x0, alpha=1, frame=[0,0][102,2560], globalScale=1, applicationInfo.name=, applicationInfo.token=<null>, touchableRegion=[0,0][102,2560], pTouchableRegion=<empty>, 1HXY=[0, 0, 0], ownerPid=3857, ownerUid=10123, dispatchingTimeout=10000ms, token=0xb40000795ed0eef0, touchOcclusionMode=BLOCK_UNTRUSTED
      transform (ROT_270) (ROTATE TRANSLATE)
          0.0000  1.0000  0.0000
          -1.0000  0.0000  102.0000
          0.0000  0.0000  1.0000
      5: name=d6abd46 com.sec.android.app.launcher/com.samsung.app.honeyspace.edge.edgepanel.app.CocktailBarService, id=757, displayId=0, inputConfig=NOT_FOCUSABLE | TRUSTED_OVERLAY, samsungFlags=FORCE_TRUSTED_OVERLAY, alpha=1, frame=[1010,2493][1175,2560], globalScale=1, applicationInfo.name=, applicationInfo.token=<null>, touchableRegion=[1010,2525][1175,2560], pTouchableRegion=<empty>, 1HXY=[0, 0, 0], ownerPid=3857, ownerUid=10123, dispatchingTimeout=10000ms, token=0xb400007a087c5140, touchOcclusionMode=BLOCK_UNTRUSTED
      transform (ROT_270) (ROTATE TRANSLATE)
          0.0000  1.0000  -2493.0000
          -1.0000  0.0000  1175.0000
          0.0000  0.0000  1.0000
      6: name=d0fb81e StatusBar, id=105, displayId=0, inputConfig=NOT_VISIBLE | NOT_FOCUSABLE | TRUSTED_OVERLAY, samsungFlags=0x0, alpha=1, frame=[1600,0][1664,2560], globalScale=1, applicationInfo.name=, applicationInfo.token=<null>, touchableRegion=[1600,0][1664,2560], pTouchableRegion=<empty>, 1HXY=[0, 0, 0], ownerPid=3391, ownerUid=10055, dispatchingTimeout=10000ms, token=0xb400007a0880ee40, touchOcclusionMode=BLOCK_UNTRUSTED
      transform (ROT_270) (ROTATE TRANSLATE)
          0.0000  1.0000  0.0000
          -1.0000  0.0000  1664.0000
          0.0000  0.0000  1.0000
      7: name=cd559c RecentsTransitionOverlay, id=135, displayId=0, inputConfig=NO_INPUT_CHANNEL | NOT_VISIBLE | NOT_FOCUSABLE | NOT_TOUCHABLE | TRUSTED_OVERLAY, samsungFlags=0x0, alpha=1, frame=[1599,0][1600,1], globalScale=1, applicationInfo.name=, applicationInfo.token=<null>, touchableRegion=<empty>, pTouchableRegion=<empty>, 1HXY=[0, 0, 0], ownerPid=3857, ownerUid=10123, dispatchingTimeout=0ms, token=0x0, touchOcclusionMode=BLOCK_UNTRUSTED
      transform (ROT_270) (ROTATE TRANSLATE)
          0.0000  1.0000  0.0000
          -1.0000  0.0000  1600.0000
          0.0000  0.0000  1.0000
      8: name=Embedded{View root of ReusableWindowDecorViewHost#1}, id=66, displayId=0, inputConfig=NOT_VISIBLE | NOT_FOCUSABLE | TRUSTED_OVERLAY, samsungFlags=0x0, alpha=1, frame=[1538,1175][1598,1388], globalScale=0, applicationInfo.name=, applicationInfo.token=<null>, touchableRegion=[1538,1175][1598,1388], pTouchableRegion=<empty>, 1HXY=[0, 0, 0], ownerPid=3391, ownerUid=10055, dispatchingTimeout=10000ms, token=0xb400007a122af960, touchOcclusionMode=BLOCK_UNTRUSTED
      transform (ROT_270) (ROTATE TRANSLATE)
          0.0000  1.0000  -1175.0000
          -1.0000  0.0000  1598.0000
          0.0000  0.0000  1.0000
      9: name=349e408 com.benginestudios.WorkbenchBR/android.app.NativeActivity, id=784, displayId=0, inputConfig=0x0, samsungFlags=0x0, alpha=1, frame=[1600,0][1600,0], globalScale=1, applicationInfo.name=ActivityRecord{213164506 u0 com.benginestudios.WorkbenchBR/android.app.NativeActivity t1037}, applicationInfo.token=0xb400007a08733800, touchableRegion=<empty>, pTouchableRegion=<empty>, 1HXY=[0, 0, 0], ownerPid=3982, ownerUid=10444, dispatchingTimeout=10000ms, token=0xb400007a0a21da40, touchOcclusionMode=BLOCK_UNTRUSTED, canOccludePresentation
      transform (ROT_270) (ROTATE TRANSLATE)
          0.0000  1.0000  0.0000
          -1.0000  0.0000  1600.0000
          0.0000  0.0000  1.0000
      10: name=37aacfc ActivityRecordInputSink com.benginestudios.WorkbenchBR/android.app.NativeActivity, id=778, displayId=0, inputConfig=NO_INPUT_CHANNEL | NOT_FOCUSABLE, samsungFlags=0x0, alpha=1, frame=[1600,0][1600,0], globalScale=0, applicationInfo.name=, applicationInfo.token=<null>, touchableRegion=[-14400,-25600][17600,25600], pTouchableRegion=<empty>, 1HXY=[0, 0, 0], ownerPid=2669, ownerUid=1000, dispatchingTimeout=0ms, token=0x0, touchOcclusionMode=BLOCK_UNTRUSTED
      transform (ROT_270) (ROTATE TRANSLATE)
          0.0000  1.0000  0.0000
          -1.0000  0.0000  1600.0000
          0.0000  0.0000  1.0000
      11: name=523b504 ActivityRecordInputSink com.sec.android.app.launcher/com.android.quickstep.RecentsActivity, id=646, displayId=0, inputConfig=NO_INPUT_CHANNEL | NOT_VISIBLE | NOT_FOCUSABLE | NOT_TOUCHABLE, samsungFlags=0x0, alpha=1, frame=[1600,0][1600,0], globalScale=0, applicationInfo.name=, applicationInfo.token=<null>, touchableRegion=[-14400,-25600][17600,25600], pTouchableRegion=<empty>, 1HXY=[0, 0, 0], ownerPid=2669, ownerUid=1000, dispatchingTimeout=0ms, token=0x0, touchOcclusionMode=BLOCK_UNTRUSTED
      transform (ROT_270) (ROTATE TRANSLATE)
          0.0000  1.0000  0.0000
          -1.0000  0.0000  1600.0000
          0.0000  0.0000  1.0000
      12: name=Dim layer#70, id=70, displayId=0, inputConfig=NO_INPUT_CHANNEL | NOT_VISIBLE, samsungFlags=0x0, alpha=1, frame=[1600,0][1600,0], globalScale=1, applicationInfo.name=, applicationInfo.token=<null>, touchableRegion=<empty>, pTouchableRegion=<empty>, 1HXY=[0, 0, 0], ownerPid=3391, ownerUid=10055, dispatchingTimeout=5000ms, token=0x0, touchOcclusionMode=BLOCK_UNTRUSTED
      transform (ROT_270) (ROTATE TRANSLATE)
          0.0000  1.0000  0.0000
          -1.0000  0.0000  1600.0000
          0.0000  0.0000  1.0000
      13: name=Dim layer#69, id=69, displayId=0, inputConfig=NO_INPUT_CHANNEL | NOT_VISIBLE, samsungFlags=0x0, alpha=1, frame=[1600,0][1600,0], globalScale=1, applicationInfo.name=, applicationInfo.token=<null>, touchableRegion=<empty>, pTouchableRegion=<empty>, 1HXY=[0, 0, 0], ownerPid=3391, ownerUid=10055, dispatchingTimeout=5000ms, token=0x0, touchOcclusionMode=BLOCK_UNTRUSTED
      transform (ROT_270) (ROTATE TRANSLATE)
          0.0000  1.0000  0.0000
          -1.0000  0.0000  1600.0000
          0.0000  0.0000  1.0000
      14: name=Dim layer#68, id=68, displayId=0, inputConfig=NO_INPUT_CHANNEL | NOT_VISIBLE, samsungFlags=0x0, alpha=1, frame=[1600,0][1600,0], globalScale=1, applicationInfo.name=, applicationInfo.token=<null>, touchableRegion=<empty>, pTouchableRegion=<empty>, 1HXY=[5.75478, -1.19211e-07, 2.66247e-44], ownerPid=3391, ownerUid=10055, dispatchingTimeout=5000ms, token=0x0, touchOcclusionMode=BLOCK_UNTRUSTED
      transform (ROT_270) (ROTATE TRANSLATE)
          0.0000  1.0000  0.0000
          -1.0000  0.0000  1600.0000
          0.0000  0.0000  1.0000
      15: name=Split Background Layer#67, id=67, displayId=0, inputConfig=NO_INPUT_CHANNEL | NOT_VISIBLE, samsungFlags=0x0, alpha=1, frame=[0,0][1600,2560], globalScale=1, applicationInfo.name=, applicationInfo.token=<null>, touchableRegion=<empty>, pTouchableRegion=<empty>, 1HXY=[0, 0, 0], ownerPid=3391, ownerUid=10055, dispatchingTimeout=5000ms, token=0x0, touchOcclusionMode=BLOCK_UNTRUSTED
      transform (ROT_270) (ROTATE TRANSLATE)
          0.0000  1.0000  0.0000
          -1.0000  0.0000  1600.0000
          0.0000  0.0000  1.0000
      16: name=34fed ActivityRecordInputSink com.sec.android.app.launcher/.Launcher, id=131, displayId=0, inputConfig=NO_INPUT_CHANNEL | NOT_VISIBLE | NOT_FOCUSABLE | NOT_TOUCHABLE, samsungFlags=0x0, alpha=1, frame=[1600,0][1600,0], globalScale=0, applicationInfo.name=, applicationInfo.token=<null>, touchableRegion=[-14400,-25600][17600,25600], pTouchableRegion=<empty>, 1HXY=[0, 0, 0], ownerPid=2669, ownerUid=1000, dispatchingTimeout=0ms, token=0x0, touchOcclusionMode=BLOCK_UNTRUSTED
      transform (ROT_270) (ROTATE TRANSLATE)
          0.0000  1.0000  0.0000
          -1.0000  0.0000  1600.0000
          0.0000  0.0000  1.0000
      17: name=Wallpaper BBQ wrapper 5_system#89, id=89, displayId=0, inputConfig=NO_INPUT_CHANNEL | NOT_VISIBLE, samsungFlags=0x0, alpha=1, frame=[0,0][1600,2560], globalScale=1, applicationInfo.name=, applicationInfo.token=<null>, touchableRegion=<empty>, pTouchableRegion=<empty>, 1HXY=[0, 0, 0], ownerPid=3391, ownerUid=10055, dispatchingTimeout=5000ms, token=0x0, touchOcclusionMode=BLOCK_UNTRUSTED
      transform (ROT_270) (ROTATE TRANSLATE)
          0.0000  1.0000  0.0000
          -1.0000  0.0000  1600.0000
          0.0000  0.0000  1.0000
      18: name=64827a3 com.android.systemui.wallpapers.ImageWallpaper, id=88, displayId=0, inputConfig=NOT_VISIBLE | NOT_FOCUSABLE | NOT_TOUCHABLE | IS_WALLPAPER, samsungFlags=0x0, alpha=1, frame=[1600,0][1600,0], globalScale=1, applicationInfo.name=, applicationInfo.token=<null>, touchableRegion=<empty>, pTouchableRegion=<empty>, 1HXY=[0, 0, 0], ownerPid=3391, ownerUid=10055, dispatchingTimeout=10000ms, token=0xb400007a122b12b0, touchOcclusionMode=BLOCK_UNTRUSTED
      transform (ROT_270) (ROTATE TRANSLATE)
          0.0000  1.0000  0.0000
          -1.0000  0.0000  1600.0000
          0.0000  0.0000  1.0000
      19: name=Wallpaper BBQ wrapper 6_lock#83, id=83, displayId=0, inputConfig=NO_INPUT_CHANNEL | NOT_VISIBLE, samsungFlags=0x0, alpha=0, frame=[0,0][1600,2560], globalScale=1, applicationInfo.name=, applicationInfo.token=<null>, touchableRegion=<empty>, pTouchableRegion=<empty>, 1HXY=[0, 0, 0], ownerPid=3391, ownerUid=10055, dispatchingTimeout=5000ms, token=0x0, touchOcclusionMode=BLOCK_UNTRUSTED
      transform (ROT_270) (ROTATE TRANSLATE)
          0.0000  1.0000  0.0000
          -1.0000  0.0000  1600.0000
          0.0000  0.0000  1.0000
      20: name=25cde4f com.android.systemui.wallpapers.ImageWallpaper, id=82, displayId=0, inputConfig=NOT_VISIBLE | NOT_FOCUSABLE | NOT_TOUCHABLE | IS_WALLPAPER, samsungFlags=0x0, alpha=0, frame=[0,0][1600,2560], globalScale=1, applicationInfo.name=, applicationInfo.token=<null>, touchableRegion=[0,0][1600,2560], pTouchableRegion=<empty>, 1HXY=[0, 0, 0], ownerPid=3391, ownerUid=10055, dispatchingTimeout=10000ms, token=0xb400007a086d07a0, touchOcclusionMode=BLOCK_UNTRUSTED
      transform (ROT_270) (ROTATE TRANSLATE)
          0.0000  1.0000  0.0000
          -1.0000  0.0000  1600.0000
          0.0000  0.0000  1.0000
  mMaximumObscuringOpacityForTouch: 0.800000
  DisplayTopologyGraph:
    PrimaryDisplayId: 0
    TopologyGraph:
    
    DisplaysDensity:
      displayId(0):340
  
  Global monitors on display 0:
    0: 'PointerEventDispatcher0', 
    1: 'SpenInputDetector', 
  Connections:
    1089: channelName='349e408 com.benginestudios.WorkbenchBR/android.app.NativeActivity', status=NORMAL, monitor=false, responsive=true
    1087: channelName='5a3b5bc Additional view container of Task=1037', status=NORMAL, monitor=false, responsive=true
    966: channelName='[Gesture Monitor] secinputdev', status=NORMAL, monitor=false, responsive=true
      InputState: mMotionMementos: {deviceId=7, hovering=0, downTime=6358305510000}, 
    970: channelName='8dba4dc KeyguardBiometricToastView', status=NORMAL, monitor=false, responsive=true
    947: channelName='9462681 com.sec.android.app.launcher/com.android.quickstep.RecentsActivity', status=NORMAL, monitor=false, responsive=true
    1011: channelName='SpenInputDetector', status=NORMAL, monitor=true, responsive=true
    1013: channelName='7fc68b2 TaskbarWindow', status=NORMAL, monitor=false, responsive=true
    1031: channelName='aafb634 com.sec.android.app.launcher/com.sec.android.app.launcher.activities.LauncherActivity', status=NORMAL, monitor=false, responsive=true
    320: channelName='PointerEventDispatcher0', status=NORMAL, monitor=true, responsive=true
      InputState: mMotionMementos: {deviceId=7, hovering=0, downTime=6358305510000}, 
    1126: channelName='d43f719 InputMethod', status=NORMAL, monitor=false, responsive=true
    1104: channelName='stylus-handwriting-event-receiver-0', status=NORMAL, monitor=false, responsive=true
    797: channelName='Embedded{View root of ReusableWindowDecorViewHost#1}', status=NORMAL, monitor=false, responsive=true
    913: channelName='[Gesture Monitor] PalmMotion', status=NORMAL, monitor=false, responsive=true
      InputState: mMotionMementos: {deviceId=7, hovering=0, downTime=6358305510000}, 
    895: channelName='d0fb81e StatusBar', status=NORMAL, monitor=false, responsive=true
    952: channelName='d6abd46 com.sec.android.app.launcher/com.samsung.app.honeyspace.edge.edgepanel.app.CocktailBarService', status=NORMAL, monitor=false, responsive=true
    897: channelName='64827a3 com.android.systemui.wallpapers.ImageWallpaper', status=NORMAL, monitor=false, responsive=true
    884: channelName='25cde4f com.android.systemui.wallpapers.ImageWallpaper', status=NORMAL, monitor=false, responsive=true
    896: channelName='dde5f4 Bouncer', status=NORMAL, monitor=false, responsive=true
    518: channelName='Embedded{View root of ReusableWindowDecorViewHost#0}', status=NORMAL, monitor=false, responsive=true
    823: channelName='recents_animation_input_consumer', status=NORMAL, monitor=false, responsive=true
    890: channelName='1f01c40 LockscreenShortcutBlur', status=NORMAL, monitor=false, responsive=true
    608: channelName='4c53228 ShellDropTarget', status=NORMAL, monitor=false, responsive=true
    894: channelName='4a5e534 NotificationShade', status=NORMAL, monitor=false, responsive=true
  RecentQueue: length=10
    MotionEvent, age=559ms
    MotionEvent, age=550ms
    MotionEvent, age=391ms
    MotionEvent, age=383ms
    MotionEvent, age=375ms
    MotionEvent, age=350ms
    MotionEvent, age=333ms
    MotionEvent, age=317ms
    MotionEvent, age=258ms
    MotionEvent, age=175ms
  PendingEvent:
    MotionEvent, age=7ms
  InboundQueue: <empty>
  CommandQueue: size=1
  TouchModePerDisplay:
    Display: 0 TouchMode: 1
  Configuration:
    KeyRepeatDelay: 50ms
    KeyRepeatTimeout: 400ms
    Show Pen hovering (Air view): true
    LatencyTracker:
      mTimelines.size() = 188
      mEventTimes.size() = 188
    LatencyAggregatorWithHistograms:
     Histograms:
      Identifier: vendor 0, product 0, sources: {1}, action: 6
       0: 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       1: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       2: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       3: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       4: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       5: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       6: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
      Identifier: vendor 0, product 0, sources: {11}, action: 1
       0: 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       1: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       2: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       3: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       4: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       5: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       6: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
      Identifier: vendor 0, product 0, sources: {11}, action: 2
       0: 1495, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       1: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       2: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       3: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       4: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       5: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       6: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
      Identifier: vendor 0, product 0, sources: {11}, action: 3
       0: 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       1: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       2: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       3: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       4: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       5: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       6: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
      Identifier: vendor 0, product 0, sources: {15}, action: 1
       0: 370, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
       1: 0, 0, 2, 3, 4, 1, 7, 2, 4, 2, 3, 2, 1, 2, 2, 0, 0, 0, 0, 5
       2: 0, 0, 0, 1, 1, 5, 6, 4, 5, 4, 2, 0, 2, 1, 2, 1, 0, 0, 1, 5
       3: 9, 10, 3, 3, 6, 3, 1, 4, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0
       4: 2, 7, 7, 3, 6, 1, 2, 2, 2, 2, 2, 0, 1, 0, 0, 1, 0, 0, 0, 2
       5: 0, 4, 2, 2, 6, 3, 3, 1, 0, 3, 4, 5, 2, 1, 0, 0, 0, 2, 0, 2
       6: 0, 10, 13, 7, 3, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2
      Identifier: vendor 0, product 0, sources: {15}, action: 2
       0: 7100, 2, 1, 4, 1, 1, 1, 0, 1, 3, 0, 1, 1, 1, 0, 0, 1, 0, 0, 6
       1: 20, 202, 240, 69, 17, 4, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4
       2: 9, 122, 81, 29, 28, 18, 30, 15, 29, 32, 27, 25, 22, 20, 6, 11, 8, 3, 7, 35
       3: 10, 9, 31, 118, 201, 92, 42, 22, 20, 6, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1
       4: 4, 76, 317, 94, 30, 25, 8, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       5: 0, 1, 0, 3, 52, 56, 68, 24, 56, 65, 87, 53, 11, 17, 11, 1, 8, 11, 20, 13
       6: 0, 202, 294, 11, 38, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
      Identifier: vendor 0, product 0, sources: {15}, action: 3
       0: 371, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       1: 0, 6, 13, 9, 4, 6, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1
       2: 0, 8, 15, 7, 3, 0, 1, 2, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 2
       3: 26, 9, 4, 2, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       4: 1, 15, 12, 7, 2, 1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
       5: 0, 2, 0, 0, 2, 3, 11, 7, 0, 2, 4, 8, 0, 0, 1, 0, 0, 1, 1, 1
       6: 0, 20, 18, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2
      Identifier: vendor 1256, product 41013, sources: {2}, action: 6
       0: 23, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1
       1: 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2
       2: 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       3: 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1
       4: 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0
       5: 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
       6: 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
      Identifier: vendor 1256, product 41013, sources: {3}, action: 6
       0: 29, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       1: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       2: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       3: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       4: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       5: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
       6: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
      mLastSlowEventTime=3354696316843
      mNumEventsSinceLastSlowEventReport = 82
      mNumSkippedSlowEvents = 0
  InputTracer: Disabled

```

