#!/bin/bash
set -ex

# NOTE:
#   Certain changes can't be done from tranmission remote and has to go through
#   the config file. Parameters we need to change are:
#
#   utp-enabled:
#   cache-size-mb:
#   download-queue-enabled
#
# Config setting file:
# https://github.com/transmission/transmission/wiki/Editing-Configuration-Files
#
# Man page:
# https://manpages.debian.org/testing/transmission-cli/transmission-remote.1.en.html

# -------------------------------
#   operations we want to enable
# -------------------------------

# Set the session's maximum memory cache size in MiB. This cache is used to reduce disk IO.
transmission-remote -n 'transmission:mypassword' --cache=0


# Disable upload speed limits. If current torrent(s) are selected this operates on them. Otherwise, it changes the global setting.
transmission-remote -n 'transmission:mypassword' --no-uplimit

# Disable download speed limits. If current torrent(s) are selected this operates on them. Otherwise, it changes the global setting.
transmission-remote -n 'transmission:mypassword' --no-downlimit

# Disable uTP for peer connections. If current torrent(s) are selected this operates on them. Otherwise, it changes the global setting.
transmission-remote -n 'transmission:mypassword' --no-utp

# ----------------------------------
#   Check status of transmission
# ----------------------------------

# List session information from the server
transmission-remote -n 'transmission:mypassword' --session-info

# List statistical information from the server
transmission-remote -n 'transmission:mypassword' --session-stats

# List all torrents
transmission-remote -n 'transmission:mypassword' --list


# TODO
# --torrent-done-script filename
# Specify a file to run each time a torrent finishes
