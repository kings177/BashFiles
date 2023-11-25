import asyncio
import aiohttp
import json
import time
import base64
import os
from datetime import datetime

now = {time.strftime('%H:%M:%S', time.localtime())}

async def get_rate_limit_from_headers(headers):
    remaining = int(headers.get('X-RateLimit-Remaining', 0))
    reset = int(headers.get('X-RateLimit-Reset', 0))
    return remaining, reset

async def wait_for_rate_limit_reset(reset_time):
    wait_time = reset_time - time.time()
    if wait_time > 0:
        print(f"Rate limit exceeded. Sleeping for {wait_time} seconds at {now}\n")
        await asyncio.sleep(wait_time + 1)

async def search_in_repo(session, repo_url, keyword):
    repo_full_name = repo_url.replace('https://github.com/', '')
    query = f"{keyword}+repo:{repo_full_name}"
    url = f"https://api.github.com/search/code?q={query}"
    found_lines = []


    async with session.get(url) as response:
        if response.status == 200 and (await response.json())['total_count'] > 0:
            items = (await response.json())['items']
            processed_files = set()

            for item in items:
                if len(found_lines) >= 5:
                    break
                file_path = item['path']
                if file_path not in processed_files:
                    processed_files.add(file_path)
                    file_url = item['url']
                    async with session.get(file_url) as file_response:
                        if file_response.status == 200:
                            file_content = (await file_response.json())['content']
                            file_lines = base64.b64decode(file_content).decode().split('\n')
                            for line_number, line in enumerate(file_lines, 1):
                                if keyword in line:
                                    found_lines.append(f"{item['path']}:{line_number} {line}")
                                    break
                        if len(found_lines) >= 5:
                            break
        await asyncio.sleep(1)
    return found_lines

async def analyze_repositories(language, threading_keyword, api_tokens):
    repos = load_repos(f'./jsons/{language}_repos.json')
    found_repos = {}
    repo_count = 0
    token_index = 0
    processed_repos_count = 0

    while repos:
        token = api_tokens[token_index % len(api_tokens)]
        headers = {'Authorization': f'token {token}'}

        async with aiohttp.ClientSession(headers=headers) as session:
            for repo in list(repos):
                found_lines = await search_in_repo(session, repo['html_url'], threading_keyword)

                if found_lines:
                    found_repos[repo['html_url']] = found_lines
                    repo_count += 1
                repos.remove(repo)

                processed_repos_count += 1
                if processed_repos_count % 100 == 0:
                    print(f"{processed_repos_count} repositories processed so far for {language} at {now}\n")


        async with aiohttp.ClientSession() as session:
            response = await session.get('https://api.github.com/rate_limit')
            remaining, reset = await get_rate_limit_from_headers(response.headers)
            if remaining == 0:
                token_index += 1
                if token_index >= len(api_tokens):
                    print("{now}\n All tokens are rate limited. Waiting for rate limit reset...")
                    await wait_for_rate_limit_reset(reset)
                    token_index = 0
                else:
                    print(f"Current token's rate limit exceeded. Switching to next token: {api_tokens[token_index % len(api_tokens)]}")

    return found_repos, repo_count
    
def load_repos(filename):
    with open(filename, 'r') as file:
        return json.load(file)

api_tokens = ['token1', 'token2', 'token3']

threading_keywords = {
    'C': 'pthread',
    'C++': 'thread',
}

result = {}
for language, keyword in threading_keywords.items():
    print(f"Analyzing {language} repositories...")
    found_repos, repo_count = asyncio.run(analyze_repositories(language, keyword, api_tokens))

    filename = f'./txts/{language}_{keyword}_results.txt'
    with open(filename, 'w') as file:
        file.write(f"Total Repositories with '{keyword}' keyword: {repo_count}\n\n")
        for repo_url, lines in found_repos.items():
            file.write(f"{repo_url}\n")
            for line in lines:
                file.write(f"- {line}\n")
            file.write('\n')

    print(f"Results for {language} saved to {filename}")
