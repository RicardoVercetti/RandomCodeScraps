# src/config.py
from __future__ import annotations
import uuid
import json
from datetime import datetime
from dataclasses import dataclass
from typing import Optional, Any
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

# --- Exceptions ---
class ConfigFileNotFoundException(Exception): pass
class MissingRequiredFieldException(Exception): pass
class IncorrectInput(Exception): pass

class Config:
    def __init__(self, filename="config.json", id: Optional[str] = None, prep_date: str = None):
        self.id: str = id if id is not None else str(uuid.uuid4())
        self.filename: str = filename
        self.prepared_date: str = prep_date if prep_date is not None else datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        
        self.card_programs_list: list[CardProgram] = []
        
        # These will hold lists of dicts: [{"path": str, "lines": int}]
        self.cards_file: list[dict] = []
        self.accounts_file: list[dict] = []
        self.customers_file: list[dict] = []
        self.card_accounts_file: list[dict] = []
        self.customer_accounts_file: list[dict] = []
        
        self.institution_nr: Optional[int] = None
        self.hostname: Optional[str] = None
        self.port: Optional[int] = None
        self.username: Optional[str] = None
        self.password: Optional[str] = None

    def store_in_file(self):
        """Serializes the current object state to the JSON file."""
        data = {
            "run_id": self.id,
            "prepared_date": self.prepared_date,
            "hostname": self.hostname,
            "port": self.port,
            "username": self.username,
            "password": self.password,
            "institution_nr": self.institution_nr,
            "card_programs": [cp.to_dict() for cp in self.card_programs_list],
            "files": {
                "cards": self.cards_file,
                "accounts": self.accounts_file,
                "customers": self.customers_file,
                "card_accounts": self.card_accounts_file,
                "customer_accounts": self.customer_accounts_file
            }
        }
        with open(self.filename, "w") as file:
            json.dump(data, file, indent=2)

    @staticmethod
    def load_from_file(path: str) -> Config:
        file_path = Path(path)
        if not file_path.exists():
            raise ConfigFileNotFoundException(f"No config file found on path: {path}")

        with open(path, "r") as file:
            try:
                config_dict = json.load(file)
                return Config.convert_dict_into_config(config_dict, path)
            except json.JSONDecodeError:
                raise IncorrectInput("File is not a valid JSON")

    @staticmethod
    def convert_dict_into_config(inp: dict, path: str) -> Config:
        # Validate Required Fields
        required = ["run_id", "prepared_date", "hostname", "port", "username", "password", "institution_nr"]
        for field in required:
            if inp.get(field) is None:
                raise MissingRequiredFieldException(f"Field '{field}' is missing or null")

        # Create instance
        conf = Config(path, id=inp["run_id"], prep_date=inp["prepared_date"])
        conf.hostname = inp["hostname"]
        conf.port = inp["port"]
        conf.username = inp["username"]
        conf.password = inp["password"]
        conf.institution_nr = inp["institution_nr"]

        # Card Programs
        raw_programs = inp.get("card_programs", [])
        if not isinstance(raw_programs, list):
            raise IncorrectInput("card_programs must be a list")
            
        for prog in raw_programs:
            if not prog.get("bin"):
                raise IncorrectInput("BIN number missing in card program entry")
            conf.card_programs_list.append(CardProgram(
                name=prog.get("name", ""),
                bin=prog.get("bin"),
                id=prog.get("id", ""),
                min_range=prog.get("min_range", ""),
                max_range=prog.get("max_range", "")
            ))

        # Files mapping
        files_dict = inp.get("files", {})
        conf.cards_file = files_dict.get("cards", [])
        conf.accounts_file = files_dict.get("accounts", [])
        conf.customers_file = files_dict.get("customers", [])
        conf.card_accounts_file = files_dict.get("card_accounts", [])
        conf.customer_accounts_file = files_dict.get("customer_accounts", [])

        return conf

    def add_card_files(self, file_entries: list[dict]):
        self.cards_file.extend(file_entries)

    def add_card_program(self, card_program: CardProgram):
        self.card_programs_list.append(card_program)

    def __str__(self) -> str:
        return f"Config(ID: {self.id}, Host: {self.hostname}, Programs: {len(self.card_programs_list)})"