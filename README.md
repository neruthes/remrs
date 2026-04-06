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
sudo curl 'https://pub-714f8d634e8f451d9f2fe91a4debfa23.r2.dev/remrs/26c0255d50da12d5432a2380/remrs' -o /usr/local/bin/remrs &&
if sha256sum -c <<< "f0cd3d676da819928f07b1ec294319ada2f1b3562c1afb17044cd923c57e03a8 /usr/local/bin/remrs"; then
    sudo chmod +x /usr/local/bin/remrs
else
    echo "[FATAL] Checksum railed!"
    rm /usr/local/bin/remrs
fi
```



## Copyright

Copyright (c) 2026 Neruthes.

Released with the GNU GPL 2.0 license.

