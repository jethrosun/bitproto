# Bittorrent Prototype



## transmission remote cmd:
https://help.ubuntu.com/community/TransmissionHowTo

stop all:
transmission-remote -n 'transmission:mypassword' -S

remote all:
transmission-remote -n 'transmission:mypassword' -r -t 3



## proper access to download dir
https://askubuntu.com/questions/221081/permission-denied-when-downloading-with-transmission-daemon

sudo usermod -a -G debian-transmission jethros
sudo chgrp debian-transmission /data/downloads
sudo chmod 770 /data/downloads



## Reference:
https://forums.sonarr.tv/t/getting-nowhere-with-transmission-setup/3795/3
