import requests
import json
import time

def load_repos(filename):
    """ Load repository data from JSON file"""
    with open(filename, 'r') as file:
        return json.load(file)

def get_rate_limit(api_token):
    """ Check the current rate limit status"""
    headers = {'Authorization': f'token {api_token}'}
    response = requests.get('https://api.github.com/rate_limit', headers=headers)
    if response.status_code == 200:
        rate_limit = response.json()['resources']['search']
        return rate_limit['remaining'], rate_limit['reset']
    return 0, 0

def wait_for_rate_limit_reset(reset_time):
    """ Wait until the rate limit is reset"""
    wait_time = reset_time - time.time()
    if wait_time > 0:
        print(f"Rate limit exceeded. Sleeping for {wait_time} seconds")
        time.sleep(wait_time + 10)


api_call_counter = 0

def search_in_repo(repo_url, keyword, api_token):
    """ Search for a keyword in a repository """
    global api_call_counter

    headers = {'Authorization': f'token {api_token}'}
    repo_full_name = repo_url.replace('https://github.com/', '')
    query = f"{keyword}+repo:{repo_full_name}"
    url = f"https://api.github.com/search/code?q={query}"

    api_call_counter += 1

    if api_call_counter % 25 == 0:
        remaining, reset = get_rate_limit(api_token)
        if remaining <= 1:
            wait_for_rate_limit_reset(reset)

    print(f"Searching for {keyword} in {repo_url}, Remaining requests before call: {remaining}")

    response = requests.get(url, headers=headers)

    remaining_after, _ = get_rate_limit(api_token)
    print(f"Remaining requests after call: {remaining_after}")

    # print(f"Response: {response.json()}")
    time.sleep(1)

    return response.status_code == 200 and response.json()['total_count'] > 0

def analyze_repositories(language, threading_keyword, api_token):
    repos = load_repos(f'./jsons/{language}_repos.json')
    count = 0

    for repo in repos:
        repo_url = repo['html_url']
        if search_in_repo(repo_url, threading_keyword, api_token):
            count += 1

    return count

api_token = ''
threading_keywords = {
    'Python': 'threading',
    'Rust': 'std::thread',
    'C': 'pthread',
    'JavaScript': "worker_threads)"
}

result = {}
for language, keyword in threading_keywords.items():
    print(f"Analyzing {language} repositories...")
    count = analyze_repositories(language, keyword, api_token)
    result[language] = count
    print(f"{language}: {count} out of 1000 repositories use threading")

print("Threading usage by language:")
for language, count in result.items():
    print(f"{language}: {count} out of 1000 repositories use threading")

