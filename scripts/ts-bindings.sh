#!/bin/bash

zits -d "import {Link} from './deps.types';" -i crates/delivery_types -i crates/delivery_integrity -i crates/delivery_common -i crates/delivery_coordinator -i crates/delivery_api -o webcomponents/src/bindings/delivery.ts

zits --default-zome-name zSecret -i crates/delivery_types -i playground/zomes/secret -i playground/zomes/secret_integrity -o playground/webapp/src/bindings/secret.ts
