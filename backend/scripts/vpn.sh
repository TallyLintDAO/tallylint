#!/bin/bash
export HTTP_PROXY=http://192.168.235.1:25526
export https_proxy=http://192.168.235.1:25526
export HTTPS_PROXY=http://192.168.235.1:25526

echo "win10 VPN server serve ubuntu  vm"

echo $http_proxy
echo $https_proxy
echo $HTTP_PROXY
echo $HTTPS_PROXY

if curl --silent --head --max-time 3 https://www.google.com/ | grep "HTTP.*200" > /dev/null; then
        printf "\033[92mGoogle connectivity OK! \033[0m\n"
else
        echo -e "\033[31mGoogle connectivity BAD.\033[0m"
fi        
