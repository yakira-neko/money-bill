use chrono::Datelike;
use details::Details;
use rusqlite::params;
use rusqlite::Connection;
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
pub mod account;
pub mod details;
use error::Error;

use crate::error;
pub mod budget;
pub mod init;
pub mod transaction;

/// Calculates the total income for the current month.
///
/// # Arguments
///
/// * `conn` - A connection to the database.
///
/// # Returns
///
/// * The total income as a `Decimal`.
pub fn get_month_incomed(conn: &Connection) -> Result<Decimal, Error> {
    let mut result = Decimal::zero();
    // Prepare and execute SQL statement to select relevant transactions for the current month.
    let mut stmt = conn
        .prepare("SELECT DETAIL.id,trans_id,account,currency,balance FROM DETAIL INNER JOIN TRANS ON trans_id=TRANS.id and strftime('%Y-%m', time) = strftime('%Y-%m', 'now')")?;
    // Iterate over the results to calculate total income.
    let iter = stmt.query_map(params![], |row| {
        Ok(Details {
            id: row.get(0)?,
            trans_id: row.get(1)?,
            account: row.get(2)?,
            currency: row.get(3)?,
            balance: Decimal::from_f32_retain(row.get::<usize, f32>(4)?).unwrap(),
        })
    })?;
    for i in iter {
        let details = i.unwrap();
        // If the transaction is related to income, add its balance to the total.
        if details.account.starts_with("income::") {
            result += details.balance;
        }
    }

    Ok(result)
}

/// Calculates the total expenses for the current month.
///
/// # Arguments
///
/// * `conn` - A connection to the database.
///
/// # Returns
///
/// * The total expenses as a `Decimal`.
pub fn get_month_expenses(conn: &Connection) -> Result<Decimal, Error> {
    let mut result = Decimal::zero();
    // Prepare and execute SQL statement to select relevant transactions for the current month.
    let mut stmt = conn
        .prepare("SELECT DETAIL.id,trans_id,account,currency,balance FROM DETAIL INNER JOIN TRANS ON trans_id=TRANS.id and strftime('%Y-%m', time) = strftime('%Y-%m', 'now')")?;
    // Iterate over the results to calculate total expenses.
    let iter = stmt.query_map(params![], |row| {
        Ok(Details {
            id: row.get(0)?,
            trans_id: row.get(1)?,
            account: row.get(2)?,
            currency: row.get(3)?,
            balance: Decimal::from_f32_retain(row.get::<usize, f32>(4)?).unwrap(),
        })
    })?;
    for i in iter {
        let details = i.unwrap();
        // If the transaction is related to expenses, add its balance to the total.
        if details.account.starts_with("expenses::") {
            result += details.balance;
        }
    }

    Ok(result)
}

/// 统计当前自然月内、某个支出账户（前缀）的总支出。
///
/// # 参数
/// * `conn` - 数据库连接。
/// * `account_prefix` - 账户前缀，例如 "expenses::food"。传入 "expenses" 即统计全部支出。
///
/// # 返回
/// * 该前缀下本月累计支出的 `Decimal`。支出在复式记账中通常为正数。
pub fn get_month_expenses_by_account(
    conn: &Connection,
    account_prefix: &str,
) -> Result<Decimal, Error> {
    let mut result = Decimal::zero();
    let mut stmt = conn.prepare(
        "SELECT account,balance FROM DETAIL INNER JOIN TRANS ON trans_id=TRANS.id and strftime('%Y-%m', time) = strftime('%Y-%m', 'now')",
    )?;
    let iter = stmt.query_map(params![], |row| {
        let account: String = row.get(0)?;
        let balance = Decimal::from_f32_retain(row.get::<usize, f32>(1)?).unwrap_or(Decimal::zero());
        Ok((account, balance))
    })?;
    for i in iter {
        let (account, balance) = i?;
        // 只统计支出账户，并按传入前缀精确过滤（避免 "expenses::food" 误匹配 "expenses::foodcourt"）。
        if account.starts_with("expenses") && account_matches_prefix(&account, account_prefix) {
            result += balance;
        }
    }

    Ok(result)
}

/// 判断某账户是否属于给定预算前缀。
///
/// - 前缀为空或恰为 "expenses"：匹配所有支出账户。
/// - 否则：账户需与前缀完全相等，或以「前缀 + ::」开头（即其子账户）。
fn account_matches_prefix(account: &str, prefix: &str) -> bool {
    if prefix.is_empty() || prefix == "expenses" {
        return true;
    }
    account == prefix || account.starts_with(&format!("{}::", prefix))
}

/// Recalculates account balances and updates them in the database.
///
/// # Arguments
///
/// * `conn` - A connection to the database.
///
/// # Returns
///
/// * An `Result<()>` indicating success or failure of the operation.
pub fn recalcute(conn: &Connection) -> Result<(), Error> {
    // Get all accounts from the database.
    let accounts = account::get_accounts(conn)?;
    for account in accounts {
        // Get details for each account and calculate the total balance.
        let details = details::get_details_by_account(conn, account.name.as_str())?;
        let mut balance = dec!(0.0);
        for i in details {
            balance += i.balance;
        }
        // Update the account's balance with the calculated value.
        let mut acc = account;
        acc.balance = balance;
        account::update_account(conn, &acc)?;
    }
    Ok(())
}

/// Calculates weekly expenses for the current week.
///
/// # Arguments
///
/// * `conn` - A connection to the database.
///
/// # Returns
///
/// * A vector of f32 representing daily expenses from Monday to Sunday.
pub fn get_weekly_expenses(conn: &Connection) -> Result<Vec<f32>, Error> {
    let mut result = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    // Prepare and execute SQL statement to select relevant transactions for the current week.
    let mut stmt = conn
        .prepare("SELECT DETAIL.id,trans_id,account,currency,balance,time FROM DETAIL INNER JOIN TRANS ON trans_id=TRANS.id and strftime('%Y %W', time) = strftime('%Y %W', 'now') and account like \"expenses%\" ORDER BY TIME ")?;
    // Iterate over the results to calculate weekly expenses.
    let iter = stmt.query_map(
        params![],
        |row: &rusqlite::Row<'_>| -> rusqlite::Result<(Details, String)> {
            Ok((
                Details {
                    id: row.get(0)?,
                    trans_id: row.get(1)?,
                    account: row.get(2)?,
                    currency: row.get(3)?,
                    balance: Decimal::from_f32_retain(row.get::<usize, f32>(4)?).unwrap(),
                },
                row.get(5)?,
            ))
        },
    )?;
    for i in iter {
        let s = i?;
        // Parse the date and add the balance to the corresponding day of the week.
        let date = chrono::NaiveDateTime::parse_from_str(s.1.as_str(), "%Y-%m-%d %H:%M").unwrap();
        result[date.weekday().num_days_from_monday() as usize] += s.0.balance.to_f32().unwrap();
    }

    Ok(result)
}

/// Calculates weekly income for the current week.
///
/// # Arguments
///
/// * `conn` - A connection to the database.
///
/// # Returns
///
/// * A vector of f32 representing daily income from Monday to Sunday.
pub fn get_weekly_income(conn: &Connection) -> Result<Vec<f32>, Error> {
    let mut result = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    // Prepare and execute SQL statement to select relevant transactions for the current week.
    let mut stmt = conn
        .prepare("SELECT DETAIL.id,trans_id,account,currency,balance,time FROM DETAIL INNER JOIN TRANS ON trans_id=TRANS.id and strftime('%Y %W', time) = strftime('%Y %W', 'now') and account like \"income%\" ORDER BY TIME ")?;
    // Iterate over the results to calculate weekly income.
    let iter = stmt.query_map(
        params![],
        |row: &rusqlite::Row<'_>| -> rusqlite::Result<(Details, String)> {
            Ok((
                Details {
                    id: row.get(0)?,
                    trans_id: row.get(1)?,
                    account: row.get(2)?,
                    currency: row.get(3)?,
                    balance: Decimal::from_f32_retain(row.get::<usize, f32>(4)?).unwrap(),
                },
                row.get(5)?,
            ))
        },
    )?;
    for i in iter {
        let s = i?;
        // Skip expenses transactions.
        if s.0.account.starts_with("expenses") {
            continue;
        }
        // Parse the date and add the balance to the corresponding day of the week.
        let date = chrono::NaiveDateTime::parse_from_str(s.1.as_str(), "%Y-%m-%d %H:%M").unwrap();
        result[date.weekday().num_days_from_monday() as usize] += s.0.balance.to_f32().unwrap();
    }

    Ok(result)
}
// fn verification(conn: &Connection) -> Result<(), Error> {
//     let trans = transaction::get_transactions(conn)?;
//     for tran in trans {
//         let details = details::get_details_by_trans(conn, tran.id.as_str())?;
//         let mut balance = dec!(0.0);
//         for i in details {
//             balance += i.balance;
//         }
//         if balance != Decimal::zero() {
//             return Err(Error::DetailSumNotZero(tran.id, balance.to_f32().unwrap()));
//         }
//     }
//     Ok(())
// }
#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::recalcute;
    use crate::database::{account, details, init, transaction};
    use error::Error;
    use rust_decimal_macros::dec;

    #[test]
    fn test_recalcute() -> Result<(), Error> {
        let conn = Connection::open_in_memory()?;
        init::init(&conn)?;
        account::add_account(&conn, "income::bar", "USD", "", "")?;
        account::add_account(&conn, "expenses::foo", "USD", "", "")?;

        let date = chrono::Utc::now();
        let id = transaction::add_transaction(&conn, date, "")?;
        details::add_details(
            &conn,
            id.as_str(),
            "income::bar",
            "USD",
            Decimal::from_f32_retain(-10.0).unwrap(),
        )?;
        details::add_details(
            &conn,
            id.as_str(),
            "expenses::foo",
            "USD",
            Decimal::from_f32_retain(10.0).unwrap(),
        )?;
        let id = transaction::add_transaction(&conn, date, "")?;
        details::add_details(
            &conn,
            id.as_str(),
            "income::bar",
            "USD",
            Decimal::from_f32_retain(-12.0).unwrap(),
        )?;
        details::add_details(
            &conn,
            id.as_str(),
            "expenses::foo",
            "USD",
            Decimal::from_f32_retain(12.0).unwrap(),
        )?;
        recalcute(&conn)?;

        let accounts = account::get_accounts(&conn)?;

        for account in accounts {
            if account.name == "expenses::foo" {
                let balance = dec!(22);
                assert_eq!(account.balance, balance);
            }
            if account.name == "income::bar" {
                let balance = dec!(-22);
                assert_eq!(account.balance, balance);
            }
        }
        Ok(())
    }

    #[test]
    fn test_get_month_expenses_by_account() -> Result<(), Error> {
        let conn = Connection::open_in_memory()?;
        init::init(&conn)?;
        account::add_account(&conn, "income::salary", "CNY", "", "")?;
        account::add_account(&conn, "expenses::food", "CNY", "", "")?;
        account::add_account(&conn, "expenses::food::lunch", "CNY", "", "")?;
        account::add_account(&conn, "expenses::transport", "CNY", "", "")?;

        let date = chrono::Utc::now();

        // 食物支出 30（food 10 + food::lunch 20）
        let id = transaction::add_transaction(&conn, date, "")?;
        details::add_details(&conn, &id, "income::salary", "CNY", dec!(-10.0))?;
        details::add_details(&conn, &id, "expenses::food", "CNY", dec!(10.0))?;
        let id = transaction::add_transaction(&conn, date, "")?;
        details::add_details(&conn, &id, "income::salary", "CNY", dec!(-20.0))?;
        details::add_details(&conn, &id, "expenses::food::lunch", "CNY", dec!(20.0))?;

        // 交通支出 5
        let id = transaction::add_transaction(&conn, date, "")?;
        details::add_details(&conn, &id, "income::salary", "CNY", dec!(-5.0))?;
        details::add_details(&conn, &id, "expenses::transport", "CNY", dec!(5.0))?;

        // 指定分类（含子分类）
        assert_eq!(dec!(30.0), get_month_expenses_by_account(&conn, "expenses::food")?);
        assert_eq!(dec!(5.0), get_month_expenses_by_account(&conn, "expenses::transport")?);
        // 总预算：所有支出 35
        assert_eq!(dec!(35.0), get_month_expenses_by_account(&conn, "expenses")?);
        assert_eq!(dec!(35.0), get_month_expenses_by_account(&conn, "")?);
        // 与 get_month_expenses 一致
        assert_eq!(get_month_expenses(&conn)?, get_month_expenses_by_account(&conn, "expenses")?);
        Ok(())
    }
}
