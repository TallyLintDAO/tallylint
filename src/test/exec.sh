#!/bin/bash
apt update
apt upgrade
apt install python3
pip install openai python-dotenv
python3 ./gpt_api.py