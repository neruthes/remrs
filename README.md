# remrs

CLI command to start a web daemon in-place to edit file in browser. 


```sh
### Instead of...
sudo nano /etc/nginx/sites-enabled/default
### Now you can...
sudo remrs /etc/nginx/sites-enabled/default
# > http://1.1.1.1:33333/?token=c0e86e3b-3b82-412c-8ca0-8647839c5db4
```



## Standard Installation
```sh
./make.sh all  # See code for details...
```


## Fast Installation
```sh
sudo curl 'https://pub-714f8d634e8f451d9f2fe91a4debfa23.r2.dev/remrs/26c0255d50da12d5432a2380/remrs' -o /tmp/downloading-quarantine--remrs &&
if sha256sum -c <<< "16ca23f1018ae95f362728d569c9c3ec38157e3c042668219fc8059eb387e91c /tmp/downloading-quarantine--remrs"; then
    sudo install -m755 /tmp/downloading-quarantine--remrs /usr/local/bin/remrs
else
    echo "[FATAL] Checksum railed!"
    rm /tmp/downloading-quarantine--remrs
fi
```



## Copyright

Copyright (c) 2026 Neruthes.

Released with the GNU GPL 2.0 license.

