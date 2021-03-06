#!/bin/sh

major=0
minor=3
patch=0

build=$(cat buildnumber)
build=$((build+1))

echo $build > buildnumber

version="$major.$minor.$patch-$build"

echo $version

sed -i -e 's/^version = .*/version = "'"$version"'"/' Cargo.toml

cargo $1
