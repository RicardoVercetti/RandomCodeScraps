
class InfoparkPostMap:
    def __init__(self, job_title, company_name, last_date_of_submission, poral_link):
        self.job_title = job_title
        self.company_name = company_name
        self.last_date_of_submission = last_date_of_submission
        self.poral_link = poral_link

    def __str__(self):
        return f"Job Title: {self.job_title}, Company Name: {self.company_name}, Last Date of Submission: {self.last_date_of_submission}, Portal Link: {self.poral_link}"
