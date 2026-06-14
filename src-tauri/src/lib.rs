use std::{env, fs, path::PathBuf, sync::Mutex};

use chrono::DateTime;
use database::{
    account::{self},
    budget::{self},
    details::{add_details, get_details_by_trans},
    get_month_expenses, get_month_expenses_by_account, get_month_incomed, get_weekly_expenses,
    get_weekly_income, init, recalcute,
    transaction::{add_transaction, get_transactions},
};
use rusqlite::Connection;
use rust_decimal::prelude::*;
use tauri_struct::{AccountAmount, AccountIconName, BudgetProgress, WeeklyIncomeExpenses};
mod database;
mod error;
mod tauri_struct;
struct ConnectionWrapper {
    db: Mutex<Connection>,
}

fn database_path() -> Result<PathBuf, String> {
    let exe_path = env::current_exe().map_err(|e| e.to_string())?;
    let exe_dir = exe_path
        .parent()
        .ok_or_else(|| "Failed to resolve executable directory".to_string())?;
    Ok(exe_dir.join("db.db3"))
}

#[tauri::command]
/// 添加账单信息到数据库
///
/// 该函数接收一个数据库连接状态、账户金额列表、日期和额外信息，并将这些信息添加到数据库中。
/// 同时会更新相关账户的余额。
///
/// # 参数
/// - `conn`: 数据库连接状态，使用 `tauri::State` 管理。
/// - `account_amounts`: 账户金额列表，每个元素包含账户名称和金额。
/// - `date`: 交易日期，以时间戳形式表示。
/// - `extra`: 额外信息，如交易备注。
/// - `currency`: 货币类型，如 "USD"。
///
/// # 返回值
/// 如果操作成功，返回 `Ok(())`；如果出现错误，返回包含错误信息的 `Err(String)`。
async fn add_bills(
    conn: tauri::State<'_, ConnectionWrapper>,
    account_amounts: Vec<AccountAmount>,
    date: i64,
    extra: String,
    currency: String,
) -> Result<(), String> {
    let conn = conn.db.lock().unwrap();
    let utc_datetime: DateTime<chrono::Utc> = DateTime::from_timestamp(date, 0).unwrap();
    let trans_id = add_transaction(&conn, utc_datetime, &extra).unwrap();
    
    // 添加交易详情并更新账户余额
    for account_amount in account_amounts {
        let amount = Decimal::from_f32_retain(account_amount.amount).unwrap();
        let _ = add_details(&conn, &trans_id, &account_amount.account, &currency, amount);
        
        // 更新账户余额
        if let Err(e) = account::update_account_balance(&conn, &account_amount.account, amount) {
            eprintln!("Failed to update account balance for {}: {}", account_amount.account, e);
            return Err(format!("Failed to update account balance: {}", e));
        }
    }
    println!("{}", date);
    Ok(())
}
#[tauri::command]
/// 获取收入账户列表
///
/// 该函数从数据库中获取所有收入账户的信息，包括账户名称、图标和货币类型。
///
/// # 参数
/// - `conn`: 数据库连接状态，使用 `tauri::State` 管理。
///
/// # 返回值
/// 返回包含账户信息的 `AccountIconName` 结构体向量。
fn get_income_accounts(conn: tauri::State<ConnectionWrapper>) -> Vec<AccountIconName> {
    let conn = conn.db.lock().unwrap();
    let accounts = account::get_income_accounts(&conn).unwrap();
    let accounts = accounts
        .iter()
        .map(|it| AccountIconName {
            name: it.name.clone(),
            icon: it.icon.clone().unwrap_or("".to_string()),
            currency: it.currency.clone(),
            balance: it.balance.to_f32().unwrap_or(0.0),
        })
        .collect();
    return accounts;
}

#[tauri::command]
/// 获取支出账户列表
///
/// 该函数从数据库中获取所有支出账户的信息，包括账户名称、图标和货币类型。
///
/// # 参数
/// - `conn`: 数据库连接状态，使用 `tauri::State` 管理。
///
/// # 返回值
/// 返回包含账户信息的 `AccountIconName` 结构体向量。
fn get_expenses_accounts(conn: tauri::State<ConnectionWrapper>) -> Vec<AccountIconName> {
    let conn = conn.db.lock().unwrap();
    let accounts = account::get_expenses_accounts(&conn).unwrap();
    let accounts = accounts
        .iter()
        .map(|it| AccountIconName {
            name: it.name.clone(),
            icon: it.icon.clone().unwrap_or("".to_string()),
            currency: it.currency.clone(),
            balance: it.balance.to_f32().unwrap_or(0.0),
        })
        .collect();
    return accounts;
}

#[tauri::command]
/// 获取资产账户列表
///
/// 该函数从数据库中获取所有资产账户的信息，包括账户名称、图标和货币类型。
///
/// # 参数
/// - `conn`: 数据库连接状态，使用 `tauri::State` 管理。
///
/// # 返回值
/// 返回包含账户信息的 `AccountIconName` 结构体向量。
fn get_assets_accounts(conn: tauri::State<ConnectionWrapper>) -> Vec<AccountIconName> {
    let conn = conn.db.lock().unwrap();
    let accounts = account::get_assets_accounts(&conn).unwrap();
    let accounts = accounts
        .iter()
        .map(|it| AccountIconName {
            name: it.name.clone(),
            icon: it.icon.clone().unwrap_or("".to_string()),
            currency: it.currency.clone(),
            balance: it.balance.to_f32().unwrap_or(0.0),
        })
        .collect();
    return accounts;
}

#[tauri::command]
/// 获取负债账户列表
///
/// 该函数从数据库中获取所有负债账户的信息，包括账户名称、图标和货币类型。
///
/// # 参数
/// - `conn`: 数据库连接状态，使用 `tauri::State` 管理。
///
/// # 返回值
/// 返回包含账户信息的 `AccountIconName` 结构体向量。
fn get_liabilities_accounts(conn: tauri::State<ConnectionWrapper>) -> Vec<AccountIconName> {
    let conn = conn.db.lock().unwrap();
    let accounts = account::get_liabilities_accounts(&conn).unwrap();
    let accounts = accounts
        .iter()
        .map(|it| AccountIconName {
            name: it.name.clone(),
            icon: it.icon.clone().unwrap_or("".to_string()),
            currency: it.currency.clone(),
            balance: it.balance.to_f32().unwrap_or(0.0),
        })
        .collect();
    return accounts;
}

#[tauri::command]
/// 添加账户
fn add_account(
    conn: tauri::State<ConnectionWrapper>,
    name: String,
    currency: String,
    icon: Option<String>,
    extra: Option<String>,
) -> Result<(), String> {
    let conn = conn.db.lock().unwrap();
    account::add_account(
        &conn,
        &name,
        &currency,
        icon.as_deref().unwrap_or(""),
        extra.as_deref().unwrap_or(""),
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
/// 更新账户
fn update_account(
    conn: tauri::State<ConnectionWrapper>,
    name: String,
    currency: String,
    icon: Option<String>,
    extra: Option<String>,
) -> Result<(), String> {
    let conn = conn.db.lock().unwrap();
    // Assuming backend update_account takes an Account struct or similar.
    // Based on account.rs: pub fn update_account(conn: &Connection, account: &Account)
    // We need to fetch the existing balance or allow updating it?
    // The current update_account in account.rs updates everything including balance.
    // However, usually we don't update balance manually via this API unless it's a correction.
    // For now let's fetch the account to get the balance, then update.
    
    let mut accounts = account::read_account(&conn, &name).map_err(|e| e.to_string())?;
    if let Some(mut account) = accounts.pop() {
        account.currency = currency;
        account.icon = icon;
        account.extra = extra;
        account::update_account(&conn, &account).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Account not found".to_string())
    }
}

#[tauri::command]
/// 删除账户
fn delete_account(conn: tauri::State<ConnectionWrapper>, name: String) -> Result<(), String> {
    let conn = conn.db.lock().unwrap();
    account::del_account(&conn, &name).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_income_expenses(conn: tauri::State<ConnectionWrapper>) -> Vec<f32> {
    let conn = conn.db.lock().unwrap();
    // let bar_id = account::add_account(&conn, "income::a", "USD").unwrap();
    // let foo_id = account::add_account(&conn, "expenses::a", "USD").unwrap();

    // let date = chrono::Utc::now();
    // let id = transaction::add_transaction(&conn, date, "").unwrap();
    // details::add_details(
    //     &conn,
    //     id.as_str(),
    //     "income::a",
    //     "USD",
    //     Decimal::from_f32_retain(12.0).unwrap(),
    // );
    // details::add_details(
    //     &conn,
    //     id.as_str(),
    //     "expenses::a",
    //     "USD",
    //     Decimal::from_f32_retain(-12.0).unwrap(),
    // );
    // recalcute(&conn).unwrap();
    let result = vec![
        get_month_incomed(&conn).unwrap().to_f32().unwrap(),
        get_month_expenses(&conn).unwrap().to_f32().unwrap(),
    ];
    // print!("{}", result);
    result
}
#[tauri::command]
/// 获取每周的收入和支出数据
///
/// 该函数从数据库中查询当前周的收支情况，并返回一个包含收入和支出总额的结构体
///
/// # 参数
/// - `conn`: 数据库连接状态，使用 `tauri::State` 管理
///
/// # 返回值
/// 返回 `WeeklyIncomeExpenses` 结构体，包含两个字段：
/// - `income`: 本周总收入
/// - `expenses`: 本周总支出
fn get_weekly_income_expenses(conn: tauri::State<ConnectionWrapper>) -> WeeklyIncomeExpenses {
    // 获取数据库连接
    let conn = conn.db.lock().unwrap();

    // 查询本周收入总额
    let incomes = get_weekly_income(&conn).unwrap();

    // 查询本周支出总额
    let expenses = get_weekly_expenses(&conn).unwrap();

    // 返回包含收支数据的结构体
    return WeeklyIncomeExpenses {
        income: incomes,
        expenses,
    };
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(serde::Serialize)]
struct TransactionHistoryItem {
    id: String,
    date: String,
    extra: String,
    details: Vec<TransactionDetail>,
}

#[derive(serde::Serialize)]
struct TransactionDetail {
    id: String,
    account: String,
    balance: f32,
}

#[tauri::command]
fn get_transaction_history(
    conn: tauri::State<ConnectionWrapper>,
) -> Result<Vec<TransactionHistoryItem>, String> {
    let conn = conn.db.lock().unwrap();

    // Get all transactions
    let transactions = get_transactions(&conn).map_err(|e| e.to_string())?;

    // For each transaction, get its details
    let mut history = Vec::new();
    for transaction in transactions {
        let details = get_details_by_trans(&conn, &transaction.id).map_err(|e| e.to_string())?;

        let details_serializable: Vec<TransactionDetail> = details
            .into_iter()
            .map(|d| TransactionDetail {
                id: d.id,
                account: d.account,
                balance: d.balance.to_f32().unwrap_or(0.0),
            })
            .collect();

        history.push(TransactionHistoryItem {
            id: transaction.id,
            date: transaction.date,
            extra: transaction.extra,
            details: details_serializable,
        });
    }

    Ok(history)
}

#[tauri::command]
/// 新增一条预算
///
/// # 参数
/// - `account`: 关联的支出账户/前缀，空字符串表示总预算（统计所有支出）
/// - `amount`: 预算上限金额
/// - `period`: 预算周期，目前为 "monthly"
/// - `currency`: 币种
fn add_budget(
    conn: tauri::State<ConnectionWrapper>,
    account: String,
    amount: f32,
    period: Option<String>,
    currency: Option<String>,
) -> Result<String, String> {
    let conn = conn.db.lock().unwrap();
    let amount = Decimal::from_f32_retain(amount).unwrap_or(Decimal::ZERO);
    budget::add_budget(
        &conn,
        &account,
        amount,
        period.as_deref().unwrap_or("monthly"),
        currency.as_deref().unwrap_or("CNY"),
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
/// 更新一条预算
fn update_budget(
    conn: tauri::State<ConnectionWrapper>,
    id: String,
    account: String,
    amount: f32,
    period: Option<String>,
    currency: Option<String>,
) -> Result<(), String> {
    let conn = conn.db.lock().unwrap();
    let amount = Decimal::from_f32_retain(amount).unwrap_or(Decimal::ZERO);
    budget::update_budget(
        &conn,
        &id,
        &account,
        amount,
        period.as_deref().unwrap_or("monthly"),
        currency.as_deref().unwrap_or("CNY"),
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
/// 删除一条预算
fn delete_budget(conn: tauri::State<ConnectionWrapper>, id: String) -> Result<(), String> {
    let conn = conn.db.lock().unwrap();
    budget::del_budget(&conn, &id).map_err(|e| e.to_string())
}

#[tauri::command]
/// 获取所有预算及其本月执行进度
///
/// 对每条预算，统计其关联账户在当前自然月内的累计支出，连同预算上限一并返回，
/// 供前端展示进度条与超支提醒。
fn get_budgets(conn: tauri::State<ConnectionWrapper>) -> Result<Vec<BudgetProgress>, String> {
    let conn = conn.db.lock().unwrap();
    let budgets = budget::get_budgets(&conn).map_err(|e| e.to_string())?;
    let mut result = Vec::new();
    for b in budgets {
        let spent = get_month_expenses_by_account(&conn, &b.account)
            .map_err(|e| e.to_string())?
            .to_f32()
            .unwrap_or(0.0);
        result.push(BudgetProgress {
            id: b.id,
            account: b.account,
            amount: b.amount.to_f32().unwrap_or(0.0),
            spent,
            period: b.period,
            currency: b.currency,
        });
    }
    Ok(result)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db_path = database_path().expect("failed to resolve database path");
    if let Some(parent) = db_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let conn = Connection::open(db_path).unwrap();
    let _ = init::init(&conn);
    // 为老版本（已存在 db.db3）的用户补建 BUDGET 表，避免因 ACCOUNT 表已存在导致 init 提前返回而漏建。
    let _ = init::init_budget(&conn);
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .manage(ConnectionWrapper {
            db: Mutex::new(conn),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_income_expenses,
            get_weekly_income_expenses,
            get_expenses_accounts,
            get_income_accounts,
            add_bills,
            get_transaction_history,
            get_assets_accounts,
            get_liabilities_accounts,
            add_account,
            update_account,
            delete_account,
            add_budget,
            update_budget,
            delete_budget,
            get_budgets
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
