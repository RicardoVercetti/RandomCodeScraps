# src/config.py
import uuid
from datetime import datetime
from dataclasses import dataclass
from typing import Optional
import json
from pathlib import Path

@dataclass
class CardProgram:
    name: str
    bin: str
    id: str = ""
    min_range: str = ""
    max_range: str = ""

    def to_dict(self) -> dict:
        return {
            "name": self.name,
            "bin": self.bin,
            "id": self.id,
            "min_range": self.min_range,
            "max_range": self.max_range
        }

class ConfigFileNotFoundExcpetion(Exception):
    """
    Raised when the file path passed in the load the config.json does not exist
    """
    pass

class MissingRequiredFieldException(Exception):
    """
    Raised when the required fields for Config object is missing
    """
    pass

class IncorrectInput(Exception):
    """
    Raised when an input present in the json file in incorrect
    """
    pass


class Config:

    def __init__(self, filename="sample_config.json", id: Optional[str] = None, prep_date: str = None):
        self.id: str = id if id is not None else str(uuid.uuid4())
        self.filename: str = filename
        self.prepared_date: str = prep_date if prep_date is not None else str(self.get_datetime())
        self.card_programs_list: list[CardProgram] = []
        self.cards_file: list[str] = []
        self.accounts_file: list[str] = []
        self.customers_file: list[str] = []
        self.card_accounts_file: list[str] = []
        self.customer_accounts: list[str] = []
        self.institution_nr: int = None          # this is meant to be filled manually by the use
        self.hostname: str = None
        self.port: int = None
        self.username: str = None
        self.password: str = None

    def get_datetime(self) -> datetime:
        return datetime.now()

    def get_id(self):
        return self.id

    def store_in_file(self):
        # already existing file will be overwritten

        data = {
            "run_id": self.id,
            "prepared_date": str(self.prepared_date),
            "hostname": None,
            "port": None,
            "username": None,
            "password": None,
            "institution_nr": None,
            "card_programs": [cp.to_dict() for cp in self.card_programs_list],
            "files": {
                "cards": self.cards_file,
                "accounts": self.accounts_file,
                "customers": self.customers_file,
                "card_accounts": self.card_accounts_file,
                "customer_accounts": self.customer_accounts
            }
        }
        with open(self.filename, "w") as file:
            json.dump(data, file, indent=2)

    @staticmethod
    def load_from_file(path: str) -> Config:
        """
        Create the Config object from a .json file provided

        Returns:
        Config object of the deserialized values

        Raises:
        ConfigFileNotFoundExcpetion: If the file passed in the {path} variable is not found
        json.JSONDecodeError: If the data present in the file is not a valid json data
        MissingRequiredFieldException: If the file doesn't have the mandatory data present
        IncorrectInput: If the data is in incorrect format
        """
        # todo: load the parameters from file
        # check if the institution ID is provided
        # check if minimum migration files are there - cards, accounts, customers, card_accounts, account_customers
        file_path = Path(path)

        if file_path.exists():
            # todo: try deserializing it
            
            with open(path, "r") as file:
                config_dict = json.loads("".join(file.readlines()))
                # print(f"config_data: {config_dict}")
                # print(f"run_id: {config_dict["run_id"]}")
                return Config.convert_dict_into_config(config_dict, path)
            
        else:
            raise ConfigFileNotFoundExcpetion(f"No config file found on path: {path}, Please do a sanity check run first to create one.")

    @staticmethod
    def convert_dict_into_config(inp: dict, path: str) -> Config:
        """
        Raises:
        MissingRequiredFieldException: if any of required fields are missing for creation of Config
        IncorrectInput: If any of the input provided in the json is not in the right format
        """
        # run_id
        run_id = inp.get("run_id")
        if not run_id:
            raise MissingRequiredFieldException("invalid run_id")

        # prepared_date
        prepared_date = inp.get("prepared_date")
        if not prepared_date:
            raise MissingRequiredFieldException("invalid prepared_date")
        
        # hostname
        hostname = inp.get("hostname")
        if not hostname:
            raise MissingRequiredFieldException("hostname must be present")

        # port
        port = inp.get("port")
        if not port:
            raise MissingRequiredFieldException("Port must be present")

        # username
        username = inp.get("username")
        if not username:
            raise MissingRequiredFieldException("username must be present")

        # password
        password = inp.get("password")
        if not password:
            raise MissingRequiredFieldException("password must be present")
        
        # instituion_nr
        institution_nr = inp.get("institution_nr")
        if not institution_nr:
            raise MissingRequiredFieldException("institution_nr must be present")

        # card_programs: product name or ID must be there for each BIN present in the entry
        card_programs: list[CardProgram] = inp.get("card_programs")
        if card_programs:
            # if present, make sure the above satisfies
            # if not present, not required
            if type(card_programs) is not list:
                raise IncorrectInput("card_programs input must be a list")
            
            for card_program in card_programs:
                if type(card_program) is not dict:
                    raise IncorrectInput("type of elements inside the card_programs list must be propper json") 
                if not card_program.get("name") and not card_program.get(id):
                    raise IncorrectInput("card_program_name or id must be present")
                if not card_program.get("bin"):
                    raise IncorrectInput("bin nubmer should never be empty for a card program")

        # optional file lists
        cards_file_list = inp.get("cards")
        accounts_file_list = inp.get("accounts")
        customers_file_list = inp.get("customers")
        card_accounts_list = inp.get("card_accounts")
        customer_accounts_list = inp.get("customer_accounts")

        # create Config object with the data available
        conf = Config(path, id=run_id, prep_date=prepared_date)
        conf.hostname = hostname
        conf.port = port
        conf.username = username
        conf.password = password
        conf.institution_nr = institution_nr
        conf.card_accounts_file = cards_file_list if cards_file_list is not None else []
        conf.accounts_file = accounts_file_list if accounts_file_list is not None else []
        conf.customers_file = customers_file_list if customers_file_list is not None else []
        conf.card_accounts_file = card_accounts_list if card_accounts_list is not None else []
        conf.customer_accounts = customer_accounts_list if customer_accounts_list is not None else []

        # all set
        return conf


    def add_card_program(self, card_program: CardProgram):
        # maybe check if this already exists and there is no conflict with name and the bin number
        self.card_programs_list.append(card_program)

    def add_all_card_program(self, card_programs: list[CardProgram]):
        self.card_programs_list.extend(card_programs)

    def check_if_valid_for_card_program():
        # todo
        pass

    def add_cards_file(self, filename: str):
        self.cards_file.append(filename)

    def add_card_files(self, filenames: list[str]):
        self.cards_file.extend(filenames)

    def add_accounts_file(self, filename: str):
        self.accounts_file.append(filename)

    def add_customers_file(self, filename: str):
        self.customers_file.append(filename)

    def add_card_accounts_file(self, filename: str):
        self.card_accounts_file.append(filename)

    def add_customer_accounts_file(self, filename: str):
        self.customer_accounts.append(filename)

    def __str__(self) -> str:
        return_dict = {
            "run_id": self.id,
            "source_file_name": self.filename,
            "prep_date": self.prepared_date,
            "hostname": self.hostname,
            "port": self.port,
            "username": self.username,
            "password": self.password,
            "institution_nr": self.institution_nr,
        }
        return json.dumps(return_dict)
    


def main():
    # from dataclasses import asdict

    # conf = Config()
    # # # print(f"datetime: {conf.get_datetime()}")
    # # # print(f"type: {type(conf.get_datetime())}")
    # # # print(f"id: {conf.id}")
    # cp1 = CardProgram("card_program_1", "123456", None)
    # cp2 = CardProgram("card_program_2", "234567", None)


    # conf.add_all_card_program([cp1, cp2])
    # conf.add_cards_file("./res/cards.txt")
    # conf.add_accounts_file("./res/accounts.txt")
    # conf.add_customers_file("./res/customers.txt")
    # conf.add_card_accounts_file("./res/card_accounts.txt")
    # conf.add_customer_accounts_file("./res/customer_accounts.txt")


    # conf.store_in_file()
    # print(f"store the json file successfully to file: {conf.filename}")

    print("started deserialization")
    config: Config = Config.load_from_file("sample_config.json")

    print(f"deserialized data: {config.__str__()}")


if __name__ == "__main__":
    print("the main file runs...")
    main()