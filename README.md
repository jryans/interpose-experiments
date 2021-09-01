This repo contains a jumble of experiments and notes exploring paths to function
interposition and code injection.

## macOS

**Target version**: macOS 10.15

macOS is the main focus of this work, as it seems to apply the most complex and
convoluted layers out of all of the major desktop platforms at the moment.

### Protections to investigate

* [App Sandbox][as] ([guide][asdg])
  * Blocks access to files and system resources when entitlement set
  * Can be relaxed through further entitlements
  * Apps get a special app container directory to work with that only they can
    access
* Sandboxing via profiles (sometimes called "Seatbelt") such as those in
  `/System/Library/Sandbox/Profiles/*`
  * While this is marked as deprecated, it's far more powerful than the newer
    App Sandbox, and remains heavily used by macOS system components, browsers,
    etc.
  * The newer App Sandbox makes use of this older system internally by applying
    the profile `/System/Library/Sandbox/Profiles/application.sb` during app
    startup
* [Hardened Runtime][hr]
  * Blocks code injection, memory access, debugger access when signing option
    (`-o runtime`) set
  * Can be relaxed through further entitlements
* [Notarization][nz]
  * Developers send apps to Apple's notarization service which staples them with
    a signature if they pass various undocumented checks
  * macOS 10.14.5 and later require apps and kernel extensions to be notarized
  * Requirements include:
    * Code signature
    * Hardened Runtime enabled
    * Secure timestamp in signature
    * `com.apple.security.get-task-allow` entitlement must not be present (but
      perhaps [permissible when hosting plugins][nzplugins] and disabling
      library validation as well)
    * Link against macOS 10.9 or later SDK
    * Must have properly-formatted XML ASCII entitlements
* [System Integrity Protection][sip]
  * Blocks access to various system files via:
    * Paths listed in `/System/Library/Sandbox/rootless.conf`
    * Files marked with `com.apple.rootless` xattr
  * [Blocks access][sipruntime] to Mach tasks for protected apps and those
    without the `get-task-allow` entitlement (all apps, or just hardened
    runtime?)
      * `SecTaskAccess` plist key on requester also plays some role
  * Blocks unsigned kernel extensions from loading
  * Can be disabled via `csrutil` in Recovery OS
* `task_for_pid` access enforcement
  * Some parts checked by kernel, others passed off to `taskgated` daemon
  * Flags examined by these checks may include:
    * SIP protection status of target
    * `get-task-allow` entitlement on target
    * `SecTaskAccess` plist key on requester
    * `debugger` entitlement on requester

### Injection strategies to explore

* Custom kernel extension
* Parent process spawning app via various methods
  * NSTask
  * XPC services
  * `spawn`
  * `launchApplication`
* Stub process that absorbs code from disk and morphs into different app

[as]: https://developer.apple.com/documentation/security/app_sandbox
[asdg]: https://developer.apple.com/library/archive/documentation/Security/Conceptual/AppSandboxDesignGuide/
[hr]: https://developer.apple.com/documentation/security/hardened_runtime
[nz]: https://developer.apple.com/documentation/xcode/notarizing_macos_software_before_distribution
[nzplugins]: https://developer.apple.com/documentation/xcode/notarizing_macos_software_before_distribution/resolving_common_notarization_issues#3087731
[sip]: https://developer.apple.com/library/archive/documentation/Security/Conceptual/System_Integrity_Protection_Guide/
[sipruntime]: https://developer.apple.com/library/archive/documentation/Security/Conceptual/System_Integrity_Protection_Guide/RuntimeProtections/RuntimeProtections.html
