#!/usr/bin/bash

name=enjoy
# install_dir=/usr/local/bin
install_dir=$(systemd-path user-binaries)
install_cmd=$(which install)
config_src=example-config.ini
config_dest=~/.config/enjoy/default.ini

"$install_cmd" -m 755 -b -C -D -t "$install_dir" "$name"
if [[ $? -ne 0 ]]
then
    echo "Error! Could not install $name to $install_dir !!"
    exit 1
else
    echo "$name installed to $install_dir/$name"
fi

if [ -f "$config_dest" ]
then
    echo "$config_dest already exists"
else
    "$install_cmd" -m 644 -b -C -D -T "$config_src" "$config_dest"
    if [[ $? -ne 0 ]]
    then
        echo "Error! Could not install $config_src to $config_dest !!"
        exit 1
    else
        echo "$config_src installed to $config_dest"
    fi
fi

