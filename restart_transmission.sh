#!/bin/bash
set -ex

/etc/init.d/transmission-daemon stop

/etc/init.d/transmission-daemon start
