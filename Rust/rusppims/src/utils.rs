use crate::store::CustomerInfo;

// find & get mutable CustomerInfo
pub fn find_n_get_mut_customer_info<'a>(
    customer_list: &'a mut Vec<CustomerInfo>,
    unique_id: &str,
) -> Option<&'a mut CustomerInfo> {
    for customer in customer_list {
        if customer.unique_id == unique_id {
            return Some(customer);
        }
    }
    None
}
