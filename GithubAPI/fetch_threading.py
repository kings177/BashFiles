import requests
import json
import time

def load_repos(filename):
    with open(filename, 'r') as file:
        return json.load(file)

def get_rate_limit_from_headers(headers):
    remaining = int(headers.get('X-RateLimit-Remaining', 0))
    reset = int(headers.get('X-RateLimit-Reset', 0))
    return remaining, reset

def wait_for_rate_limit_reset(reset_time):
    wait_time = reset_time - time.time()
    if wait_time > 0:
        print(f"at {time.strftime('%H:%M:%S', time.localtime())}\n")
        print(f"Rate limit exceeded. Sleeping for {wait_time} seconds\n")
        time.sleep(wait_time + 10)

api_call_counter = 0

def search_in_repo(repo_url, keyword, api_token):
    global api_call_counter
    api_call_counter += 1

    headers = {'Authorization': f'token {api_token}'}
    repo_full_name = repo_url.replace('https://github.com/', '')
    query = f"{keyword}+repo:{repo_full_name}"
    url = f"https://api.github.com/search/code?q={query}"

    response = requests.get(url, headers=headers)

    remaining, reset = get_rate_limit_from_headers(response.headers)
    if remaining <= 1:
        wait_for_rate_limit_reset(reset)

    if response.status_code == 200 and response.json()['total_count'] > 0:
        print(f"Found {keyword} in {repo_url}")
    else:
        print(f"No {keyword} found in {repo_url}")

    return response.status_code == 200 and response.json()['total_count'] > 0

def analyze_repositories(language, threading_keyword, api_token):
    repos = load_repos(f'./jsons/{language}_repos.json')
    count = 0
    found_repos = []

    for repo in repos:
        repo_url = repo['html_url']
        if search_in_repo(repo_url, threading_keyword, api_token):
            count += 1
            found_repos.append(repo_url)

    return count, found_repos

api_token = 'YOUR_TOKEN'
threading_keywords = {
    'Python': 'threading',
    'Rust': 'std::thread',
    'C': 'pthread',
    'JavaScript': "workers",
    'C++': 'pthread',
}

result = {}
for language, keyword in threading_keywords.items():
    print(f"Analyzing {language} repositories...")
    count, found_repos = analyze_repositories(language, keyword, api_token)
    result[language] = count
    print(f"{language}: {count} out of 1000 repositories use threading")

    filename = f'./txts/{language}_threading_results.txt'
    with open(filename, 'w') as file:
        file.write(f"Total repositories using {keyword}: {count}\n")
        for repo in found_repos:
            file.write(repo + '\n')

    print(f"Results for {language} saved to {filename}")
