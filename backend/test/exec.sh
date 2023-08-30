#!/bin/bash
if [ "$(id -u)" != "0" ]; then
   echo -e "\e[31mThis script must be run as root\e[0m" 1>&2
   exit 1
fi

apt update -y
apt upgrade -y 
apt install python3 -y 
pip install openai python-dotenv -y 
python3 ./gpt_api.py
