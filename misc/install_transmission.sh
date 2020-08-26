#!/bin/bash
set -ex

# https://help.ubuntu.com/community/TransmissionHowTo
sudo add-apt-repository ppa:transmissionbt/ppa -y
sudo apt-get update -y
sudo apt-get install transmission-cli transmission-common transmission-daemon -y
