import requests
import time
import json
import os
from datetime import datetime, timedelta

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

def generate_yearly_segments(start_year, end_year):
    segments = []
    for year in range(start_year, end_year + 1):
        start_date = f'{year}-01-01'
        end_date = f'{year}-12-31'
        segments.append((start_date, end_date))
    return segments

def fetch_repos(language, api_token, start_year, end_year, max_repos=10000):
    headers = {'Authorization': f'token {api_token}'}
    repos = {}
    segments = generate_yearly_segments(start_year, end_year)
    sort_criteria = ['stars', 'forks']

    for segment in segments:
        for sort in sort_criteria:
            page = 1
            max_pages = 10

            print(f"\nSwitching to segment {segment}, sort {sort} for language {language}\n")

            while page <= max_pages:
                query = f"language:{language}+created:{segment[0]}..{segment[1]}"
                url = f'https://api.github.com/search/repositories?q={query}&sort={sort}&order=desc&page={page}&per_page=100'
                response = requests.get(url, headers=headers)

                print(f"Fetching {sort} repos for {language}, segment {segment}, page {page}")

                remaining, reset = get_rate_limit_from_headers(response.headers)
                if remaining <= 0:
                    wait_for_rate_limit_reset(reset)

                if response.status_code == 200:
                    items = response.json().get('items', [])
                    for item in items:
                        repos[item['id']] = item
                elif response.status_code == 403:
                    wait_for_rate_limit_reset(reset)
                elif response.status_code == 422:
                    print("End of available results for this query")
                    break
                else:
                    print(f"Failed to fetch repos: {response.status_code}")
                    print("Error details: ", response.json())
                    break

                page += 1
                time.sleep(1) # rate limited sleep

                if len(repos) >= max_repos:
                    return list(repos.values())

    return list(repos.values())

def save_repos_to_file(repos, language):
    with open(f'./jsons/{language}_repos.json', 'w') as file:
        json.dump(repos, file, indent=4)

api_token = os.getenv('GITHUB_API_TOKEN')
start_year = 2013
end_year = 2023
languages = ['Python', 'Rust', 'JavaScript', 'C', 'C++']
top_repos = {}

for language in languages:
    print(f"Fetching top repos for {language}")
    top_repos = fetch_repos(language, api_token, start_year, end_year)
    print(f"Fetched {len(top_repos)} repos for {language}")

    print(f"Saving repos to file for {language}")
    save_repos_to_file(top_repos, language)
