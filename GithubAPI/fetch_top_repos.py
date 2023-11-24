import requests
import time
import json
import os

def get_rate_limit_from_headers(headers):
    remaining = int(headers.get('X-RateLimit-Remaining', 0))
    reset = int(headers.get('X-RateLimit-Reset', 0))
    return remaining, reset

def wait_for_rate_limit_reset(reset_time):
    wait_time = reset_time - time.time()
    if wait_time > 0:
        print(f"Rate limit exceeded. Sleeping for {wait_time} seconds")
        print(f"at {time.strftime('%H:%M:%S', time.localtime())}\n")
        time.sleep(wait_time + 1)

def fetch_repos(language, api_token, max_repos=10000):
    headers = {'Authorization': f'token {api_token}'}
    repos = []
    page = 1
    max_pages = 100

    while len(repos) < max_repos and page <= max_pages:
        url = f'https://api.github.com/search/repositories?q=language:{language}&sort=stars&order=desc&page={page}&per_page=100'
        response = requests.get(url, headers=headers)

        print(f"Fetching page {page} of repos for {language}")

        remaining, reset = get_rate_limit_from_headers(response.headers)
        if remaining <= 0:
            wait_for_rate_limit_reset(reset)

        if response.status_code == 200:
            repos.extend(response.json()['items'])
        elif response.status_code == 403:
            wait_for_rate_limit_reset(reset)
        else:
            print(f"Failed to fetch repos: {response.status_code}")
            print("Error details: ", response.json())
            break

        page += 1
        time.sleep(1) # rate limited sleep

        if len(repos) >= max_repos:
            repos = repos[:max_repos]

    return repos

def save_repos_to_file(repos, language):
    with open(f'./jsons/{language}_repos.json', 'w') as file:
        json.dump(repos, file, indent=4)

api_token = os.getenv('GITHUB_API_TOKEN')
languages = ['Python', 'Rust', 'JavaScript', 'C', 'C++']
top_repos = {}

for language in languages:
    print(f"Fetching top repos for {language}")
    top_repos = fetch_repos(language, api_token)
    print(f"Fetched {len(top_repos)} repos for {language}")

    print(f"Saving repos to file for {language}")
    save_repos_to_file(top_repos, language)
