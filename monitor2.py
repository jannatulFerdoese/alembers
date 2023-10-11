import requests
import json

import time


req = requests.get('https://api.github.com/repos/Traumatism/alembers/commits/main')


data = req.json()
#print(json.dumps(data))

print(data['stats'])

repository_owner = 'Traumatism'
repository_name = 'alembers'
branch_name = 'main'

# Create the URL to fetch commit statistics for the specified branch
url = f'https://api.github.com/repos/{repository_owner}/{repository_name}/commits/{branch_name}'

# Make the API request
response = requests.get(url)

if response.status_code == 200:
    data = response.json()

    # Extract commit statistics
    additions = data['stats']['additions']
    deletions = data['stats']['deletions']
    total = data['stats']['total']

    print(f'Commit Statistics for {branch_name} branch:')
    print(f'Additions: {additions}')
    print(f'Deletions: {deletions}')
    print(f'Total Changes: {total}')
else:
    print(f'Failed to fetch commit statistics. Status Code: {response.status_code}')
