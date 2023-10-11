#from github import Github
import requests
import time
import json

req = requests.get('https://api.github.com/repos/Traumatism/alembers/commits/main')

data = req.json()
#print(json.dumps(data))

print(data['stats'])


# #github_token = "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"
#github_token="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"


