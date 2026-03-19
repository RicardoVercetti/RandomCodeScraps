import uuid
from datetime import datetime
from dataclasses import dataclass
from typing import Optional

@dataclass
class CardProgram:
    name: str
    bin: str
    id: Optional[str]


class Config:
    self.institution_nr = None          # this is meant to be filled manually by the user

    def __init__(self):
        self.id = str(uuid.uuid4())
        self.prepared_date = self.get_datetime()
        self.card_programs_list: list[CardProgram] = []
        self.cards_file: list[str] = []
        self.accounts_file: list[str] = []
        self.customers_file: list[str] = []
        self.card_accounts_file: list[str] = []
        self.customer_accounts: list[str] = []

    def get_datetime(self) -> datetime:
        return datetime.now()

    def get_id(self):
        return self.id

    def store_in_file(self):
        # todo
        pass

    def add_card_program(self, card_program: CardProgram):
        # maybe check if this already exists and there is no conflict with name and the bin number
        self.card_programs_list.append(card_program)

    def is_program_added():
        # todo
        pass

    def check_if_valid_for_card_program():
        # todo
        pass

    def add_cards_file(self, filename: str):
        self.cards_file.append(filename)

    def add_accounts_file(self, filename: str):
        self.accounts_file.append(filename)

    def add_customers_file(self, filename: str):
        self.customers_file.append(filename)

    def add_card_accounts_file(self, filename: str):
        self.card_accounts_file.append(filename)

    def add_customer_accounts_file(self, filename: str):
        self.customer_accounts.append(filename)
    


def main():
    conf = Config()
    print(f"datetime: {conf.get_datetime()}")
    print(f"type: {type(conf.get_datetime())}")
    print(f"id: {conf.id}")

if __name__ == "__main__":
    print("the main file runs...")
    main()