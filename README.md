# remrs

CLI command to start a web daemon in-place to edit file in browser. 


```
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
# sudo curl 'https://github.com/neruthes/remrs/releases/download/0.1.0/remrs' -o /usr/local/bin/remrs && sudo chmod +x /usr/local/bin/remrs
```



## Copyright

Copyright (c) 2026 Neruthes.

Released with the GNU GPL 2.0 license.

