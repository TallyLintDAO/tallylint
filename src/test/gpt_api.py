import os
from dotenv import load_dotenv
import openai
load_dotenv()
OPENAI_KEY="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"
# Set openai.api_key to the OPENAI environment variable
openai.api_key = OPENAI_KEY
completion = openai.ChatCompletion()

def askgpt(question, chat_log=None):
    if chat_log is None:
        chat_log = [{
            'role': 'system',
            'content': 'You are a helpful, upbeat and funny assistant.',
        }]
    chat_log.append({'role': 'user', 'content': question})
    response = completion.create(model='gpt-3.5-turbo', messages=chat_log)
    answer = response.choices[0]['message']['content']
    chat_log.append({'role': 'assistant', 'content': answer})
    return answer, chat_log

prompt='Q:   Why did the chicken cross the road?'
print(prompt)
answer, log = askgpt(prompt)

print('A:  '+answer)
print(log)