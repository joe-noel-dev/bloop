# Projects production readiness

This plan prepares the consolidated local and cloud projects model for release. It
must be completed in order: the product behaviour needs to be defined before the
sync implementation or API can be finalised.

## 1. Project state contract

Document the expected behaviour for every project state:

| State | Required behaviour |
| --- | --- |
| Local only | Available offline; can be explicitly uploaded if the user is signed in. |
| Cloud only | Visible when cloud access is available; opening downloads a local mirror. |
| Mirrored and current | Available both locally and in the cloud; opening works offline. |
| Local changes pending upload | Clearly indicated; must not be overwritten without the selected conflict policy. |
| Cloud changes pending download | Clearly indicated; opening follows the selected conflict policy. |
| Both changed | Surface a conflict; do not silently discard either copy. |
| Cloud unavailable | Keep local projects usable and state that cloud information is unavailable. |

### Agreed policies

1. A mirrored project's local edits automatically upload after a debounced delay
   when the cloud revision still matches the mirror's recorded base revision. This
   keeps normal edits in sync without issuing an upload for every change.
2. When both local and cloud copies changed from the same base revision, do not
   overwrite either copy. Prompt the user to choose the local or cloud version and
   show the most recent edit time for both.
3. A local-only project is added to the cloud only through an explicit UI action.
   Once added, it is a mirrored project and should best-effort remain in sync.
4. Cloud deletion does not implicitly remove the local copy. The UI presents three
   distinct actions: **Delete Cloud**, **Delete Local**, and **Delete Both**.

The remaining decision is where per-device song and section selection is persisted;
it does not change the synchronisation contract below.

### Keep playback selection out of synced project data

`Project.selections` currently contains the selected song and section. Selection is
device/session state: loading a project, selecting a song, or starting playback
must not make the project dirty or create a cloud update. Move this state out of
the synced `Project` payload before finalising project revision and conflict logic.

Decide and document:

1. Whether the last selected song and section are retained only for the active
   session or persisted locally per project and device.
2. The default selection when no local selection exists, or when the remembered
   song or section has since been deleted.
3. How the API supplies selection to clients after a project is loaded without
   modifying the project bytes or its cloud revision.

## 2. Correct sync and deletion behaviour

1. Add an explicit **Add to Cloud** action for local-only projects. It creates the
   cloud project, uploads its metadata, project file, and referenced samples, then
   records a common base revision for the resulting mirror.
2. Queue a debounced best-effort upload after local edits to a mirrored project.
   Before uploading, compare the cloud revision with the recorded base revision. If
   it still matches, upload and advance the base revision; otherwise enter conflict
   handling without writing to either copy.
3. Record local revision, cloud revision, and the last common base revision. A
   matching cloud revision alone must not mean a locally edited mirror is current.
4. On divergent changes, return both versions' most recent edit time and prompt the
   user to keep the local or cloud version. Preserve both until the user chooses.
5. Compare revisions before replacing a mirror. Preserve both copies until conflict
   resolution succeeds.
6. Build deletion targets from the chosen UI action and actual project availability:
   **Delete Cloud** requests remote only, **Delete Local** requests local only, and
   **Delete Both** requests both.
7. Treat deletion as a two-step operation with an explicit partial-result outcome.
   After either success or partial failure, return a fresh project snapshot.
8. Surface cloud unavailability, authentication failure, and retryable sync errors
   in the clients instead of representing them as an empty cloud list.

## 3. Refactor core responsibilities

Separate the current `ProjectStore` responsibilities into focused components:

| Component | Responsibilities |
| --- | --- |
| `ProjectStore` | Local project metadata, files, samples, and last-opened project. |
| `CloudMirror` | Cloud revision tracking, transactional download, staging, interrupted-refresh recovery, and conflict detection. |
| `ProjectCatalog` | Local/cloud enumeration, cloud availability, and `ProjectsSnapshot` construction. |
| Sync service | Add-to-cloud, debounced best-effort uploads, downloads, revision comparison, conflict outcomes, and partial failure results. |
| Project session state | Per-device active project, song/section selection, and playback-only state. |

`MainController` should translate requests and service outcomes into responses. It
should not directly inspect local and remote backends to decide how a project is
loaded. Services should return typed outcomes such as `Current`, `Downloaded`,
`LocalChangesPending`, `Conflict`, and `CloudUnavailable`.

## 4. Complete the API migration

1. Make `ProjectsSnapshot` the sole project-list payload for every response path,
   including sync completion, project deletion, project creation, rename, and other
   project mutations.
2. Update Android and iOS to consume the snapshot exclusively and show cloud
   availability and project sync state in the project UI.
3. Retain `Response.projects` and `Response.cloud_projects` during the client
   compatibility window. Existing clients and the current sync response still use
   those fields.
4. Once the minimum supported Android and iOS versions consume snapshots on every
   path, remove the legacy response fields, response helpers, fallback reducers, and
   legacy tests in one breaking API change.

The snapshot should expose enough information for the UI to distinguish local-only,
cloud-only, current mirrors, pending local changes, pending cloud changes, and
conflicts. Two lists and `cloud_available` alone are not sufficient for that.

## 5. Add the integration test matrix

Add end-to-end tests under `core/tests` using the controller fixture and mock cloud
backend. Each test should assert both persisted backend state and the resulting
`ProjectsSnapshot`.

### Lifecycle

- Create and save a local project.
- Load, rename/update, duplicate, and delete a project.
- Confirm project mutation responses return an updated snapshot.
- Load a project, change the selected song or section, and begin playback without
  changing the project bytes, dirty state, or cloud revision.
- Restore a persisted local selection when valid, and safely fall back when the
  remembered song or section no longer exists.

### Availability combinations

- Local-only project.
- Cloud-only project, then download/open it.
- Mirrored project that is current.
- Multiple mixed projects in one snapshot.
- Cloud unavailable with local projects still present.

### Synchronisation and conflicts

- Add a local-only project to cloud and verify cloud files, samples, metadata, and a common base revision.
- Upload local changes after the debounce and verify cloud files, samples, and metadata.
- Verify a changed cloud revision prevents automatic upload and produces a conflict response with both edit times.
- Choose the local or cloud version during a conflict and verify the selected version becomes the new mirror base.
- Download cloud changes and verify the local mirror atomically changes.
- Local changes pending upload.
- Cloud changes pending download.
- Both copies changed, asserting the chosen conflict policy preserves data.
- Failed download or missing referenced audio preserves the previous local mirror.
- Restart recovery after an interrupted mirror replacement.

### Deletion and error handling

- Delete local-only and cloud-only projects.
- Delete a mirrored project with **Delete Local**, **Delete Cloud**, and **Delete Both**; verify **Delete Cloud** preserves the local copy.
- Remote deletion failure preserves the local project when that is the policy.
- Local deletion failure after remote success reports a partial result and refreshes
  the snapshot.
- Unauthenticated and unavailable-cloud responses are distinct from an empty cloud
  account.

## 6. Release gate

Before release:

1. Run the full core test suite, including the new integration matrix.
2. Run Android unit tests and a debug compilation.
3. Run iOS tests and build the supported application targets.
4. Perform a manual two-device smoke test: create, upload, download, edit, conflict,
   offline reopen, retry, and delete.
5. Confirm upgrades preserve existing local projects, cloud mirrors, and interrupted
   refresh backups.
6. Document the chosen sync/conflict/deletion policy and the legacy API removal
   version in the release notes.

Release is blocked until the state contract is agreed, local edits have a safe cloud
sync path, conflict handling is deterministic, and the integration matrix passes.
