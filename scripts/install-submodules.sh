#!/bin/bash

# Script for downloading submodule dependencies

echo Executing \"$0\".

# Check pre-conditions
if [ $# != 1 ]; then
  echo 1>&2 "$0: Aborting. Missing argument: holochain version"
  exit 2
fi

hcversion=$1
echo for holochain version $hcversion
if [ "$hcversion" == "hc" ] || [ "$hcversion" == "" ] ; then
  echo Missing \"hc-version\" field in \"package.json\".
  exit 1
fi
branchname=hdk-${hcversion:2}
echo Getting branch: $branchname

echo \* Create 'submodules' folder
rm -rf submodules
mkdir submodules
cd submodules

echo \* Download latest install scripts
git clone --depth 1 https://github.com/ddd-mtl/hc-prebuilt

cd ..
echo
echo \* Done
