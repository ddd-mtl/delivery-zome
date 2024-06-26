#!/bin/bash

set -e

zits -d "import {EntryDefIndex} from './deps.types';" -i submodules/zome-signals -i crates/delivery_types -i crates/delivery_integrity -i crates/delivery_coordinator -i crates/delivery_api -o webcomponents/src/bindings/delivery.ts

zits --default-zome-name zSecret -d "export type EntryDefIndex = number;" -i crates/delivery_types -i playground/zomes/secret -i playground/zomes/secret_integrity -o playground/webapp/src/bindings/secret.ts
