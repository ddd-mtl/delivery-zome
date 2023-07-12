#!/bin/bash
# TOP LEVEL
rm .running
rm .hc_live*
rm Cargo.lock
rm -rf target
# WEBCOMPONENTS
rm -rf webcomponents/dist
rm webcomponents/tsconfig.tsbuildinfo
# PLAYGROUND DNA
rm secret.dna
rm secret.happ
# PLAYGROUND WEB-APP
rm -rf playground/webapp/dist
rm -rf playground/webapp/out-tsc
rm playground/webapp/tsconfig.tsbuildinfo
rm playground/webapp/ui.zip