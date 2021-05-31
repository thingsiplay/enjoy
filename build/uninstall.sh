#!/usr/bin/bash

name=enjoy
# install_dir=/usr/local/bin
install_dir=$(systemd-path user-binaries)
install_cmd=$(which install)

rm -f "$install_dir/$name"

if [[ $? -ne 0 ]]
then
    echo "Error! Could not uninstall $name from $install_dir !!"
    exit 1
else
    echo "$name uninstalled from $install_dir"
fi
