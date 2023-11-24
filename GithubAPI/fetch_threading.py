import requests
import json
import time
import base64

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
        print(f"Rate limit exceeded. Sleeping for {wait_time} seconds")
        print(f"at {time.strftime('%H:%M:%S', time.localtime())}\n")
        time.sleep(wait_time + 1)

api_call_counter = 0

def search_in_repo(repo_url, keyword, api_token):
    global api_call_counter
    api_call_counter += 1

    headers = {'Authorization': f'token {api_token}'}
    repo_full_name = repo_url.replace('https://github.com/', '')
    query = f"{keyword}+repo:{repo_full_name}"
    url = f"https://api.github.com/search/code?q={query}"

    response = requests.get(url, headers=headers)

    if api_call_counter % 10 == 0:
        remaining, reset = get_rate_limit_from_headers(response.headers)
        if remaining <= 1:
            wait_for_rate_limit_reset(reset)

    found_lines = []
    if response.status_code == 200 and response.json()['total_count'] > 0:
        items = response.json()['items']
        processed_files = set()

        for item in items:
            if len(found_lines) >= 5:
                break
            file_path = item['path']
            if file_path not in processed_files:
                processed_files.add(file_path)
                file_url = item['url']
                file_response = requests.get(file_url, headers=headers)
                if file_response.status_code == 200:
                    file_content = file_response.json()['content']
                    file_lines = base64.b64decode(file_content).decode().split('\n')
                    for line_number, line in enumerate(file_lines, 1):
                        if keyword in line:
                            found_lines.append(f"{item['path']}:{line_number} {line}")
                            break
                if len(found_lines) >= 5:
                    break
    time.sleep(1)
    return found_lines

def analyze_repositories(language, threading_keyword, api_token):
    repos = load_repos(f'./jsons/{language}_repos.json')
    found_repos = {}

    for repo in repos:
        repo_url = repo['html_url']
        found_lines = search_in_repo(repo_url, threading_keyword, api_token)
        if found_lines:
            found_repos[repo_url] = found_lines
    return found_repos

api_token = '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'
threading_keywords = {
    'Python': 'threading',
    'Rust': 'std::thread',
    'C': 'pthread',
    'JavaScript': "workers",
    'C++': 'thread',
}

result = {}
for language, keyword in threading_keywords.items():
    print(f"Analyzing {language} repositories...")
    found_repos = analyze_repositories(language, keyword, api_token)

    filename = f'./txts/{language}_{keyword}_results.txt'
    with open(filename, 'w') as file:
        for repo_url, lines in found_repos.items():
            file.write(f"{repo_url}\n")
            for line in lines:
                file.write(f"- {line}\n")
            file.write('\n')

    print(f"Results for {language} saved to {filename}")
