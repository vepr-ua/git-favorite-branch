#!/bin/bash

set -e
###
### This script is deprecated and no longer being used.
###

# Get new version from the tag
tag_version=$(git tag --points-at HEAD | grep -m 1 "^release/v.*" || true)
# Remove release prefix
tag_version=$(echo "$tag_version" | sed 's/release\/v//g')
# Get current version from Cargo.toml
toml_version=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')

if [ -z "$tag_version" ]; then
    tag_version="$toml_version"
    IFS=. read MAJOR MINOR PATCH <<<"${tag_version}"
    unset IFS
    ((PATCH = PATCH + 1))
    tag_version="${MAJOR}.${MINOR}.${PATCH}"
fi

printf "Resolving versions v%s -> v%s\n" $toml_version $tag_version

# Update Cargo.toml file with new version
cat Cargo.toml | sed "s/version = \"${toml_version}\"$/version = \"${tag_version}\"/" >Cargo2.toml
mv Cargo2.toml Cargo.toml

build_version="v${tag_version}"
build_target="aarch64-apple-darwin"
build_path="target/${build_target}/release"
binary_name="gfb"
build_basename="${binary_name}-${build_version}-${build_target}"
archive_path="${build_path}/${build_basename}"

cargo build --release --target "$build_target"

#
# Signing the binary
#

# Decode certificate
echo $MACOS_CERTIFICATE | base64 --decode >certificate.p12

# Temporary password for a temporary keychain
keychain_password="dfk3zyg_mby_HAP8hqm"
security create-keychain -p "$keychain_password" build.keychain
security default-keychain -s build.keychain

security unlock-keychain -p "$keychain_password" build.keychain
security import certificate.p12 -k build.keychain -P $MACOS_CERTIFICATE_PWD -T /usr/bin/codesign

security set-key-partition-list -S apple-tool:,apple:,codesign: -s -k "$keychain_password" build.keychain
/usr/bin/codesign --force -s "$APPLE_DEVELOPER_ID" "./${build_path}/${binary_name}" -v
rm certificate.p12
#
# Package Signed Binary
#

# Create a directory that will be then zipped together and distributed
mkdir -p "$archive_path"

# Include files in the distribution artifact
mv "${build_path}/${binary_name}" "${archive_path}/${binary_name}"
cp README.md "${archive_path}/README.md"
cp LICENSE.txt "${archive_path}/LICENSE.txt"

# Package the binary
pushd "${build_path}" >/dev/null
tar -czf "${build_basename}.tar.gz" "${build_basename}"/*

# Echo the sha. We will need it for homebrew
shasum -a 256 "${build_basename}.tar.gz"
popd >/dev/null

# Let subsequent steps know where to find the distribution
echo "DIST_FILENAME=${build_basename}.tar.gz" >>$GITHUB_OUTPUT
echo "DIST_PATH=${build_path}/${build_basename}.tar.gz" >>$GITHUB_OUTPUT

# Push these changes to main
git checkout main
git config user.name "Revent Studio Github Bot"
git config user.email bot@revent.studio
git commit -am "🍄 Bumping version to v${tag_version}"
git push origin HEAD:main

# Tag the binary as a new version
git tag "${build_version}"
git push origin HEAD:main --tags
