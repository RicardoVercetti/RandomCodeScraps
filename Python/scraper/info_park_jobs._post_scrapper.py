import requests
from bs4 import BeautifulSoup
import csv
import urllib3

# Disable SSL warnings
urllib3.disable_warnings(urllib3.exceptions.InsecureRequestWarning)


def save_to_csv(filename, headers, rows):
    with open(filename, mode='w', newline='', encoding='utf-8') as file:
        writer = csv.writer(file)
        writer.writerow(headers)
        writer.writerows(rows)
    print(f"✅ Data saved successfully to '{filename}'")


def get_content(url):
    """Fetch and parse HTML content of a page."""
    response = requests.get(url, verify=False)
    response.raise_for_status()
    soup = BeautifulSoup(response.text, 'html.parser')
    return soup


def get_table_data(soup, div_class):
    """Extract headers and rows from the target div's table."""
    target_div = soup.find('div', class_=div_class)
    if not target_div:
        print(f"No div found with class '{div_class}'")
        return [], []

    table = target_div.find('table')
    if not table:
        print("No table found inside the div.")
        return [], []

    headers = [th.get_text(strip=True) for th in table.find_all('th')]

    rows = []
    for tr in table.find_all('tr')[1:]:  # skip header row
        tds = tr.find_all('td')
        if not tds:
            continue

        cells = []
        for i, td in enumerate(tds):
            if i == 4:  # 5th column (index starts at 0)
                link_tag = td.find('a')
                if link_tag and link_tag.get('href'):
                    link = link_tag['href']
                    # ensure full URL
                    if link.startswith('/'):
                        link = 'https://infopark.in' + link
                    cells.append(link)
                else:
                    cells.append('')
            else:
                cells.append(td.get_text(strip=True))
        rows.append(cells)

    return headers, rows


def get_next_page(soup):
    """Find the 'Next' page link if it exists."""
    next_page_tag = soup.select_one('ul.pagination li a[rel="next"]')
    if next_page_tag and next_page_tag.get('href'):
        return next_page_tag['href']
    return None


if __name__ == "__main__":
    base_url = "https://infopark.in/companies/job-search"
    div_class = "job-info-sec"
    all_rows = []
    headers = []

    current_url = base_url
    page_num = 1

    while current_url:
        print(f"📄 Scraping page {page_num}: {current_url}")
        soup = get_content(current_url)

        headers, rows = get_table_data(soup, div_class)
        if not rows:
            print("No rows found on this page.")
            break

        all_rows.extend(rows)
        next_page = get_next_page(soup)
        if next_page and next_page != current_url:
            current_url = next_page
            page_num += 1
        else:
            break

    # Save all collected data
    if headers and all_rows:
        save_to_csv("jobs.csv", headers, all_rows)
    else:
        print("No data found to save.")
