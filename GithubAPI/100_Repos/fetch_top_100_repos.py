import requests
import time
import json
import os

def fetch_repos(language, api_token, max_repos=100):
    repos = []
    page = 1
    headers = {'Authorization': f'token {api_token}'}

    while len(repos) < max_repos:
        url = f'https://api.github.com/search/repositories?q=language:{language}&sort=stars&order=desc&page={page}&per_page=100'
        response = requests.get(url, headers=headers)

        if response.status_code != 200:
            print(f"Failed to fetch page {page} of repos for {language} with status code {response.status_code}")
            break
            
        data = response.json()['items']
        repos.extend(data)

        if len(data) < 100:
            break

        page += 1
        time.sleep(1) # rate limited sleep

        if len(repos) >= max_repos:
            print(f"Fetched {len(repos)} repos for {language}")
            repos = repos[:max_repos] 

    return repos

def save_repos_to_file(repos, language):
    with open(f'./jsons/{language}_100_repos.json', 'w') as file:
        json.dump(repos, file, indent=4)

api_token= os.getenv('GITHUB_API_TOKEN')
languages = ['C', 'C++'] # Just C/C++ cuz... yeah!

for language in languages:
    print(f"Fetching top repos for {language}")
    top_repos = fetch_repos(language, api_token)
    print(f"Fetched {len(top_repos)} repos for {language}")

    print(f"Saving repos to file for {language}")
    save_repos_to_file(top_repos, language)
