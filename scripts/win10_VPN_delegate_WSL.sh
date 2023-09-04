#!/bin/bash
# i want to add this to PATH of fish . how ? like a win10 shortcut in Programs folder
# 输入: V2rayN最下面一排的 4个端口号之一. 我用的是第四个
# 例子: ./win10_VPN_delegate_WSL.sh "25526"
#  如果有问题请看: https://github.com/doraemonkeys/WSL_IP  我参考的她的教程
# 自动获取宿主 Windows 的 IP 地址
proxy_server=`cat /etc/resolv.conf|grep nameserver|awk '{print $2}'`
# 改成你的 http_proxy（局域网）端口号
proxy_port=$1
export http_proxy=http://$proxy_server:$proxy_port
export HTTP_PROXY=$http_proxy
export https_proxy=$http_proxy
export HTTPS_PROXY=$http_proxy

echo "win10 VPN works to WSL start"

echo $http_proxy
echo $https_proxy
echo $HTTP_PROXY
echo $HTTPS_PROXY

if curl --silent --head --max-time 3 https://www.google.com/ | grep "HTTP.*200" > /dev/null; then
        printf "\033[92mGoogle connectivity OK! \033[0m\n"
else
        echo -e "\033[31mGoogle connectivity BAD.\033[0m"
fi        
