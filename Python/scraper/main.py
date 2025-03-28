import requests
from bs4 import BeautifulSoup
# from bs4 import element

from get_page_content import scrape_website
# from ObjectMaps import InfoparkPostMap

INFOPARK_WEBSITE = "https://infopark.in/companies/job-search"

# should get all rows from the info section 
# and add the portal link into seperate object
# instead of the text from the fourth column
# it should get the link of the button
def scrape_info_section(soup):
    all_from_info_section = []
    job_listing_div = soup.find('div', class_='job-info-sec')
    # print(job_listing_div.prettify())
    # Find the table element inside job_listing_div
    table = job_listing_div.find('table')
    # Check if the table exists
    if table:
        # Iterate through all rows of the table
        for row in table.find_all('tr'):
            # Find all cells (td) within the row
            cells = row.find_all('td')
            if cells:
                # Print the contents of each cell
                row_contents = []
                for cell in cells:
                    if "btn-sec" in cell.get('class', []):
                        a_tag = cell.find('a')
                        if a_tag and a_tag.get('href'):
                            row_contents.append(a_tag['href'])  # Add the link to row_contents
                        else:
                            row_contents.append('No link found')
                    else:
                        row_contents.append(cell.get_text(strip=True))
                # print(row_contents)
                # print("--------------------------------------------------")

                all_from_info_section.append(row_contents)
    else:
        print("No table found in the specified div.")

    return all_from_info_section


def main():
    print("Scraping Infopark Website...")
    scrape_info_section(scrape_website(INFOPARK_WEBSITE))
    print("Scraping completed.")


if __name__ == "__main__":
    main()