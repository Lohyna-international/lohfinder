#!usr/bin/bash

export PUBSUB_PROJECT_ID=lohfinder
$(gcloud beta emulators pubsub env-init)
echo $PUBSUB_EMULATOR_HOST
gcloud beta emulators pubsub start