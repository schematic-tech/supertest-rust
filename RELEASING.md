# Releasing

The interop library is dual licensed under **MIT OR Apache-2.0**, at the recipient's
option. Both license texts ship with the package. `VERSION` is the stable release
version and must agree with all package metadata.

Development happens in `schematic-internal`. A release is a squash snapshot copied
to the repository of the same name under `schematic-tech`. These workflows do not
copy history, create public repositories, or push release tags.

## Trigger and behavior

After copying the reviewed snapshot to the public repository, commit it, then push
a matching version tag such as `v0.1.0`. `.github/workflows/release.yml` runs CI,
builds and tests the installable artifacts, and publishes the same verified artifacts.
It only publishes from this repository's exact `schematic-tech` name. A private-repo
tag does not publish. A normal branch push only runs CI.

The publish job uses a GitHub environment named **release**. Create that environment
in the public repository before the first release. Set its deployment rules to
allow the release tags. Required reviewers are optional; omit them if tag pushes
should publish without a second manual step. Keep Actions enabled and allow the
pinned actions used in these workflows. GitHub release uploads use the automatically
provided `GITHUB_TOKEN`; no GitHub personal access token is needed.

All public GitHub releases include source `.tar.gz` and `.zip` archives, both
licenses, and `SHA256SUMS`, alongside the language-specific artifacts. Assets are
uploaded to a draft first and the draft is then published. Already published
GitHub release assets are not overwritten on retries.

## Versions and retries

Update `VERSION` and the language's package metadata together. The first prepared
version is `0.1.0`. The release validator rejects tags that do not match the package
version. Run CI on the final public squash commit before tagging it.

If publication fails partway through, correct the external configuration and rerun
the failed GitHub Actions run for the same tag. Registry uploads skip versions
already uploaded during an earlier attempt. Do not change the contents of an
already published version or move a published tag; use a new version for code fixes.

Registry publication is separate from checker support. These packages provide
markers and runtime assumptions; they do not expand Pup/backend language discovery.

## crates.io setup

Two crates are published, in dependency order:

1. `schematic-supertest-macros`
2. `schematic-supertest`

**For the first release**, create a crates.io API token able to create/publish both
names. Put it in the public repository's `release` environment as the secret
**`CRATES_IO_TOKEN`**. Use a short expiration and restrict it to these names where
the token settings allow. The workflow uses this token when present.

**After both crates exist**, configure trusted publishing on **each** crate:

| Field | Value |
| --- | --- |
| GitHub owner | `schematic-tech` |
| GitHub repository | `supertest-rust` |
| Workflow filename | `release.yml` |
| Environment | `release` |

Then delete `CRATES_IO_TOKEN` from GitHub and revoke it on crates.io. With that
secret absent, the workflow automatically uses OIDC via `crates-io-auth-action`.
You must configure both crates before removing the bootstrap token.

Cargo waits for the macros release to become available before publishing the main
crate. A rerun skips an already published version, allowing a partial release to
finish. GitHub receives both `.crate` archives; docs.rs builds public crate docs.

The library's MSRV is Rust 1.85. Release packaging uses Cargo 1.97.1 so a workspace
can be packaged and verified against a temporary local registry before first publication.
Run `python scripts/package.py` from a clean committed checkout to verify packages.

Reference: [crates.io trusted publishing](https://crates.io/docs/trusted-publishing).
