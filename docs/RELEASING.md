# Releasing Bloop

The version on `main` is the version to release. After publishing it, bump
`main` to the next target version.

## Release checklist

1. Check that `main` is green and its version is the one being released.
2. In [GitHub Releases](https://github.com/joe-noel-dev/bloop/releases), create
   and publish a release from `main` with a new tag named `v<version>` (for
   example, `v0.15.0`).
3. Watch the tag-triggered GitHub Actions runs:
   - Core builds and attaches the Debian package to the GitHub release.
   - Android builds and attaches the signed APK to the GitHub release.
4. Release iOS manually from a checkout of the released tag:

   ```sh
   BUILD_NUMBER=<new-unique-build-number> ./scripts/release-ios.sh
   ```

   The App Store Connect and Match environment variables used by Fastlane must
   already be available in the shell.
5. Run the
   [Set Version workflow](https://github.com/joe-noel-dev/bloop/actions/workflows/set-version.yml)
   with the **next** target version, without the `v` prefix. For example, after
   releasing `v0.15.0`, enter `0.16.0`.
6. Review and merge the version-bump pull request created by the workflow.

The version bump happens after the release so ongoing work on `main` identifies
itself as the next release.

## iOS workflow

The iOS GitHub Actions workflow is currently disabled, so the manual release in
step 4 is required. If the workflow is re-enabled, a `v*` tag will run the
Fastlane `beta` lane and upload to TestFlight automatically. In that case, do
not also run the manual script for the same release.
