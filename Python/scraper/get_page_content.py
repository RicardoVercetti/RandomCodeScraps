import requests
from bs4 import BeautifulSoup

def scrape_website(url):
    try:
        response = requests.get(url, timeout=10, verify=False)
        response.raise_for_status()
        
        soup = BeautifulSoup(response.text, 'lxml')
        title = soup.title.string if soup.title else 'No Title Found'
        
        # print(f"--- Title: {title}")
        return soup
    
    except requests.exceptions.RequestException as e:
        print(f"Error fetching the website: {e}")