#!/bin/bash

export http_proxy=http://192.168.235.1:25526

export HTTP_PROXY=$http_proxy
export https_proxy=$http_proxy
export HTTPS_PROXY=$http_proxy

echo "win10 VPN works to WSL start"
if curl --silent --head --max-time 3 https://www.google.com/ | grep "HTTP.*200" > /dev/null; then
        printf "\033[92mGoogle connectivity OK! \033[0m\n"
else
        echo -e "\033[31mGoogle connectivity BAD.\033[0m"
fi        
