#!/bin/bash
set -ex


# stop all:
transmission-remote -n 'transmission:mypassword' -S

# remote all:
transmission-remote -n 'transmission:mypassword' -r -t 3

# list
transmission-remote -n 'transmission:mypassword' -l
