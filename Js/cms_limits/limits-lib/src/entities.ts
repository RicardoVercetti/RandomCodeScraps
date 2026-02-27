// -- Plain entities --
let id: number = 1;

function generateId(): number {
    const current_id: number = id;
    id +=1;
    return current_id;
}

function generateAccountNumber(): string {
    const id = generateId();
    const id_len = new String(id).length;
    // account max length say 16
    const rem_len = 16 - id_len;
    return "0".repeat(rem_len) + id;
}

// Institution

export interface Instituion {
    institution_id: number,
    institutionName: string,
}

export function createInstitution(institution_name?: string): Instituion {
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

export interface Card {
    card_no: string,
    level_profile_id: number
    limit_profile_id: number
}

export function createCard(card_no: string, limit_profile_id: number, level_profile_id: number): Card {
    return {
        card_no,
        level_profile_id,
        limit_profile_id
    }
}

// Account

export interface Account {
    account_no: string,
    account_type: string,
    limit_profile_id: number
}

export function createAccount(limit_profile_id: number, account_type?: string, account_no?: string): Account {
    const new_account_no = account_no ? account_no : generateAccountNumber();
    return {
        account_no: new_account_no,
        account_type: account_type ? account_type : "00",
        limit_profile_id: limit_profile_id
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

// account customers