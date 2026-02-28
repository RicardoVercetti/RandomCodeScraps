import { LimitProfile } from "./limits";
import { accountsStore, cardAccountsStore, cardsStore } from "./store";

// -- Plain entities --
let id: number = 1;

function generateId(): number {
    const current_id: number = id;
    id +=1;
    return current_id;
}


// Institution

export interface Institution {
    institution_id: number,
    institutionName: string,
}

export function createInstitution(institution_name?: string): Institution {
    const id = generateId();
    const new_institution_name = institution_name ? institution_name : `institution_${id}`;
    return {
        institution_id: id,
        institutionName: new_institution_name,
    }
}

// Card product

export interface CardProduct {
    product_id: number,
    product_name: string,
    bin: string,
    limit_profile_id: number
}

export function createCardProduct(bin: string, limit_profile_id: number, product_name?: string): CardProduct {
    const product_id = generateId();
    const new_product_name = product_name ? product_name : `product_${product_id}`;
    return {
        product_id,
        product_name: new_product_name,
        bin,
        limit_profile_id
    }
}

// Card

export class Card {
    private static lastNo = 0;
    private institution_nr: number;
    private bin_number: string;
    private card_no: string;
    private card_product: CardProduct;
    private limit_profile_id: number;

    constructor(institution: Institution, card_product: CardProduct, limit_profile?: LimitProfile) {
        this.institution_nr = institution.institution_id;
        this.bin_number = card_product.bin;
        this.card_product = card_product;
        this.card_no = Card.createCardNumber(this.bin_number);
        Card.push(this);
        this.limit_profile_id = limit_profile ? limit_profile.limit_profile_id : 0;
    }

    public static createCardNumber(bin_no: string): string {
        const this_no = Card.lastNo;
        Card.lastNo += 1;
        const this_no_length = new String(this_no).length;
        const pad_length = 15 - this_no_length - bin_no.length;
        return bin_no + "0".repeat(pad_length) + this_no + "1";      // using 0 as the check digit all the time
    }

    public getInstitutionNr(): number {
        return this.institution_nr;
    }

    public getCardNo(): string {
        return this.card_no;
    }

    public getBinNo(): string {
        return this.bin_number;
    }

    public setLimitProfile(limit_profile: LimitProfile) {
        this.limit_profile_id = limit_profile.limit_profile_id;
    }

    public static push(thisCard: Card) {
        for(const card of cardsStore) {
            if (card.card_no === thisCard.card_no) {
                console.log(`card: ${card.card_no} already exists in store, not adding`);
                return;
            }
        }
        cardsStore.push(thisCard);
        console.log(`Added card: ${thisCard.card_no} into store`);
    }
}

// Account
export class Account {
    private institution_nr: number;
    private account_no: string;
    private limit_profile_id: number;

    private static last_account_no = 1;

    constructor(institution: Institution, limit_profile?: LimitProfile) {
        this.institution_nr = institution.institution_id;
        this.account_no = Account.createAccountNumber();
        Account.push(this);
        this.limit_profile_id = limit_profile ? limit_profile.limit_profile_id : 0;
    }

    public static createAccountNumber(): string {
        const last_account_no = Account.last_account_no;
        const last_no_len = new String().length;
        const pad_length = 15 - last_no_len;
        Account.last_account_no += 1;
        return "0".repeat(pad_length) + last_account_no;
    }

    public getAccountNo(): string {
        return this.account_no;
    }

    public getInstitutionNr(): number {
        return this.institution_nr;
    }

    public setLimitProfile(limit_profile: LimitProfile) {
        this.limit_profile_id = limit_profile.limit_profile_id;
    }

    public static push(this_account: Account) {
        for(const account of accountsStore) {
            if(account.account_no === this_account.account_no) {
                console.log(`The account: ${this_account.account_no} already exists in store.`);
                return;
            }
        }
        accountsStore.push(this_account);
        console.log(`Added account: ${this_account.account_no} into store`);
    }
}

// Customer
export interface Customer {
    customer_id: number,
    customer_name: string,
    limit_profile_id: number,
}

export function createCustomer(limit_profile_id: number, customer_name?: string): Customer {
    const id = generateId();
    const new_customer_name = customer_name ? customer_name : `customer_name_${id}`;
    return {
        customer_id: id,
        customer_name: new_customer_name,
        limit_profile_id: limit_profile_id
    }
}


// -- Associations --

// card accounts
export class CardAccounts {
    private static id: number = 1;
    private card_no: string;
    private account_no: string;
    private institution_nr: number;

    constructor(card: Card, account: Account) {
        if(card.getInstitutionNr() != account.getInstitutionNr()) {
            throw new Error("card and account must be under the same institution to perform linking process");
        }

        this.card_no = card.getCardNo();
        this.account_no = account.getAccountNo();
        this.institution_nr = card.getInstitutionNr();

        // check if this link already exists(if yes, ignore), if it doesn't - add it into queue
        CardAccounts.push(this);
    }

    public static createNewId(): number {
        const this_id = CardAccounts.id;
        CardAccounts.id += 1;
        return this_id;
    }

    public static push(this_card_account: CardAccounts) {
        for(const cardAccount of cardAccountsStore) {
            if(this_card_account.account_no === cardAccount.account_no && this_card_account.card_no === cardAccount.card_no) {
                console.log(`Link of card: ${cardAccount.card_no} & account: ${cardAccount.account_no} already exists, not adding to store`);
                return;
            }
        }
        cardAccountsStore.push(this_card_account);
        console.log(`Added (${this_card_account.card_no}-${this_card_account.account_no})card-accounts link to store.`);
    }
}

// account customers