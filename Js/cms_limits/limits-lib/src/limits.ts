let id_index: number = 1;

function next_id(): number {
    const current_id = id_index;
    id_index += 1;
    return current_id;
}

// Limit profile definitions
export interface LimitProfileDef {
    id: number,
    limit_def_name: string,
    limit_profile_id: number,
    institution_nr: number,
    last_updated_date?: Date,
    last_updated_user?: string
}

export function createLimitProfileDef(
    limit_profile_id: number,
    institution_nr: number,
): LimitProfileDef {
    const id = next_id();
    return {
        id: id,
        limit_def_name: `limit_profile_def_${id}`,
        limit_profile_id: limit_profile_id,
        institution_nr: institution_nr,
        last_updated_date: new Date(),
        last_updated_user: "SYSTEM"
    }
}

// Limit profile - max withdrawal amount, max_balance_amount, topup_amount, no_of_transactions | daily, weekly, monthly | online_atm, online_pos, offline, CNP 
export interface LimitProfile {
    limit_profile_id: number,
    limit_profile_desc: string,

    total_daily_withdrawal_amount_online_atm: number,
    total_daily_balance_amount_online_atm: number,
    total_daily_topup_amount_online_atm: number,
    total_daily_no_of_transactions_online_atm: number,

    total_weekly_withdrawal_amount_online_atm: number,
    total_weekly_balance_amount_online_atm: number,
    total_weekly_topup_amount_online_atm: number,
    total_weekly_no_of_transactions_online_atm: number,

    total_monthly_withdrawal_amount_online_atm: number,
    total_monthly_balance_amount_online_atm: number,
    total_monthly_topup_amount_online_atm: number,
    total_monthly_no_of_transactions_online_atm: number,

    // daily, weekly, monthly - online pos

    // daily, weekly, monthly - offline

    // daily, weekly, monthly - CNP
}

export function createLimitProfile(): LimitProfile {
    const id = next_id();
    return {
        limit_profile_id: id,
        limit_profile_desc: `limit_profile_des_${id}`,

        total_daily_withdrawal_amount_online_atm: 0,
        total_daily_balance_amount_online_atm: 0,
        total_daily_topup_amount_online_atm: 0,
        total_daily_no_of_transactions_online_atm: 0,

        total_weekly_withdrawal_amount_online_atm: 0,
        total_weekly_balance_amount_online_atm: 0,
        total_weekly_topup_amount_online_atm: 0,
        total_weekly_no_of_transactions_online_atm: 0,

        total_monthly_withdrawal_amount_online_atm: 0,
        total_monthly_balance_amount_online_atm: 0,
        total_monthly_topup_amount_online_atm: 0,
        total_monthly_no_of_transactions_online_atm: 0,
    }
};



// Limit level profile def
export interface LimitLevelProfileDef {
    limit_level_id: number,
    limit_level_profile_id: number,
    // the idea is to have data on which entity to be used for the limit values
}

export function createLimitLevelProfileDef(limit_level_profile_id: number): LimitLevelProfileDef {
    return {
        limit_level_id: next_id(),
        limit_level_profile_id: limit_level_profile_id
    }
}

// Limit level profile ID
export interface LimitLevelProfile {
    limit_level_profile_id: number,

    card_no?: string,
    account_no?: string,
    customer_no?: number,

    use_card_limits: boolean,
    use_customer_limits: boolean,
    use_account_limits: boolean,
    use_card_account_limits: boolean,
    use_account_customer_limits: boolean
}

export function createLimitLevelProfile( params: {
    use_limits_of: "CARD" | "CUSTOMER" | "ACCOUNT",
    card_no?: string,
    account_no?: string,
    customer_no?: number
    }
): LimitLevelProfile {
    const card_no = params.card_no ? params.card_no : "NA";
    const account_no = params.account_no ? params.account_no : "NA";
    const customer_no = params.customer_no ? params.customer_no : 0;
    return {
        limit_level_profile_id: next_id(),
        card_no: card_no,
        account_no: account_no,
        customer_no: customer_no,
        use_card_limits: params.use_limits_of == "CARD",
        use_customer_limits: params.use_limits_of == "CUSTOMER",
        use_account_limits: params.use_limits_of == "ACCOUNT",
        use_account_customer_limits: false,
        use_card_account_limits: false,
    }
}