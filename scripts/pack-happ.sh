#!/bin/bash

set -e

hc dna pack --output=artifacts/secret.dna playground/workdir
hc app pack --output=artifacts/secret.happ playground/workdir
